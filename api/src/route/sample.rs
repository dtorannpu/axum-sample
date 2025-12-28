use crate::handler::sample;
use axum::routing::{get, post};
use axum::Router;
use registry::AppState;

pub fn build_sample_routers() -> Router<AppState> {
    let sample_routes = Router::new()
        .route("/", get(sample::sample))
        .route("/", post(sample::register));

    Router::new().nest("/sample", sample_routes)
}
