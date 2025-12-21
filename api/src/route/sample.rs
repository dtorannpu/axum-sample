use crate::handler::sample;
use axum::Router;
use axum::routing::get;

pub fn build_sample_routers() -> Router {
    Router::new().route("/sample", get(sample::sample))
}
