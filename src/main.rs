use anyhow::{Context, Result};
use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.context("Failed to bind")?;
    axum::serve(listener, app).await.context("Failed to start server")
}

async fn root() -> &'static str {
    "Hello, World!"
}
