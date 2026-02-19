// Responsibility: Define the DTOs produced by parsing inxi output and served via JSON.
// Design reasoning: Structuring the report makes serialization predictable and keeps parsing logic testable.
// Extension guidance: Add precise field types or nested structures when extracting richer metadata.
// Security considerations: Treat sensitive strings as raw text without executing them or exposing beyond this schema.

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SystemReport {
    pub timestamp: u64,
    pub mode: String,
    pub sections: Vec<SystemSection>,
}

#[derive(Serialize, Debug)]
pub struct SystemSection {
    pub title: String,
    pub entries: Vec<SystemEntry>,
}

#[derive(Serialize, Debug)]
pub struct SystemEntry {
    pub key: String,
    pub value: String,
}
