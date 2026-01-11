use crate::model::id::SampleId;
use crate::model::sample::Sample;
use crate::model::sample::event::{CreateSample, DeleteSample, UpdateSample};
use async_trait::async_trait;
use shaku::Interface;
use shared::error::AppResult;

#[async_trait]
pub trait SampleRepository: Interface {
    async fn create(&self, event: CreateSample) -> AppResult<Sample>;
    async fn find_all(&self) -> AppResult<Vec<Sample>>;
    async fn find_by_id(&self, id: SampleId) -> AppResult<Option<Sample>>;
    async fn update(&self, event: UpdateSample) -> AppResult<()>;
    async fn delete(&self, event: DeleteSample) -> AppResult<()>;
}
