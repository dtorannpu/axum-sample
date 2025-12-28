use async_trait::async_trait;
use shaku::Interface;

#[async_trait]
pub trait SampleRepository: Interface {
    async fn get_sample(&self) -> String;
}
