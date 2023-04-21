/*
    Appellation: server <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A simple server for static assets
*/
use axum::routing::get_service;

use std::net::SocketAddr;
use tower_http::services::ServeDir;
use xtask_sdk::project_root;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");
    wasm_server().await?;

    Ok(())
}

/// Quickstart a server for static assets
pub async fn wasm_server() -> anyhow::Result<()> {
    let workdir = project_root().join("dist");
    let serve_dir = get_service(ServeDir::new(workdir.to_str().unwrap()));
    let app = axum::Router::new().nest_service("/", serve_dir);
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

pub struct XtaskServer {
    pub addr: SocketAddr
}

impl XtaskServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let root = project_root().to_str().unwrap().to_string();
        let dist = format!("{}/{}", root, "dist");

        let serve_dir = get_service(ServeDir::new(dist.as_str()));
        let app = axum::Router::new().nest_service("/", serve_dir);
        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
        Ok(())
    }
}