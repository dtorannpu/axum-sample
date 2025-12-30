use crate::model::sample::Sample;
use crate::model::sample::event::CreateSample;
use async_trait::async_trait;
use shaku::Interface;
use shared::error::AppResult;

#[async_trait]
pub trait SampleRepository: Interface {
    async fn find_all(&self) -> AppResult<Vec<Sample>>;
    async fn create(&self, event: CreateSample) -> AppResult<Sample>;
}
