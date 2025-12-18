use anyhow::Context;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware::from_extractor_with_state;
use axum::routing::{get, patch};
use axum::{Router, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::common::error::AppError;
use crate::{api::middleware::AuthChecker, store::Store};

async fn handle_get_my_user_info(AuthChecker(user): AuthChecker) -> impl IntoResponse {
    (StatusCode::OK, Json(json!(user)))
}

#[derive(Deserialize)]
struct UpdateUser {
    username: Option<String>,
    password: Option<String>,
    github_name: Option<String>,
    wx_open_id: Option<String>,
}

async fn handle_update_my_user_info(
    AuthChecker(user): AuthChecker,
    State(store): State<Arc<Store>>,
    Json(payload): Json<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
    let updated_user = store
        .update_user(
            user.id.as_str(),
            payload.username.as_deref(),
            payload.password.as_deref(),
            payload.github_name.as_deref(),
            payload.wx_open_id.as_deref(),
        )
        .await
        .context("Failed to update user information")?;
    let resp = (StatusCode::OK, Json(json!(updated_user)));
    Ok(resp)
}

pub async fn register_user_routes(store: Arc<Store>) -> Router<Arc<Store>> {
    Router::new()
        .route("/me", get(handle_get_my_user_info))
        .route("/me", patch(handle_update_my_user_info))
        .route_layer(from_extractor_with_state::<AuthChecker, Arc<Store>>(store))
}
