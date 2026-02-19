// Responsibility: Wire HTTP routes to their handlers and expose them for the server setup.
// Design reasoning: Grouping routes keeps router composition transparent while allowing route-specific middleware.
// Extension guidance: Add new submodules and extend the router in `router()` when expanding the surface area.
// Security considerations: Each module sanitizes inputs and relies on AppError for consistent responses.

pub mod api;
pub mod dashboard;
pub mod download;
pub mod static_files;

pub use api::api_handler;
pub use dashboard::dashboard_handler;
pub use download::download_handler;
pub use static_files::static_handler;
