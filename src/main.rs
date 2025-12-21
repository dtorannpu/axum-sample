use anyhow::{Context, Result};
use api::route::v1;
use axum::Router;
use axum::http::Request;
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, SdkTracerProvider};
use opentelemetry_semantic_conventions::{
    SCHEMA_URL,
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION},
};
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<()> {
    let tracer_provider = init_tracing_subscriber()?;

    let router = Router::new().merge(v1::routes());
    let app = router.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let name = format!("{} {}", request.method(), request.uri());

                tracing::span!(
                    Level::INFO,
                    "request",
                    otel.name = name,
                    method = %request.method(),
                    uri = %request.uri(),
                    headers = ?request.headers(),
                    version = ?request.version(),
                )
            })
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Millis),
            ),
    );

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080);
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind")?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(tracer_provider))
        .await
        .context("Failed to start server")
}

fn resource() -> Resource {
    Resource::builder()
        .with_service_name("axum-sample")
        .with_schema_url(
            [
                KeyValue::new(SERVICE_VERSION, "1.0.0"),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
            ],
            SCHEMA_URL,
        )
        .build()
}

fn init_tracer_provider() -> Result<SdkTracerProvider> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

    Ok(SdkTracerProvider::builder()
        // Customize sampling strategy
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        // If export trace to AWS X-Ray, you can use XrayIdGenerator
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build())
}

fn init_tracing_subscriber() -> Result<SdkTracerProvider> {
    let tracer_provider = init_tracer_provider()?;

    let tracer = tracer_provider.tracer("axum-sample-tracer");

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    Ok(tracer_provider)
}

async fn shutdown_signal(tracer_provider: SdkTracerProvider) {
    fn purge_spans(tracer_provider: &SdkTracerProvider) {
        tracer_provider
            .shutdown()
            .expect("Failed to shut down tracer provider");
    }
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await
            .expect("Failed to receive SIGTERM signal");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c=>{
            tracing::info!("Ctrl+C を受信しました。");
            purge_spans(&tracer_provider);
        },
        _ = terminate=>{
            tracing::info!("SIGTERM を受信しました。");
            purge_spans(&tracer_provider);
        },
    }
}
