use std::net::{Ipv4Addr, SocketAddr};
use anyhow::{Context, Result};
use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080);
    let listener = tokio::net::TcpListener::bind(addr).await.context("Failed to bind")?;
    axum::serve(listener, app).await.context("Failed to start server")
}

async fn root() -> &'static str {
    "Hello, World!"
}
