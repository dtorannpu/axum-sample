use axum::extract::FromRef;
#[derive(Debug, Clone, FromRef)]
pub struct AppRegistry {
    pub without_validation_arguments: (),
}
