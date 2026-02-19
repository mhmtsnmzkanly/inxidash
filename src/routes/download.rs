// Responsibility: Build a standalone HTML snapshot and stream it with a download header.
// Design reasoning: This handler mirrors the JSON API but enriches the report with inline assets and disposition headers.
// Extension guidance: Add additional query controls (e.g., include logs) by layering on top of this handler.
// Security considerations: Download export only reflects sanitized report data and known assets; no template injections are allowed.

use axum::{
    body::Body,
    extract::{Extension, Query},
    http::{StatusCode, header},
    response::Response,
};
use serde::Deserialize;
use std::{io, sync::Arc};

use crate::config::{DEFAULT_MODE, DOWNLOAD_FILENAME_PREFIX};
use crate::error::AppError;
use crate::rendering::download_page;
use crate::services::{InxiMode, InxiService};

#[derive(Deserialize)]
pub(crate) struct DownloadQuery {
    mode: Option<String>,
}

pub async fn download_handler(
    Extension(service): Extension<Arc<InxiService>>,
    Query(query): Query<DownloadQuery>,
) -> Result<Response, AppError> {
    let mode = query.mode.as_deref().unwrap_or(DEFAULT_MODE);
    let final_mode = InxiMode::parse(mode)?;
    let report = service.run(final_mode).await?;
    let html = download_page(&report)?;
    let filename = format!("{}-{}.html", DOWNLOAD_FILENAME_PREFIX, report.mode);
    let disposition = format!("attachment; filename=\"{filename}\"");

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header(header::CONTENT_DISPOSITION, disposition)
        .body(Body::from(html))
        .map_err(|err| AppError::Io(io::Error::new(io::ErrorKind::InvalidData, err)))?;

    Ok(response)
}
