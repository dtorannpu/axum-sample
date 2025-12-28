use async_trait::async_trait;
use derive_new::new;
pub use kernel::repository::sample::SampleRepository;
use shaku::Component;

#[derive(new, Component)]
#[shaku(interface = SampleRepository)]
pub struct SampleRepositoryImpl {}

#[async_trait]
impl SampleRepository for SampleRepositoryImpl {
    async fn get_sample(&self) -> String {
        return "Sample".to_string();
    }
}
