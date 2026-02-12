use axum::Json;
use axum::extract::{Query, State};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::application::AppState;

const DEFAULT_DATA: &[u8] = b"hello world";

#[derive(Debug, Deserialize)]
pub struct QuoteParams {
    data: Option<String>,
}

/// Root endpoint handler.
///
/// Returns basic service information including the service name and current timestamp.
/// This endpoint is typically used for service discovery and basic connectivity checks.
///
/// # Returns
///
/// JSON response containing:
/// - `service`: The service name ("dstack-quote-sidecar")
/// - `timestamp`: Current UTC timestamp in RFC3339 format
pub async fn root() -> Json<Value> {
    Json(json!({
        "service": "dstack-quote-sidecar",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

/// Health check endpoint handler.
///
/// Returns a simple "OK" response to indicate that the service is running.
/// This endpoint is typically used for health checks and service monitoring.
///
/// # Returns
///
/// JSON response containing:
/// - `status`: The status of the service ("ok")
pub async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Get quote endpoint handler.
///
/// Returns a quote for the given data. If no data is provided via query param,
/// uses the default value "hello world".
///
/// # Query Parameters
///
/// - `data`: Optional string to include in the quote
///
/// # Returns
///
/// JSON response containing:
/// - `quote`: The quote data
/// - `rtmrs`: The replay RTMRs from the event log
/// - `error`: Error message if the operation failed
pub async fn get_quote(
    State(state): State<AppState>,
    Query(params): Query<QuoteParams>,
) -> Json<Value> {
    let data = params
        .data
        .map(|s| s.into_bytes())
        .unwrap_or_else(|| DEFAULT_DATA.to_vec());

    match state.dstack_client.get_quote(data).await {
        Ok(quote) => match quote.replay_rtmrs() {
            Ok(rtmrs) => Json(json!({
                "quote": format!("{:?}", quote),
                "rtmrs": format!("{:?}", rtmrs)
            })),
            Err(e) => Json(json!({
                "error": format!("Failed to replay RTMRs: {}", e)
            })),
        },
        Err(e) => Json(json!({
            "error": format!("Failed to get quote: {}", e)
        })),
    }
}

/// Attest endpoint handler.
///
/// Generates an attestation quote for the given data. If no data is provided via query param,
/// uses the default value "hello world".
///
/// # Query Parameters
///
/// - `data`: Optional string to include in the attestation
///
/// # Returns
///
/// JSON response containing:
/// - `attestation`: The attestation data
/// - `error`: Error message if the operation failed
pub async fn attest(
    State(state): State<AppState>,
    Query(params): Query<QuoteParams>,
) -> Json<Value> {
    let data = params
        .data
        .map(|s| s.into_bytes())
        .unwrap_or_else(|| DEFAULT_DATA.to_vec());

    match state.dstack_client.attest(data).await {
        Ok(resp) => Json(json!({
            "attestation": resp.attestation
        })),
        Err(e) => Json(json!({
            "error": format!("Failed to attest: {}", e)
        })),
    }
}
