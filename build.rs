use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let project_root =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set"));
    let static_dir = project_root.join("static");

    if !static_dir.exists() {
        panic!("'static' directory is required but missing");
    }

    println!("cargo:rerun-if-changed=static");

    let mut entries = Vec::new();

    for entry in WalkDir::new(&static_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let rel = path.strip_prefix(&project_root).unwrap();
            let normalized = rel
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            let route = format!("/{}", normalized);
            let mime = mime_type(path);
            entries.push((route, mime, normalized));
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut file = File::create(project_root.join("src/generated_assets.rs"))?;

    writeln!(
        file,
        "// Responsibility: Provide embedded static asset lookup compiled from build.rs."
    )?;
    writeln!(
        file,
        "// Design reasoning: Matching paths to include_str! keeps the runtime server free of filesystem access."
    )?;
    writeln!(
        file,
        "// Extension guidance: Re-run build.rs or add files under static/; new entries appear automatically."
    )?;
    writeln!(
        file,
        "// Security considerations: Only tracked assets are served, eliminating directory traversal risks."
    )?;
    writeln!(file, "pub enum AssetContent {{")?;
    writeln!(file, "    Text(&'static str),")?;
    writeln!(file, "    Binary(&'static [u8]),")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(
        file,
        "pub fn get_asset(path: &str) -> Option<(&'static str, AssetContent)> {{"
    )?;
    writeln!(file, "    match path {{")?;

    for (route, mime, full) in entries {
        let is_binary = matches!(
            mime,
            "image/png" | "image/jpeg" | "application/octet-stream"
        );
        let include = if is_binary {
            "include_bytes!"
        } else {
            "include_str!"
        };
        let variant = if is_binary { "Binary" } else { "Text" };
        writeln!(
            file,
            "        \"{route}\" => Some((\"{mime}\", AssetContent::{variant}({include}(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{full}\"))))),",
            route = route,
            mime = mime,
            variant = variant,
            include = include,
            full = full.replace('\\', "/")
        )?;
    }

    writeln!(file, "        _ => None,")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    Ok(())
}

fn mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        _ => "application/octet-stream",
    }
}
