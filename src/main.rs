use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "rust_microservice_logging=debug,tower_http=debug,axum::rejection=trace".into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app = Router::new()
        .route("/health", get(handler));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

}

async fn handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "healthy"
    });

    Json(json_response)
}