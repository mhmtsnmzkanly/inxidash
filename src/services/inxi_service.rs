// Responsibility: Encapsulate how we invoke inxi with allowed modes and ensure its output is sanitized.
// Design reasoning: This service isolates platform calls so we can swap mocking or extend caching without touching routes.
// Extension guidance: Add buffering, caching, or new modes by keeping the interface unchanged and expanding the mode enum.
// Security considerations: No user input is forwarded directly to the shell—modes map to fixed argument lists and ANSI sequences are stripped later.

use crate::error::AppError;
use crate::models::SystemReport;
use crate::services::parser;
use crate::utils::strip_ansi;
use std::fmt;
use tokio::process::Command;

#[derive(Clone, Copy, Debug)]
pub enum InxiMode {
    Basic,
    Full,
    Verbose,
    Maximum,
}

impl InxiMode {
    pub fn args(&self) -> &'static [&'static str] {
        match self {
            InxiMode::Basic => &["-F"],
            InxiMode::Full => &["-F", "-z"],
            InxiMode::Verbose => &["-a", "-F", "-z"],
            InxiMode::Maximum => &["-a", "-F", "-x", "-x", "-x", "-z"],
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            InxiMode::Basic => "basic",
            InxiMode::Full => "full",
            InxiMode::Verbose => "verbose",
            InxiMode::Maximum => "maximum",
        }
    }

    pub fn parse(input: &str) -> Result<Self, AppError> {
        let normalized = input.trim().to_lowercase();
        match normalized.as_str() {
            "basic" => Ok(InxiMode::Basic),
            "full" => Ok(InxiMode::Full),
            "verbose" => Ok(InxiMode::Verbose),
            "maximum" => Ok(InxiMode::Maximum),
            _ => Err(AppError::InvalidMode(input.to_string())),
        }
    }
}

impl fmt::Display for InxiMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct InxiService;

impl InxiService {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self, mode: InxiMode) -> Result<SystemReport, AppError> {
        tracing::info!(command = "inxi", mode = %mode, args = ?mode.args(), "running inxi");
        let output = Command::new("inxi")
            .args(mode.args())
            .output()
            .await
            .map_err(|err| AppError::CommandFailure(err.to_string()))?;

        if !output.status.success() {
            return Err(AppError::CommandFailure(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let raw = String::from_utf8_lossy(&output.stdout).to_string();
        let cleaned = strip_ansi(&raw);
        parser::parse_system_report(&cleaned, mode)
    }
}
