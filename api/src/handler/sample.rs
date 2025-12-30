use crate::model::sample::{SampleList, SampleRequest};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum_valid::Garde;
use kernel::repository::sample::SampleRepository;
use registry::AppState;
use shaku::HasComponent;
use shared::error::AppResult;

pub async fn sample(State(registry): State<AppState>) -> AppResult<Json<SampleList>> {
    let sample_repository: &dyn SampleRepository = registry.module.resolve_ref();
    sample_repository
        .find_all()
        .await
        .map(SampleList::from)
        .map(Json)
}

pub async fn register(
    State(_registry): State<AppState>,
    Garde(Json(_)): Garde<Json<SampleRequest>>,
) -> StatusCode {
    StatusCode::CREATED
}
