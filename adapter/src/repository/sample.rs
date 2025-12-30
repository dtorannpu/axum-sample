use crate::database::DbPool;
use async_trait::async_trait;
use derive_new::new;
pub use kernel::repository::sample::SampleRepository;
use shaku::Component;
use shared::error::{AppError, AppResult};
use std::sync::Arc;

#[derive(new, Component)]
#[shaku(interface = SampleRepository)]
pub struct SampleRepositoryImpl {
    #[shaku(inject)]
    pool: Arc<dyn DbPool>,
}

#[async_trait]
impl SampleRepository for SampleRepositoryImpl {
    async fn find_all(&self) -> AppResult<String> {
        sqlx::query("SELECT * FROM sample")
            .execute(self.pool.get().inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?;
        Ok("Sample".to_string())
    }
}
