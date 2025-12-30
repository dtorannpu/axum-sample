use crate::database::DbPool;
use crate::database::model::sample::SampleRow;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::sample::Sample;
use kernel::repository::sample::SampleRepository;
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
    async fn find_all(&self) -> AppResult<Vec<Sample>> {
        let result = sqlx::query_as!(SampleRow, "SELECT id, name, age FROM sample")
            .fetch_all(self.pool.get().inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?
            .into_iter()
            .filter_map(|row| Sample::try_from(row).ok())
            .collect();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{ConnectionPool, SqlxDbPool};
    use crate::repository::sample::SampleRepositoryImpl;
    use kernel::repository::sample::SampleRepository;
    use std::sync::Arc;

    #[sqlx::test]
    async fn test_find_all(pool: sqlx::PgPool) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO sample(name, age) VALUES ('test1', 10), ('test2', 20), ('test3', 30);"#
        )
        .execute(&pool)
        .await?;

        let repo = SampleRepositoryImpl::new(Arc::new(SqlxDbPool::new(Arc::new(
            ConnectionPool::new(pool.clone()),
        ))));
        let mut samples = repo.find_all().await?;
        samples.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(samples.len(), 3);
        assert_eq!(samples[0].name, "test1");
        assert_eq!(samples[0].age, 10);
        assert_eq!(samples[1].name, "test2");
        assert_eq!(samples[1].age, 20);
        assert_eq!(samples[2].name, "test3");
        assert_eq!(samples[2].age, 30);

        Ok(())
    }
}
