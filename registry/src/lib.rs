use axum::extract::FromRef;
use shaku::module;
use std::sync::Arc;

module! {
    pub AppModule {
        components = [],
        providers = []
    }
}

impl AppModule {
    pub fn new() -> Arc<Self> {
        Arc::new(AppModule::builder().build())
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub module: Arc<AppModule>,
    pub without_validation_arguments: (),
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            module: AppModule::new(),
            without_validation_arguments: (),
        }
    }
}
