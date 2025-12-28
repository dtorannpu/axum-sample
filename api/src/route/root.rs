use crate::handler::root;
use axum::Router;
use axum::routing::get;
use registry::AppRegistry;

pub fn build_root_routers() -> Router<AppRegistry> {
    Router::new().route("/", get(root::root))
}
