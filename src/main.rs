use anyhow::{Context, Result};
use axum::{Router, routing::get};
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080);
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind")?;
    axum::serve(listener, app)
        .await
        .context("Failed to start server")
}

async fn root() -> &'static str {
    "Hello, World!"
}
