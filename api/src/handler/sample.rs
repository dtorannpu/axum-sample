use crate::model::sample::{
    CreateSampleRequest, SampleList, SampleResponse, UpdateSampleRequest,
    UpdateSampleRequestWithIds,
};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum_valid::Garde;
use kernel::model::id::SampleId;
use kernel::repository::sample::SampleRepository;
use registry::AppState;
use shaku::HasComponent;
use shared::error::{AppError, AppResult};

pub async fn register(
    State(registry): State<AppState>,
    Garde(Json(req)): Garde<Json<CreateSampleRequest>>,
) -> AppResult<StatusCode> {
    let sample_repository: &dyn SampleRepository = registry.module.resolve_ref();
    sample_repository
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_sample_list(State(registry): State<AppState>) -> AppResult<Json<SampleList>> {
    let sample_repository: &dyn SampleRepository = registry.module.resolve_ref();
    sample_repository
        .find_all()
        .await
        .map(SampleList::from)
        .map(Json)
}
pub async fn show_sample(
    State(registry): State<AppState>,
    Path(id): Path<SampleId>,
) -> AppResult<Json<SampleResponse>> {
    let sample_repository: &dyn SampleRepository = registry.module.resolve_ref();
    sample_repository
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::EntityNotFound(format!("SampleId: {} is not found", id.get())))
        .map(SampleResponse::from)
        .map(Json)
}

pub async fn update_sample(
    State(registry): State<AppState>,
    Path(id): Path<SampleId>,
    Garde(Json(req)): Garde<Json<UpdateSampleRequest>>,
) -> AppResult<StatusCode> {
    let update_sample = UpdateSampleRequestWithIds::new(id, req);
    let sample_repository: &dyn SampleRepository = registry.module.resolve_ref();
    sample_repository.update(update_sample.into()).await?;
    Ok(StatusCode::NO_CONTENT)
}
