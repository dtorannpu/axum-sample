use crate::model::sample::{SampleList, SampleRequest, SampleResponse};
use adapter::repository::sample::SampleRepository;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum_valid::Garde;
use registry::AppState;
use shaku::HasComponent;

pub async fn sample(State(registry): State<AppState>) -> Json<SampleList> {
    let sample_repository: &dyn SampleRepository = registry.module.resolve_ref();
    let _a = sample_repository.get_sample().await;
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
    State(_registry): State<AppState>,
    Garde(Json(_)): Garde<Json<SampleRequest>>,
) -> StatusCode {
    StatusCode::CREATED
}
