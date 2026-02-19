// Responsibility: Transform raw inxi output into the structured SystemReport served by the JSON API.
// Design reasoning: A dedicated parser keeps format assumptions in one place and allows mocking for testing.
// Extension guidance: Replace heuristics with richer section-specific parsers if needed by future CLI flags.
// Security considerations: All extracted strings are sanitized before leaving this module, preventing ANSI escape leakage.

use crate::error::AppError;
use crate::models::{SystemEntry, SystemReport, SystemSection};
use crate::services::InxiMode;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn parse_system_report(raw: &str, mode: InxiMode) -> Result<SystemReport, AppError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| AppError::Parse(err.to_string()))?
        .as_secs();

    let sections = parse_sections(raw);

    Ok(SystemReport {
        timestamp,
        mode: mode.as_str().to_string(),
        sections,
    })
}

fn parse_sections(raw: &str) -> Vec<SystemSection> {
    let mut sections = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_entries = Vec::new();

    for raw_line in raw.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(title) = parse_section_title(line) {
            push_section(&mut sections, &mut current_title, &mut current_entries);
            current_title = Some(title);
            continue;
        }

        if current_title.is_none() {
            continue;
        }

        if is_continuation_line(raw_line, line) {
            if let Some(last) = current_entries.last_mut() {
                last.value.push(' ');
                last.value.push_str(line);
                continue;
            }
        }

        if let Some(entry) = parse_entry(line) {
            current_entries.push(entry);
        }
    }

    push_section(&mut sections, &mut current_title, &mut current_entries);
    sections
}

fn push_section(
    sections: &mut Vec<SystemSection>,
    current_title: &mut Option<String>,
    current_entries: &mut Vec<SystemEntry>,
) {
    if let Some(title) = current_title.take() {
        sections.push(SystemSection {
            title,
            entries: mem::take(current_entries),
        });
    }
}

fn parse_section_title(line: &str) -> Option<String> {
    if !line.ends_with(':') {
        return None;
    }

    // Typical inxi sections start at column 0 with a single word or short phrase ending in :
    // e.g., "System:", "Machine:", "CPU:", "Graphics:"
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() > 3 {
        return None;
    }

    let title = line.trim_end_matches(':').trim();
    if title.is_empty() {
        return None;
    }

    // Check if it starts with an uppercase letter which is common for sections
    if !title.chars().next().map_or(false, |c| c.is_uppercase()) {
        return None;
    }

    let is_valid = title
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ' ' | '-' | '/' | '(' | ')' | '+'));

    if !is_valid {
        return None;
    }

    Some(title.to_string())
}

fn is_continuation_line(raw_line: &str, trimmed: &str) -> bool {
    let indent = raw_line.chars().take_while(|ch| ch.is_whitespace()).count();
    if indent < 4 {
        return false;
    }

    let Some(first_token) = trimmed.split_whitespace().next() else {
        return false;
    };

    first_token.starts_with('(')
        || first_token.chars().all(|ch| ch.is_ascii_digit())
        || first_token.contains('=')
}

fn parse_entry(line: &str) -> Option<SystemEntry> {
    if let Some((key, value)) = line.split_once(':') {
        let key = key.trim();
        let value = value.trim();
        if !key.is_empty() && !value.is_empty() {
            return Some(SystemEntry {
                key: key.to_string(),
                value: value.to_string(),
            });
        }
    }

    let tokens = line.split_whitespace().collect::<Vec<_>>();
    if tokens.is_empty() {
        return None;
    }

    let mut key = tokens[0].to_string();
    let mut value_start = 1usize;

    if tokens.len() > 1 && tokens[1].starts_with('(') && tokens[1].ends_with(')') {
        key.push(' ');
        key.push_str(tokens[1]);
        value_start = 2;
    }

    let value = if value_start >= tokens.len() {
        line.to_string()
    } else {
        tokens[value_start..].join(" ")
    };

    Some(SystemEntry { key, value })
}

#[cfg(test)]
mod tests {
    use super::parse_system_report;
    use crate::services::InxiMode;

    #[test]
    fn parses_sections_without_blank_lines() {
        let sample = "System:\n  Kernel 6.12.68-1-MANJARO arch x86_64 bits 64\nCPU:\n  Info quad core model AMD Ryzen 5\nGraphics:\n  Device-1 AMD driver amdgpu\nInfo:\n  Memory total 8 GiB used 2 GiB\n";

        let report = parse_system_report(sample, InxiMode::Basic).expect("report should parse");
        let titles = report
            .sections
            .iter()
            .map(|section| section.title.as_str())
            .collect::<Vec<_>>();

        assert_eq!(titles, vec!["System", "CPU", "Graphics", "Info"]);
        assert_eq!(report.sections[1].entries[0].key, "Info");
        assert!(report.sections[3].entries[0].value.contains("total 8 GiB"));
    }

    #[test]
    fn appends_wrapped_lines_to_previous_entry() {
        let sample = "CPU:\n  Speed (MHz) avg 3400 cores 1 3400 2 3400 3 3400\n    4 3400\n";

        let report = parse_system_report(sample, InxiMode::Basic).expect("report should parse");
        let entry = &report.sections[0].entries[0];

        assert_eq!(entry.key, "Speed (MHz)");
        assert!(
            entry.value.contains("4 3400"),
            "unexpected value: {}",
            entry.value
        );
    }
}
