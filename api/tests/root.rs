use adapter::database::{ConnectionPool, DbPool};
use api::route::v1;
use async_trait::async_trait;
use axum::body::Body;
use axum::http::Request;
use kernel::model::sample::Sample;
use kernel::repository::sample::SampleRepository;
use mockall::mock;
use registry::{AppModule, AppState};
use rstest::{fixture, rstest};
use shared::error::AppResult;
use std::sync::Arc;
use tower::ServiceExt;

mock! {
    TestDbPool {}
    impl DbPool for TestDbPool {
        fn get(&self) -> &ConnectionPool;
    }
}

mock! {
    TestSampleRepository {}
    #[async_trait]
    impl SampleRepository for TestSampleRepository {
        async fn find_all(&self) ->  AppResult<Vec<Sample>>;
    }
}

#[fixture]
fn db_pool() -> Box<dyn DbPool> {
    Box::new(MockTestDbPool::new())
}

#[fixture]
fn sample_repository() -> Box<MockTestSampleRepository> {
    Box::new(MockTestSampleRepository::new())
}

#[rstest]
#[tokio::test]
async fn show_root(
    db_pool: Box<dyn DbPool>,
    sample_repository: Box<MockTestSampleRepository>,
) -> anyhow::Result<()> {
    let state = AppState {
        module: Arc::new(
            AppModule::builder()
                .with_component_override(db_pool)
                .with_component_override::<dyn SampleRepository>(sample_repository)
                .build(),
        ),
        without_validation_arguments: (),
    };
    let app = v1::routes().with_state(state);

    let req = Request::builder().uri("/v1").body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), 200);

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
    assert_eq!(String::from_utf8(body.to_vec())?, "Hello, World!");

    Ok(())
}
