use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::errors::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(_) => {
                let msg = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::JsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
