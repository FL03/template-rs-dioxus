/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use xtask_sdk::*;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get_service,
};
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");
    wasm_server().await?;

    cli()?;

    Ok(())
}

pub fn base_matches(sc: Command) -> ArgMatches {
    command!()
        .arg(
            arg!(
                config: -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(debug:
                -d --debug ... "Turn debugging information on"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(port: -p --port <PORT>)
                .help("Network port to use")
                .value_parser(value_parser!(u16).range(1..))
                .default_value("8080"),
        )
        .arg_required_else_help(true)
        .propagate_version(true)
        .subcommand(sc)
        .subcommand_required(false)
        .get_matches()
}

pub fn cli() -> anyhow::Result<()> {
    let matches = base_matches(
        Command::new("app")
            .about("Application commands")
            .arg(arg!(build: -b --build <BUILD> "Build the workspace"))
            .arg(arg!(serve: -s --serve <BUILD> "Build the workspace")),
    );

    let port: u16 = *matches.get_one::<u16>("PORT").unwrap();

    println!("{:?}", port);
    Ok(())
}

pub struct System {
    pub workdir: String,
}

impl System {
    pub fn new(workdir: Option<String>) -> Self {
        Self {
            workdir: workdir.unwrap_or_else(|| "/pkg".to_string()),
        }
    }
}

/// Quickstart a server for static assets
pub async fn wasm_server() -> anyhow::Result<()> {
    let root = project_root().to_str().unwrap().to_string();
    let dist = format!("{}/{}", root, "dist");

    let serve_dir = get_service(ServeDir::new(dist.as_str())).handle_error(handle_error);
    let app = axum::Router::new().nest_service("/", serve_dir);
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

pub struct Wasm {
    pub port: u16,
    pub workdir: String,
}

impl Wasm {
    pub fn new(port: u16, workdir: String) -> Self {
        Self { port, workdir }
    }
    pub async fn client(&self) -> axum::Router {
        let serve_dir =
            get_service(ServeDir::new(self.workdir.as_str())).handle_error(handle_error);
        axum::Router::new().nest_service("/", serve_dir)
    }
}

impl Default for Wasm {
    fn default() -> Self {
        let root = project_root().to_str().unwrap().to_string();
        let workdir = format!("{}/{}", root, "dist");
        Wasm::new(8080, workdir)
    }
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
