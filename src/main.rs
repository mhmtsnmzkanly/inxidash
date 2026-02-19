// Responsibility: Bootstrap the HTTP server, configure observability, and wire routes to services.
// Design reasoning: Keeping startup logic centralized ensures configuration, dependency checks, and server lifecycle are coherent.
// Extension guidance: Add metrics, global middleware, or feature flags here while keeping route definitions intact.
// Security considerations: We verify required binaries early to fail fast rather than later allowing undefined behavior.

mod config;
mod error;
mod generated_assets;
mod models;
mod rendering;
mod routes;
mod services;
mod utils;

use axum::{Extension, Router, routing::get, serve};
use std::{io, process::Stdio, sync::Arc};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::config::{API_ROUTE, BIND_ADDR, DASHBOARD_ROUTE, DOWNLOAD_ROUTE, STATIC_ROUTE};
use crate::error::AppError;
use crate::routes::{api_handler, dashboard_handler, download_handler, static_handler};
use crate::services::InxiService;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        tracing::error!(error = %err, "service terminated");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("inxi_to_html=info".parse().unwrap()),
        )
        .init();

    tracing::info!(version = env!("CARGO_PKG_VERSION"), "inxi-dash starting");
    ensure_inxi_available()?;

    let service = Arc::new(InxiService::new());

    let router = Router::new()
        .route(DASHBOARD_ROUTE, get(dashboard_handler))
        .route(API_ROUTE, get(api_handler))
        .route(DOWNLOAD_ROUTE, get(download_handler))
        .route(STATIC_ROUTE, get(static_handler))
        .layer(Extension(service));

    let listener = TcpListener::bind(BIND_ADDR)
        .await
        .map_err(|err| AppError::Io(io::Error::new(io::ErrorKind::AddrNotAvailable, err)))?;

    let addr = listener
        .local_addr()
        .map_err(|err| AppError::Io(io::Error::new(io::ErrorKind::Other, err)))?;

    tracing::info!(address = %addr, "binding server");
    serve(listener, router)
        .await
        .map_err(|err| AppError::Io(err))?;

    Ok(())
}

fn ensure_inxi_available() -> Result<(), AppError> {
    let status = std::process::Command::new("inxi")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(state) if state.success() => {
            tracing::info!("verified inxi binary availability");
            Ok(())
        }
        Ok(_) => Err(AppError::CommandFailure(
            "inxi --version returned non-zero".to_string(),
        )),
        Err(_) => Err(AppError::MissingBinary("inxi")),
    }
}
