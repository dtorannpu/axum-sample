use crate::database::DbPool;
use crate::database::model::sample::SampleRow;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::sample::Sample;
use kernel::model::sample::event::CreateSample;
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
        let result = sqlx::query_as!(SampleRow, "SELECT id, name, email, age FROM sample")
            .fetch_all(self.pool.get().inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?
            .into_iter()
            .filter_map(|row| Sample::try_from(row).ok())
            .collect();
        Ok(result)
    }

    async fn create(&self, event: CreateSample) -> AppResult<Sample> {
        let sample = sqlx::query_as!(
            SampleRow,
            r#"INSERT INTO sample(name, email, age) VALUES ($1, $2, $3) RETURNING id, name, email, age"#,
            event.name,
            event.email,
            event.age
        )
        .map(|row| Sample::try_from(row).ok())
        .fetch_one(self.pool.get().inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .ok_or(AppError::NoRowsAffectedError("Failed to insert row".into()))?;

        Ok(sample)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{ConnectionPool, SqlxDbPool};
    use crate::repository::sample::SampleRepositoryImpl;
    use kernel::model::sample::event::CreateSample;
    use kernel::repository::sample::SampleRepository;
    use shared::error::AppError;
    use std::sync::Arc;

    #[sqlx::test]
    async fn test_find_all(pool: sqlx::PgPool) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO sample(name, email, age)
                VALUES
                    ('test1', 'test1@example.com', 0),
                    ('test2', 'test2@example.com', 20),
                    ('test3', 'test3@example.com', 100);
            "#
        )
        .execute(&pool)
        .await?;

        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));
        let mut samples = repo.find_all().await?;
        samples.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(samples.len(), 3);
        assert_eq!(samples[0].name, "test1");
        assert_eq!(samples[0].email, "test1@example.com");
        assert_eq!(samples[0].age, 0);
        assert_eq!(samples[1].name, "test2");
        assert_eq!(samples[1].email, "test2@example.com");
        assert_eq!(samples[1].age, 20);
        assert_eq!(samples[2].name, "test3");
        assert_eq!(samples[2].email, "test3@example.com");
        assert_eq!(samples[2].age, 100);

        Ok(())
    }

    #[sqlx::test]
    async fn test_create_ok(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let res = repo
            .create(CreateSample {
                name: "test".into(),
                email: "test@example.com".into(),
                age: 10,
            })
            .await?;

        assert_eq!(res.name, "test");
        assert_eq!(res.email, "test@example.com");
        assert_eq!(res.age, 10);

        Ok(())
    }

    #[sqlx::test]
    async fn test_create_ng(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let res1 = repo
            .create(CreateSample {
                name: "test".into(),
                email: "test@example.com".into(),
                age: 10,
            })
            .await?;

        assert_eq!(res1.name, "test");
        assert_eq!(res1.email, "test@example.com");
        assert_eq!(res1.age, 10);

        let res2 = repo
            .create(CreateSample {
                name: "test".into(),
                email: "test@example.com".into(),
                age: 10,
            })
            .await;

        assert!(res2.is_err());
        match res2 {
            Err(AppError::SpecificOperationError(e)) => assert_eq!(
                e.to_string(),
                "error returned from database: duplicate key value violates unique constraint \"sample_email_key\""
            ),
            _ => panic!("Unexpected error"),
        }
        Ok(())
    }

    #[sqlx::test]
    async fn test_create_ng_age_minus(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(SqlxDbPool::new(Arc::new(
            ConnectionPool::new(pool.clone()),
        ))));

        let res = repo
            .create(CreateSample {
                name: "test".into(),
                email: "test@example.com".into(),
                age: -1,
            })
            .await;

        assert!(res.is_err());
        match res {
            Err(AppError::SpecificOperationError(e)) => assert_eq!(
                e.to_string(),
                "error returned from database: new row for relation \"sample\" violates check constraint \"sample_age_check\""
            ),
            _ => panic!("Unexpected error"),
        }
        Ok(())
    }

    #[sqlx::test]
    async fn test_create_ng_age_over(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(SqlxDbPool::new(Arc::new(
            ConnectionPool::new(pool.clone()),
        ))));

        let res = repo
            .create(CreateSample {
                name: "test".into(),
                email: "test@example.com".into(),
                age: 101,
            })
            .await;

        assert!(res.is_err());
        match res {
            Err(AppError::SpecificOperationError(e)) => assert_eq!(
                e.to_string(),
                "error returned from database: new row for relation \"sample\" violates check constraint \"sample_age_check\""
            ),
            _ => panic!("Unexpected error"),
        }
        Ok(())
    }
}
