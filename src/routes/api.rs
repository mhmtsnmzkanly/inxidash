// Responsibility: Expose the /api/system endpoint that proxies to the inxi service.
// Design reasoning: Query parsing, mode validation, and JSON serialization stay within a focused handler.
// Extension guidance: Add pagination or caching headers here when performance enhancements are introduced.
// Security considerations: All mode inputs are validated against a fixed allowlist and sanitized in the parser.

use axum::{
    extract::{Extension, Query},
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::config::DEFAULT_MODE;
use crate::error::AppError;
use crate::models::SystemReport;
use crate::services::{InxiMode, InxiService};

#[derive(Deserialize)]
pub(crate) struct ModeQuery {
    mode: Option<String>,
}

pub async fn api_handler(
    Extension(service): Extension<Arc<InxiService>>,
    Query(query): Query<ModeQuery>,
) -> Result<Json<SystemReport>, AppError> {
    let mode = query.mode.as_deref().unwrap_or(DEFAULT_MODE);
    let final_mode = InxiMode::parse(mode)?;
    let report = service.run(final_mode).await?;
    Ok(Json(report))
}
