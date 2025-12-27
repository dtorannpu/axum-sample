use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidateError(#[from] garde::Report),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::ValidateError(_) => StatusCode::BAD_REQUEST,
        };

        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
