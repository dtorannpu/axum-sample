use crate::handler::root;
use axum::routing::get;
use axum::Router;
use registry::AppState;

pub fn build_root_routers() -> Router<AppState> {
    Router::new().route("/", get(root::root))
}
