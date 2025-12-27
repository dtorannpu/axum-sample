use crate::model::sample::{SampleList, SampleRequest, SampleResponse};
use axum::Json;
use axum::http::StatusCode;
use axum_valid::Garde;

pub async fn sample() -> Json<SampleList> {
    Json(SampleList {
        samples: vec![
            SampleResponse {
                name: "Sample".to_string(),
                age: 20,
            },
            SampleResponse {
                name: "Sample2".to_string(),
                age: 30,
            },
        ],
    })
}

pub async fn register(Garde(Json(_)): Garde<Json<SampleRequest>>) -> StatusCode {
    StatusCode::CREATED
}
