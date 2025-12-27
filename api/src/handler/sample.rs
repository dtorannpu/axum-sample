use crate::model::sample::SampleRequest;
use axum::http::StatusCode;
use axum::Json;
use garde::Validate;
use shared::error::AppResult;

pub async fn sample() -> &'static str {
    "Hello, Sample!"
}

pub async fn register(
    Json(req): Json<SampleRequest>,
) -> AppResult<StatusCode> {
    req.validate()?;
    Ok(StatusCode::CREATED)
}
