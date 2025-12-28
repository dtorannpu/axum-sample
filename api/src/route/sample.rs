use crate::handler::sample;
use axum::Router;
use axum::routing::{get, post};
use registry::AppRegistry;

pub fn build_sample_routers() -> Router<AppRegistry> {
    let sample_routes = Router::new()
        .route("/", get(sample::sample))
        .route("/", post(sample::register));

    Router::new().nest("/sample", sample_routes)
}
