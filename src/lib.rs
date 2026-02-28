use anyhow::Result;
use std::fs;
use std::path::Path;

pub const DEFAULT_CSS: &str = r#"body {
    font-family: system-ui, -apple-system, sans-serif;
    line-height: 1.5;
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    background: #fafafa;
    color: #333;
}
pre {
    background: #f0f0f0;
    padding: 1rem;
    border-radius: 5px;
    overflow-x: auto;
}
"#;

pub const DEFAULT_JS: &str = r#"console.log("RAWSSG site loaded");"#;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn copy_default_assets(output_dir: &Path) -> Result<()> {
    let style_dir = output_dir.join("style");
    let scripts_dir = output_dir.join("scripts");

    fs::create_dir_all(&style_dir)?;
    fs::create_dir_all(&scripts_dir)?;

    fs::write(style_dir.join("main.css"), DEFAULT_CSS)?;
    fs::write(scripts_dir.join("scripts.js"), DEFAULT_JS)?;

    Ok(())
}