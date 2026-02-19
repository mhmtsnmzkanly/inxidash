// Responsibility: Build the dashboard HTML strings consumed by the router and download exporter.
// Design reasoning: Centralizing markup construction ensures addition of features (download, themes, sections) is confined here.
// Extension guidance: Add helper builders for new UI sections or alternative layouts without touching routing.
// Security considerations: This module escapes dynamic values before embedding them in the HTML to prevent injection.

use crate::config::THEME_SUGGESTIONS;
use crate::error::AppError;
use crate::generated_assets;
use crate::models::{SystemReport, SystemSection};
use crate::rendering::theme::THEME_OPTIONS;

const MODE_OPTIONS: &[(&str, &str)] = &[
    ("basic", "Basic"),
    ("full", "Full"),
    ("verbose", "Verbose"),
    ("maximum", "Maximum"),
];

struct CategoryConfig {
    label: &'static str,
    keywords: &'static [&'static str],
}

const CATEGORY_CONFIG: &[CategoryConfig] = &[
    CategoryConfig {
        label: "CPU",
        keywords: &["cpu", "processor", "core", "thread", "cache", "mhz", "ghz"],
    },
    CategoryConfig {
        label: "GPU",
        keywords: &[
            "gpu", "graphics", "video", "vram", "display", "vulkan", "opengl",
        ],
    },
    CategoryConfig {
        label: "Memory",
        keywords: &["memory", "ram", "swap", "slot", "dimm", "channel"],
    },
    CategoryConfig {
        label: "Motherboard",
        keywords: &[
            "machine",
            "mobo",
            "motherboard",
            "board",
            "bios",
            "chipset",
            "firmware",
        ],
    },
    CategoryConfig {
        label: "Storage",
        keywords: &[
            "drives",
            "drive",
            "disk",
            "storage",
            "ssd",
            "hdd",
            "nvme",
            "partition",
        ],
    },
    CategoryConfig {
        label: "Network",
        keywords: &[
            "network", "ethernet", "wifi", "lan", "wireless", "wlan", "if",
        ],
    },
    CategoryConfig {
        label: "Power",
        keywords: &["power", "volt", "battery", "charging"],
    },
    CategoryConfig {
        label: "General",
        keywords: &[],
    },
];

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

pub fn dashboard_page() -> String {
    let theme_options = THEME_OPTIONS
        .iter()
        .map(|(value, label)| format!("<option value=\"{value}\">{label}</option>"))
        .collect::<Vec<_>>()
        .join("");

    let mode_options = MODE_OPTIONS
        .iter()
        .map(|(value, label)| format!("<option value=\"{value}\">{label}</option>"))
        .collect::<Vec<_>>()
        .join("");

    format!(
        r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Inxi System Dashboard</title>
  <link rel="preconnect" href="/" />
  <link rel="stylesheet" href="/static/css/melt.css" />
  <link rel="stylesheet" href="/static/css/app.css" />
</head>
<body theme="default">
  <app class="app-shell">
    <header class="backdrop-blur">
      <div class="container flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="bg-primary p-2 rounded-2 text-white shadow-sm">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect><line x1="8" y1="21" x2="16" y2="21"></line><line x1="12" y1="17" x2="12" y2="21"></line></svg>
          </div>
          <h1 class="text-xl font-bold m-0 hide-sm">Inxi Dashboard</h1>
        </div>
      </div>
    </header>

    <main class="container py-4">
      <section class="hero mb-4">
        <div class="py-4">
          <h2 class="font-bold mb-2">System Performance & Specs</h2>
          <p class="text-muted text-lg max-w-lg">Comprehensive hardware overview generated from inxi. Real-time data visualization for CPU, Memory, Storage, and Graphics.</p>
          <div id="status-text" class="status-badge inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-primary-light text-primary">
            Initializing dashboard...
          </div>
        </div>
      </section>

      <div class="grid grid-md-1 gap-3 items-start" style="grid-template-columns: 280px 1fr;">
        <aside>
          <div class="card p-3 position-sticky top-0" style="top: 80px;">
            <h3 class="text-sm font-bold uppercase tracking-wider text-muted mb-3">Controls</h3>
            
            <div class="form-group">
              <label class="form-label text-xs">Detail Level</label>
              <select id="mode-select" class="form-item form-item-sm">{mode_options}</select>
            </div>

            <div class="form-group">
              <label class="form-label text-xs">Visual Theme</label>
              <select id="theme-select" class="form-item form-item-sm">{theme_options}</select>
            </div>

            <div class="mt-4 flex flex-col gap-2">
              <button id="refresh-button" class="btn btn-primary w-100" type="button">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2"><path d="M23 4v6h-6"></path><path d="M1 20v-6h6"></path><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path></svg>
                Refresh Data
              </button>
              <a id="download-link" class="btn btn-outline w-100" href="/download?mode=maximum">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
                Download Snapshot
              </a>
            </div>

            <div class="mt-4 pt-4 border-t border-dashed">
              <h4 class="text-xs font-bold uppercase tracking-wider text-muted mb-2">About</h4>
              <p class="text-xs text-muted leading-relaxed">Generated via <code>inxi</code> tool. Data is categorized into primary hardware components for better readability.</p>
            </div>
          </div>
        </aside>

        <section class="component-grid" id="component-cards" aria-label="System components"></section>
      </div>
    </main>

    <footer class="container text-sm text-muted py-8 text-center border-t">
      <p class="mb-2">Inxi System Dashboard · Standalone Version</p>
      <p class="mb-2 text-xs opacity-75">Icons from <a href="https://www.flaticon.com/" class="text-muted" target="_blank">Flaticon</a></p>
      <p class="m-0 text-xs opacity-50">Theme defaults: {themes}.</p>
    </footer>
  </app>
  <script src="/static/js/melt.js"></script>
  <script src="/static/js/dashboard.js"></script>
</body>
</html>"##,
        mode_options = mode_options,
        theme_options = theme_options,
        themes = THEME_SUGGESTIONS
            .iter()
            .map(|(_, label)| *label)
            .collect::<Vec<_>>()
            .join(" · ")
    )
}

