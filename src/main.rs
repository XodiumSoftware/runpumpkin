mod download;

use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
    let server_dir = PathBuf::from(".pumpkin-server");
    let plugins_dir = server_dir.join("plugins");
    std::fs::create_dir_all(&plugins_dir)?;

    let pumpkin_bin = download::get_pumpkin()?;

    println!("Building plugin...");
    let status = Command::new("cargo")
        .args(["build", "--release", "--target", "wasm32-wasip2"])
        .status()
        .context("failed to run cargo build")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    let wasm = find_wasm().context("could not find built .wasm file")?;
    let dest = plugins_dir.join(wasm.file_name().unwrap());
    std::fs::copy(&wasm, &dest)
        .with_context(|| format!("failed to copy {} to plugins/", wasm.display()))?;

    println!("Running Pumpkin...");
    Command::new(&pumpkin_bin)
        .current_dir(&server_dir)
        .status()
        .context("failed to run Pumpkin")?;

    Ok(())
}

fn find_wasm() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .context("failed to run cargo metadata")?;

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let name = metadata["packages"][0]["name"]
        .as_str()
        .context("could not determine package name")?
        .replace('-', "_");

    let wasm = PathBuf::from(format!("target/wasm32-wasip2/release/{name}.wasm"));

    if !wasm.exists() {
        bail!("expected wasm at {}, not found", wasm.display());
    }

    Ok(wasm)
}
