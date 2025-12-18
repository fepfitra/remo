use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub struct AppError {
    pub inner: anyhow::Error,
    pub status: StatusCode,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let msg = self.inner.to_string();

        // Log 5xx errors as errors, 4xx as warnings
        // if self.status.is_server_error() {
        //     tracing::error!("Server Error: {}", msg);
        // } else {
        //     tracing::warn!("Client Error: {}", msg);
        // }
        //
        (self.status, Json(json!({ "error": msg }))).into_response()
    }
}

impl AppError {
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            inner: err.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
