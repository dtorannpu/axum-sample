use crate::handler::sample;
use axum::Router;
use axum::routing::{delete, get, post, put};
use registry::AppState;

pub fn build_sample_routers() -> Router<AppState> {
    let sample_routes = Router::new()
        .route("/", post(sample::register))
        .route("/", get(sample::show_sample_list))
        .route("/{id}", get(sample::show_sample))
        .route("/{id}", put(sample::update_sample))
        .route("/{id}", delete(sample::delete_sample));

    Router::new().nest("/sample", sample_routes)
}
