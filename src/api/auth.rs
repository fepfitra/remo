use std::sync::Arc;

use anyhow::Context;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;
use serde_json::json;

use crate::{common::error::AppError, store::Store};

#[derive(Deserialize)]
struct UserSign {
    username: String,
    password: String,
}

async fn handle_user_sign_up(
    State(store): State<Arc<Store>>,
    Json(payload): Json<UserSign>,
) -> Result<impl IntoResponse, AppError> {
    let user = store
        .create_new_user(&payload.username, &payload.password, "", "")
        .await
        .context("Failed to create user in database")?;

    let resp = (StatusCode::CREATED, Json(json!(user)));
    Ok(resp)
}

async fn handle_user_sign_in(
    State(store): State<Arc<Store>>,
    jar: CookieJar,
    Json(payload): Json<UserSign>,
) -> Result<(CookieJar, impl IntoResponse), AppError> {
    let user = store
        .get_user_by_username_and_password(&payload.username, &payload.password)
        .await
        .map_err(|e| AppError::from(e).with_status(StatusCode::UNAUTHORIZED))?;

    let cookie = Cookie::build(("user_id", user.id.clone()))
        .path("/")
        .max_age(time::Duration::days(30));
    let jar = jar.add(cookie);
    let resp = (StatusCode::OK, Json(json!(user)));
    Ok((jar, resp))
}

async fn handle_user_sign_out(jar: CookieJar) -> (CookieJar, impl IntoResponse) {
    let jar = jar.remove(Cookie::from("user_id"));
    (
        jar,
        (
            StatusCode::OK,
            Json(json!({"message": "Signed out successfully"})),
        ),
    )
}

pub async fn register_auth_routes() -> Router<Arc<Store>> {
    Router::new()
        .route("/signup", post(handle_user_sign_up))
        .route("/signin", post(handle_user_sign_in))
        .route("/signout", post(handle_user_sign_out))
}
