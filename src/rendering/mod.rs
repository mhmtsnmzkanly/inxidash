// Responsibility: Expose rendering utilities for HTML templates used by routes and download exports.
// Design reasoning: Keeping markup generation centralized lets UI changes stay within rendering without touching routing logic.
// Extension guidance: Add new renderers or helpers here as components and plug them into the exposed API.
// Security considerations: Rendering code must escape any dynamic text before inclusion to avoid injection.

pub mod html_renderer;
pub mod theme;

pub use html_renderer::{dashboard_page, download_page};
