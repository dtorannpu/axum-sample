use crate::handler::root;
use axum::Router;
use axum::routing::get;

pub fn build_root_routers() -> Router {
    Router::new().route("/", get(root::root))
}
