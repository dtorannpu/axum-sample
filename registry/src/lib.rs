use adapter::database::{ConnectionPool, ConnectionPoolParameters};
use adapter::repository::sample::SampleRepositoryImpl;
use axum::extract::FromRef;
use shaku::module;
use std::sync::Arc;

module! {
    pub AppModule {
        components = [ConnectionPool, SampleRepositoryImpl],
        providers = []
    }
}

impl AppModule {
    pub fn new(pool: ConnectionPool) -> Arc<Self> {
        Arc::new(
            AppModule::builder()
                .with_component_parameters::<ConnectionPool>(ConnectionPoolParameters {
                    pool: pool.inner_ref().clone(),
                })
                .build(),
        )
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub module: Arc<AppModule>,
    pub without_validation_arguments: (),
}

impl AppState {
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            module: AppModule::new(pool),
            without_validation_arguments: (),
        }
    }
}
