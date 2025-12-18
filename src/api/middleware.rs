use std::sync::Arc;

use axum::Json;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use serde_json::Value;

use crate::store::Store;
use crate::store::user::User;

pub struct AuthChecker(pub User);

impl<S> FromRequestParts<S> for AuthChecker
where
    Arc<Store>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = Arc::<Store>::from_ref(state);
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Failed to extract cookies"})),
                )
            })?;

        if let Some(user_id) = jar.get("user_id") {
            if let Ok(user) = store.get_user_by_id(user_id.value()).await {
                return Ok(AuthChecker(user));
            }
        }
        Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"})),
        ))
    }
}