pub fn download_page(report: &SystemReport) -> Result<String, AppError> {
    let css = asset_content("/static/css/melt.css")?;
    let app_css = asset_content("/static/css/app.css")?;
    let js = asset_content("/static/js/dashboard.js")?;

    let categorized = categorize_sections(&report.sections);
    let sections_html = categorized
        .iter()
        .map(|category| {
            let sections_block = category
                .sections
                .iter()
                .map(|section| {
                    let rows = section
                        .entries
                        .iter()
                        .map(|entry| {
                            format!(
                                r#"<tr>
  <td class="font-semibold text-muted">{key}</td>
  <td class="entry-value">{value}</td>
</tr>"#,
                                key = escape_html(&entry.key),
                                value = escape_html(&entry.value)
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("");

                    format!(
                        r#"<div class="report-section mb-4 last-mb-0">
  <h3 class="text-xs font-bold uppercase tracking-wider text-primary mb-2 pl-3 border-l-2 border-primary">{title}</h3>
  <table class="report-table">
    <thead>
      <tr>
        <th>Metric</th>
        <th>Value</th>
      </tr>
    </thead>
    <tbody>
      {rows}
    </tbody>
  </table>
</div>"#,
                        title = escape_html(&section.title),
                        rows = rows
                    )
                })
                .collect::<Vec<_>>()
                .join("");

            format!(
                r#"<article class="card component-card shadow-sm">
  <header class="component-card-header">
    <div class="component-title">
      <div class="insight-media">
        <h4 class="m-0">{label}</h4>
        <p class="component-meta">{count} technical sections</p>
      </div>
    </div>
  </header>
  <div class="card-body p-0">
    <div class="p-3">
      {sections}
    </div>
  </div>
</article>"#,
                label = category.label,
                count = category.sections.len(),
                sections = sections_block
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let timestamp = report.timestamp;
    let when = format!("UTC {timestamp}");

    Ok(format!(
        r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Inxi Snapshot ({mode})</title>
  <style>{css}
{app_css}
.last-mb-0:last-child {{ margin-bottom: 0 !important; }}
</style>
</head>
<body theme="default">
  <app class="app-shell">
    <header class="backdrop-blur">
      <div class="container flex items-center justify-between">
        <div class="flex items-center gap-3">
          <h1 class="text-xl font-bold m-0">Inxi System Snapshot</h1>
        </div>
        <span class="text-xs font-semibold px-2 py-1 rounded bg-primary-light text-primary">{mode}</span>
      </div>
    </header>

    <main class="container py-4">
      <section class="hero mb-4 text-center">
        <h2 class="font-bold mb-2">Hardware Configuration Export</h2>
        <p class="text-muted">Generated on {when}</p>
      </section>

      <section id="report-cards" class="grid gap-4" style="grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));">{sections}</section>
    </main>
    <footer class="container text-sm text-muted py-8 text-center border-t">
      <p class="mb-2">Standalone export generated by inxi-dash.</p>
      <p class="mb-0 text-xs opacity-75">Icons from <a href="https://www.flaticon.com/" class="text-muted" target="_blank">Flaticon</a></p>
    </footer>
  </app>
  <script>{js}</script>
</body>
</html>"##,
        css = css,
        app_css = app_css,
        sections = sections_html,
        when = when,
        mode = escape_html(&report.mode),
        js = js
    ))
}

fn asset_content(path: &str) -> Result<&'static str, AppError> {
    match generated_assets::get_asset(path) {
        Some((_, generated_assets::AssetContent::Text(text))) => Ok(text),
        Some((_, generated_assets::AssetContent::Binary(_))) => {
            Err(AppError::AssetNotFound(path.to_string()))
        }
        None => Err(AppError::AssetNotFound(path.to_string())),
    }
}

struct CategorySections<'a> {
    label: &'static str,
    keywords: &'static [&'static str],
    sections: Vec<&'a SystemSection>,
}

fn categorize_sections<'a>(sections: &'a [SystemSection]) -> Vec<CategorySections<'a>> {
    let mut buckets = CATEGORY_CONFIG
        .iter()
        .map(|config| CategorySections {
            label: config.label,
            keywords: config.keywords,
            sections: Vec::new(),
        })
        .collect::<Vec<_>>();

    for section in sections {
        let normalized_title = section.title.to_lowercase();
        let mut best_idx = None;
        let mut best_score = 0usize;

        for (idx, bucket) in buckets.iter().enumerate() {
            if bucket.label == "General" {
                continue;
            }

            let mut score = 0usize;
            if bucket
                .keywords
                .iter()
                .any(|keyword| normalized_title.contains(keyword))
            {
                score += 6;
            }

            for entry in &section.entries {
                let key = entry.key.to_lowercase();
                let value = entry.value.to_lowercase();
                for keyword in bucket.keywords {
                    if key.contains(keyword) {
                        score += 2;
                    }
                    if value.contains(keyword) {
                        score += 1;
                    }
                }
            }

            if score > best_score {
                best_score = score;
                best_idx = Some(idx);
            }
        }

        if let Some(idx) = best_idx {
            if best_score >= 2 {
                buckets[idx].sections.push(section);
                continue;
            }
        }

        if let Some(general) = buckets.iter_mut().find(|bucket| bucket.label == "General") {
            general.sections.push(section);
        }
    }

    buckets
        .into_iter()
        .filter(|bucket| !bucket.sections.is_empty())
        .collect()
}
