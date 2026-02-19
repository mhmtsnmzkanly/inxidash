// Responsibility: Serve the single-page dashboard HTML.
// Design reasoning: Keeps the HTML renderer isolated so routing code just forwards the generated markup.
// Extension guidance: Add middleware or preflight checks for the dashboard route as needed.
// Security considerations: Handler uses sanitized, static markup, never interpolating untrusted data.

use crate::rendering::dashboard_page;
use axum::response::Html;

pub async fn dashboard_handler() -> Html<String> {
    Html(dashboard_page())
}
