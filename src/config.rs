// Responsibility: Define immutable configuration values and route constants used across the app.
// Design reasoning: Centralizing the binding address, default mode, and route IDs keeps routing and deployment changes isolated.
// Extension guidance: Add new feature flags, alternate addresses, or env-driven overrides here when needed.
// Security considerations: Avoid embedding secrets here; the values are safe immutable defaults for a local service.

pub const BIND_ADDR: &str = "127.0.0.1:3050";
pub const STATIC_PREFIX: &str = "/static";
pub const STATIC_ROUTE: &str = "/static/{*file}";
pub const API_ROUTE: &str = "/api/system";
pub const DOWNLOAD_ROUTE: &str = "/download";
pub const DASHBOARD_ROUTE: &str = "/";
pub const DEFAULT_MODE: &str = "basic";
pub const DOWNLOAD_FILENAME_PREFIX: &str = "inxi-dashboard";

/// Built-in theme options exposed to the UI without touching rendering logic.
pub const THEME_SUGGESTIONS: &[(&str, &str)] = &[
    ("default", "Balanced"),
    ("dark", "Night"),
    ("royal", "Royal"),
    ("glass", "Glass"),
];
