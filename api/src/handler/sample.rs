use crate::model::sample::{SampleList, SampleRequest, SampleResponse};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum_valid::Garde;
use registry::AppRegistry;

pub async fn sample(State(_registry): State<AppRegistry>) -> Json<SampleList> {
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

pub async fn register(
    State(_registry): State<AppRegistry>,
    Garde(Json(_)): Garde<Json<SampleRequest>>,
) -> StatusCode {
    StatusCode::CREATED
}
