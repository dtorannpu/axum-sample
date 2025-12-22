use api::route::v1;
use axum::body::Body;
use axum::http::Request;
use rstest::rstest;
use tower::ServiceExt;

#[rstest]
#[tokio::test]
async fn show_sample() -> anyhow::Result<()> {
    let app = v1::routes();

    let req = Request::builder().uri("/v1/sample").body(Body::empty())?;
    let resp = app.oneshot(req).await?;

    assert_eq!(resp.status(), 200);

    let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
    assert_eq!(String::from_utf8(body.to_vec())?, "Hello, Sample!");

    Ok(())
}
