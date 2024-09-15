/*
    Appellation: server <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use axum::Router;
use std::net::SocketAddr;
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const LOCALHOST: [u8; 4] = [127, 0, 0, 1];

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::join!(
        serve(using_serve_dir(), 3001),
    );
    Ok(())
}

fn dist() -> String {
    format!("{}/dist", ROOT)
}

fn using_serve_dir() -> Router {
    Router::new().nest_service("/", ServeDir::new("../../dist"))
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from((LOCALHOST, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}