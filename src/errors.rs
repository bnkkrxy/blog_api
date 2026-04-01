use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InvalidData(String), // 400
    NotFound(String), // 404
    InternalServer(String), // 500
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        use AppError::*;
        let (status, error_message) = match self {
            InvalidData(err_msg) => (StatusCode::BAD_REQUEST, err_msg),
            NotFound(err_msg) => (StatusCode::NOT_FOUND, err_msg),
            InternalServer(err_msg) => (StatusCode::INTERNAL_SERVER_ERROR, err_msg),
        };
        let error = Json(json!({"error": error_message}));
        (status, error).into_response()
    }
}