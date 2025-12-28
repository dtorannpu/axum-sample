use crate::route::root::build_root_routers;
use crate::route::sample::build_sample_routers;
use axum::Router;
use registry::AppState;

pub fn routes() -> Router<AppState> {
    let router = Router::new()
        .merge(build_root_routers())
        .merge(build_sample_routers());
    Router::new().nest("/v1", router)
}
