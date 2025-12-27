use api::route::v1;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use rstest::rstest;
use tower::ServiceExt;

#[rstest]
#[tokio::test]
async fn show_sample() -> anyhow::Result<()> {
    let app = v1::routes();

    let req = Request::builder().uri("/v1/sample").body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
    assert_eq!(String::from_utf8(body.to_vec())?, "Hello, Sample!");

    Ok(())
}

#[rstest]
#[tokio::test]
async fn register_sample_ok() -> anyhow::Result<()> {
    let app = v1::routes();

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
async fn register_sample_ng() -> anyhow::Result<()> {
    let app = v1::routes();

    let req = Request::builder()
        .method("POST")
        .uri("/v1/sample")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name": "", "age": 10}"#))?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    Ok(())
}
