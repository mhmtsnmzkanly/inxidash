// Responsibility: Re-export models for easier consumption by services and routes.
// Design reasoning: Keeps the module tree flat while the models evolve with new system metadata.
// Extension guidance: Add more report types or DTOs here and export them centrally.
// Security considerations: Models expose only sanitized fields to downstream layers.

pub mod system_report;

pub use system_report::{SystemEntry, SystemReport, SystemSection};
