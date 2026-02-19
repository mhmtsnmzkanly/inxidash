// Responsibility: Aggregate utility helpers that support parsing and sanitizing text.
// Design reasoning: A dedicated utilities module avoids scattering small helper functions across the tree.
// Extension guidance: Add shared helpers or re-export crates here as new needs arise.
// Security considerations: Helpers must not rely on untrusted data when constructing command strings or file paths.

pub mod ansi;

pub use ansi::strip_ansi;
