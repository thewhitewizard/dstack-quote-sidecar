use anyhow::{Context, Result};
use axum::{Router, routing::get};
use std::sync::Arc;

use dstack_sdk::dstack_client::DstackClient;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, warn};

use crate::config::Config;
use crate::handlers;

#[derive(Clone)]
pub struct AppState {
    pub dstack_client: Arc<DstackClient>,
}

pub struct Application {
    config: Config,
    state: AppState,
}

impl Application {
    pub fn new(config: Config) -> Result<Self> {
        let dstack_client = Arc::new(DstackClient::new(None));
        Ok(Self {
            config,
            state: AppState { dstack_client },
        })
    }

    fn build_router(&self) -> Router {
        debug!("Building application router");

        Router::new()
            // Root endpoint
            .route("/", get(handlers::root))
            // Health check endpoint
            .route("/health", get(handlers::health_check))
            // Quote endpoint
            .route("/quote", get(handlers::get_quote))
            // Attest endpoint
            .route("/attest", get(handlers::attest))
            .with_state(self.state.clone())
            .layer(TraceLayer::new_for_http())
    }

    pub async fn run(self) -> Result<()> {
        let addr = self.config.bind_addr();
        let app = self.build_router();
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .with_context(|| format!("Failed to bind server to address {}", addr))?;

        info!("Server bound to {}", addr);

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("Server encountered an error during execution")?;

        info!("Server shutdown complete");
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down gracefully...");
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down gracefully...");
        },
    }

    warn!("Shutdown signal received, cleaning up...");
}
