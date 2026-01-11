use crate::database::DbPool;
use crate::database::model::sample::SampleRow;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::id::SampleId;
use kernel::model::sample::Sample;
use kernel::model::sample::event::{CreateSample, DeleteSample, UpdateSample};
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
    async fn create(&self, event: CreateSample) -> AppResult<Sample> {
        let sample = sqlx::query_as!(
            SampleRow,
            r#"INSERT INTO sample(name, email, age) VALUES ($1, $2, $3) RETURNING id, name, email, age"#,
            event.name,
            event.email,
            event.age
        )
            .fetch_one(self.pool.get().inner_ref())
            .await
            .map_err(AppError::SpecificOperationError)?
            .into();
        Ok(sample)
    }

    async fn find_all(&self) -> AppResult<Vec<Sample>> {
        Ok(
            sqlx::query_as!(SampleRow, "SELECT id, name, email, age FROM sample")
                .fetch_all(self.pool.get().inner_ref())
                .await
                .map_err(AppError::SpecificOperationError)?
                .into_iter()
                .map(Sample::from)
                .collect(),
        )
    }

    async fn find_by_id(&self, id: SampleId) -> AppResult<Option<Sample>> {
        sqlx::query_as!(
            SampleRow,
            "SELECT id, name, email, age FROM sample WHERE id = $1",
            id.get(),
        )
        .fetch_optional(self.pool.get().inner_ref())
        .await
        .map(|opt| opt.map(Sample::from))
        .map_err(AppError::SpecificOperationError)
    }

    async fn update(&self, event: UpdateSample) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                UPDATE sample
                SET
                    name = $1,
                    email = $2,
                    age = $3
                WHERE id = $4
            "#,
            event.name,
            event.email,
            event.age,
            event.id.get(),
        )
        .execute(self.pool.get().inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("specified book not found".into()));
        }

        Ok(())
    }

    async fn delete(&self, event: DeleteSample) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                DELETE FROM sample
                WHERE id = $1
            "#,
            event.id.get()
        )
        .execute(self.pool.get().inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("specified book not found".into()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::database::ConnectionPool;
    use crate::database::model::sample::SampleRow;
    use crate::repository::sample::SampleRepositoryImpl;
    use kernel::model::id::SampleId;
    use kernel::model::sample::event::{CreateSample, DeleteSample, UpdateSample};
    use kernel::repository::sample::SampleRepository;
    use shared::error::AppError;
    use std::sync::Arc;

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
    async fn test_create_ng_not_email_unique(pool: sqlx::PgPool) -> anyhow::Result<()> {
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
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

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
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

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
    async fn test_find_by_id_ok(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let insert_id: i64 = sqlx::query_scalar!(
            r#"
                INSERT INTO sample(name, email, age)
                VALUES ('test', 'test@example.com', 25)
                RETURNING id
                ;
            "#
        )
        .fetch_one(&pool)
        .await?;
        let id = SampleId::new(insert_id);

        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let res = repo.find_by_id(id).await?;

        assert!(res.is_some());
        let sample = res.unwrap();
        assert_eq!(sample.id, id);
        assert_eq!(sample.name, "test");
        assert_eq!(sample.email, "test@example.com");
        assert_eq!(sample.age, 25);

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_by_id_none(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let res = repo.find_by_id(SampleId::new(-1)).await?;

        assert!(res.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_ok(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let id = SampleId::new(
            sqlx::query_scalar!(
                r#"INSERT INTO sample(name, email, age) VALUES ($1, $2, $3) RETURNING id"#,
                "test",
                "test@example.com",
                10
            )
            .fetch_one(&pool)
            .await?,
        );

        repo.update(UpdateSample {
            id,
            name: "updated".into(),
            email: "updated@example.com".into(),
            age: 20,
        })
        .await?;

        let updated = sqlx::query_as!(
            SampleRow,
            "SELECT id, name, email, age FROM sample WHERE id = $1",
            id.get()
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(updated.name, "updated");
        assert_eq!(updated.email, "updated@example.com");
        assert_eq!(updated.age, 20);

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_ng_not_found(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let res = repo
            .update(UpdateSample {
                id: SampleId::new(-1),
                name: "test".into(),
                email: "test@example.com".into(),
                age: 10,
            })
            .await;

        assert!(res.is_err());
        match res {
            Err(AppError::EntityNotFound(_)) => {}
            _ => panic!("Unexpected error"),
        }

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_ng_not_email_unique(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let id1 = SampleId::new(
            sqlx::query_scalar!(
                r#"INSERT INTO sample(name, email, age) VALUES ($1, $2, $3) RETURNING id"#,
                "test1",
                "test1@example.com",
                10
            )
            .fetch_one(&pool)
            .await?,
        );

        sqlx::query!(
            r#"INSERT INTO sample(name, email, age) VALUES ($1, $2, $3)"#,
            "test2",
            "test2@example.com",
            20
        )
        .execute(&pool)
        .await?;

        let res = repo
            .update(UpdateSample {
                id: id1,
                name: "test1".into(),
                email: "test2@example.com".into(),
                age: 10,
            })
            .await;

        assert!(res.is_err());
        match res {
            Err(AppError::SpecificOperationError(e)) => assert_eq!(
                e.to_string(),
                "error returned from database: duplicate key value violates unique constraint \"sample_email_key\""
            ),
            _ => panic!("Unexpected error"),
        }

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_ok(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let id = SampleId::new(
            sqlx::query_scalar!(
                r#"INSERT INTO sample(name, email, age) VALUES ($1, $2, $3) RETURNING id"#,
                "test",
                "test@example.com",
                10
            )
            .fetch_one(&pool)
            .await?,
        );

        repo.delete(DeleteSample { id }).await?;

        let count = sqlx::query_scalar!(r#"SELECT count(*) FROM sample WHERE id = $1"#, id.get())
            .fetch_one(&pool)
            .await?;

        assert_eq!(count, Some(0));

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_ng_not_found(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = SampleRepositoryImpl::new(Arc::new(ConnectionPool::new(pool.clone())));

        let res = repo
            .delete(DeleteSample {
                id: SampleId::new(-1),
            })
            .await;

        assert!(res.is_err());
        match res {
            Err(AppError::EntityNotFound(_)) => {}
            _ => panic!("Unexpected error"),
        }

        Ok(())
    }
}
