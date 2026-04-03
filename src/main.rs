mod download;

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

const WASM_TARGET: &str = "wasm32-wasip2";

#[derive(Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
}

/// Downloads the Pumpkin server binary if needed, builds the plugin as a WASM target,
/// copies it into `.server/plugins/`, and launches the server.
fn main() -> Result<()> {
    let server_dir = PathBuf::from(".server");
    let plugins_dir = server_dir.join("plugins");

    std::fs::create_dir_all(&plugins_dir)?;

    let pumpkin_bin = download::get_pumpkin()?;

    println!("Building plugin...");

    let status = Command::new("cargo")
        .args(["build", "--release", "--target", WASM_TARGET])
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

/// Locates the compiled `.wasm` file for the current package under the `WASM_TARGET` release directory.
///
/// Uses `cargo metadata` to derive the package name, then verifies the file exists.
fn find_wasm() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .context("failed to run cargo metadata")?;
    let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)?;
    let name = metadata
        .packages
        .into_iter()
        .next()
        .context("cargo metadata returned no packages")?
        .name
        .replace('-', "_");
    let wasm = PathBuf::from(format!("target/{WASM_TARGET}/release/{name}.wasm"));

    if !wasm.exists() {
        bail!("expected wasm at {}, not found", wasm.display());
    }

    Ok(wasm)
}
