use crate::handler::root;
use axum::Router;
use axum::routing::get;
use registry::AppState;

pub fn build_root_routers() -> Router<AppState> {
    Router::new().route("/", get(root::root))
}
