use adapter::database::{ConnectionPool, DbPool};
use api::route::v1;
use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
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
async fn show_sample(
    db_pool: Box<dyn DbPool>,
    mut sample_repository: Box<MockTestSampleRepository>,
) -> anyhow::Result<()> {
    sample_repository.expect_find_all().returning(move || {
        Ok(vec![
            Sample {
                id: 1.into(),
                name: "Sample".to_string(),
                age: 20,
            },
            Sample {
                id: 2.into(),
                name: "Sample2".to_string(),
                age: 30,
            },
        ])
    });
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

    let req = Request::builder().uri("/v1/sample").body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
    assert_eq!(
        String::from_utf8(body.to_vec())?,
        r#"{"samples":[{"name":"Sample","age":20},{"name":"Sample2","age":30}]}"#
    );

    Ok(())
}

#[rstest]
#[tokio::test]
async fn register_sample_ok(
    db_pool: Box<dyn DbPool>,
    sample_repository: Box<dyn SampleRepository>,
) -> anyhow::Result<()> {
    let state = AppState {
        module: Arc::new(
            AppModule::builder()
                .with_component_override(db_pool)
                .with_component_override(sample_repository)
                .build(),
        ),
        without_validation_arguments: (),
    };
    let app = v1::routes().with_state(state);

    let req = Request::builder()
        .method("POST")
        .uri("/v1/sample")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name": "test", "age": 10}"#))?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    Ok(())
}

#[rstest]
#[tokio::test]
async fn register_sample_ng(
    db_pool: Box<dyn DbPool>,
    sample_repository: Box<dyn SampleRepository>,
) -> anyhow::Result<()> {
    let state = AppState {
        module: Arc::new(
            AppModule::builder()
                .with_component_override(db_pool)
                .with_component_override(sample_repository)
                .build(),
        ),
        without_validation_arguments: (),
    };
    let app = v1::routes().with_state(state);

    let req = Request::builder()
        .method("POST")
        .uri("/v1/sample")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name": "", "age": 10}"#))?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    Ok(())
}
