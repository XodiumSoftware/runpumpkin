mod build;
mod download;

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Downloads the Pumpkin server binary if needed, builds the plugin as a WASM target,
/// copies it into `.server/plugins/`, and launches the server.
fn main() -> Result<()> {
    let server_dir = PathBuf::from(".server");
    let plugins_dir = server_dir.join("plugins");

    std::fs::create_dir_all(&plugins_dir)?;

    let pumpkin_bin = download::Downloader::new()?.get_pumpkin()?;

    println!("Building plugin...");

    let wasm = build::Builder::build()?;
    let dest = plugins_dir.join(wasm.file_name().context("wasm path has no filename")?);

    std::fs::copy(&wasm, &dest)
        .with_context(|| format!("failed to copy {} to plugins/", wasm.display()))?;

    println!("Running Pumpkin...");
    Command::new(&pumpkin_bin)
        .current_dir(&server_dir)
        .status()
        .context("failed to run Pumpkin")?;

    Ok(())
}
