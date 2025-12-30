use crate::model::sample::Sample;
use async_trait::async_trait;
use shaku::Interface;
use shared::error::AppResult;

#[async_trait]
pub trait SampleRepository: Interface {
    async fn find_all(&self) -> AppResult<Vec<Sample>>;
}
