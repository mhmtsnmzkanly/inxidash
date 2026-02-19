// Responsibility: Re-export service modules that mediate between routes and low-level utilities.
// Design reasoning: Keeping a flat services namespace simplifies dependency inversion and testing in future.
// Extension guidance: Introduce new services here and provide a shared state if required.
// Security considerations: Validate service inputs before invoking system commands or parsing user data.

pub mod inxi_service;
pub mod parser;

pub use inxi_service::{InxiMode, InxiService};
