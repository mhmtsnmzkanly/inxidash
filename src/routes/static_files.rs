// Responsibility: Serve embedded assets at /static/* while avoiding filesystem access at runtime.
// Design reasoning: This handler delegates to generated asset map to keep runtime I/O minimal.
// Extension guidance: Extend caching headers here if you later support versioned assets.
// Security considerations: Only known paths are served; any unknown path returns a controlled 404.

use crate::config::STATIC_PREFIX;
use crate::error::AppError;
use crate::generated_assets::{self, AssetContent};
use axum::body::{Body, Bytes};
use axum::{extract::Path, http::header, response::Response};
use std::io;

pub async fn static_handler(Path(path): Path<String>) -> Result<Response, AppError> {
    let trimmed = path.trim_start_matches('/');
    let normalized = if trimmed.is_empty() {
        STATIC_PREFIX.to_string()
    } else {
        format!("{}/{}", STATIC_PREFIX.trim_end_matches('/'), trimmed)
    };

    if let Some((content_type, content)) = generated_assets::get_asset(&normalized) {
        let body = match content {
            AssetContent::Text(value) => Body::from(value),
            AssetContent::Binary(bytes) => Body::from(Bytes::from_static(bytes)),
        };
        let response = Response::builder()
            .header(header::CONTENT_TYPE, content_type)
            .body(body)
            .map_err(|err| AppError::Io(io::Error::new(io::ErrorKind::Other, err)))?;
        Ok(response)
    } else {
        Err(AppError::AssetNotFound(normalized))
    }
}
