use crate::model::sample::SampleRequest;
use axum::http::StatusCode;
use axum::Json;
use axum_valid::Garde;

pub async fn sample() -> &'static str {
    "Hello, Sample!"
}

pub async fn register(Garde(Json(_)): Garde<Json<SampleRequest>>) -> StatusCode {
    StatusCode::CREATED
}
