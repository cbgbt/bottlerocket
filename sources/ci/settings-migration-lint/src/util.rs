use anyhow::{Context, Result};
use lsp_types as lsp;
use std::path::Path;

pub fn crate_name(crate_dir: impl AsRef<Path>) -> Result<String> {
    let crate_dir = crate_dir.as_ref();
    let cargo_toml_path = crate_dir.join("Cargo.toml");

    let cargo_toml_str = std::fs::read_to_string(&cargo_toml_path).context(format!(
        "Failed to read Cargo.toml at '{}'",
        cargo_toml_path.display()
    ))?;
    let cargo_toml: toml::Table = toml::from_str(&cargo_toml_str).context(format!(
        "Failed to parse Cargo.toml at '{}'",
        cargo_toml_path.display()
    ))?;

    cargo_toml
        .get("package")
        .and_then(|package| package.get("name").and_then(|name| name.as_str()))
        .map(str::to_string)
        .context(format!(
            "Crate at '{}' seemingly has no package name",
            cargo_toml_path.display()
        ))
}

pub fn file_uri(file_path: impl AsRef<Path>) -> Result<lsp::Uri> {
    let file_path = file_path.as_ref().display();
    format!("file://{file_path}")
        .parse()
        .context(format!("Failed to create URI from file path '{file_path}'",))
}
