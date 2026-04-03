use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

/// The Cargo target triple used to compile plugins.
const WASM_TARGET: &str = "wasm32-wasip2";

/// Subset of `cargo metadata` output used to derive the package name.
#[derive(Deserialize)]
struct CargoMetadata {
    /// All packages in the workspace (only the first is used).
    packages: Vec<CargoPackage>,
}

/// A single package entry from `cargo metadata`.
#[derive(Deserialize)]
struct CargoPackage {
    /// The package name as declared in `Cargo.toml`.
    name: String,
}

/// Handles compiling the plugin and locating the built `.wasm` artifact.
pub struct Builder;

impl Builder {
    /// Compiles the plugin as a WASM target and returns the path to the built `.wasm` file.
    pub fn build() -> Result<PathBuf> {
        let status = Command::new("cargo")
            .args(["build", "--release", "--target", WASM_TARGET])
            .status()
            .context("failed to run cargo build")?;

        if !status.success() {
            bail!("cargo build failed");
        }

        Self::find_wasm()
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
}
