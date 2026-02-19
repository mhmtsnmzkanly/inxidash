// Responsibility: Provide embedded static asset lookup compiled from build.rs.
// Design reasoning: Matching paths to include_str! keeps the runtime server free of filesystem access.
// Extension guidance: Re-run build.rs or add files under static/; new entries appear automatically.
// Security considerations: Only tracked assets are served, eliminating directory traversal risks.
pub enum AssetContent {
    Text(&'static str),
    Binary(&'static [u8]),
}

pub fn get_asset(path: &str) -> Option<(&'static str, AssetContent)> {
    match path {
        "/static/README.md" => Some(("application/octet-stream", AssetContent::Binary(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/README.md"))))),
        "/static/css/app.css" => Some(("text/css", AssetContent::Text(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/css/app.css"))))),
        "/static/css/melt.css" => Some(("text/css", AssetContent::Text(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/css/melt.css"))))),
        "/static/example.html" => Some(("application/octet-stream", AssetContent::Binary(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/example.html"))))),
        "/static/icons/chip.png" => Some(("image/png", AssetContent::Binary(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/icons/chip.png"))))),
        "/static/icons/graphics-card.png" => Some(("image/png", AssetContent::Binary(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/icons/graphics-card.png"))))),
        "/static/icons/keyboard-and-mouse.png" => Some(("image/png", AssetContent::Binary(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/icons/keyboard-and-mouse.png"))))),
        "/static/icons/mainboard.png" => Some(("image/png", AssetContent::Binary(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/icons/mainboard.png"))))),
        "/static/icons/ssd-drive.png" => Some(("image/png", AssetContent::Binary(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/icons/ssd-drive.png"))))),
        "/static/icons/ssd.png" => Some(("image/png", AssetContent::Binary(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/icons/ssd.png"))))),
        "/static/js/dashboard.js" => Some(("application/javascript", AssetContent::Text(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/js/dashboard.js"))))),
        "/static/js/melt.js" => Some(("application/javascript", AssetContent::Text(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/js/melt.js"))))),
        _ => None,
    }
}
