use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

const PUMPKIN_RELEASE_TAG: &str = "nightly";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

/// Returns a path to the cached Pumpkin binary, downloading it first if needed.
pub fn get_pumpkin() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .context("could not determine cache directory")?
        .join("runpumpkin");

    std::fs::create_dir_all(&cache_dir)?;

    let client = reqwest::blocking::Client::builder()
        .user_agent("cargo-run-pumpkin")
        .build()?;
    let release: Release = client
        .get(format!(
            "https://api.github.com/repos/Pumpkin-MC/Pumpkin/releases/tags/{PUMPKIN_RELEASE_TAG}"
        ))
        .send()?
        .error_for_status()?
        .json()?;

    let version = &release.tag_name;
    let bin_path = cache_dir.join(format!("pumpkin-{version}{}", exe_suffix()));

    if bin_path.exists() {
        return Ok(bin_path);
    }

    let asset = release
        .assets
        .iter()
        .find(|a| asset_matches(&a.name))
        .with_context(|| {
            format!(
                "no Pumpkin binary found for {}/{}",
                std::env::consts::OS,
                std::env::consts::ARCH
            )
        })?;

    println!("Downloading Pumpkin {} ...", version);

    let bytes = client
        .get(&asset.browser_download_url)
        .send()?
        .error_for_status()?
        .bytes()?;

    std::fs::write(&bin_path, &bytes)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&bin_path, std::fs::Permissions::from_mode(0o755))?;
    }

    Ok(bin_path)
}

/// Returns the platform-specific executable suffix (`.exe` on Windows, empty string elsewhere).
fn exe_suffix() -> &'static str {
    if cfg!(windows) { ".exe" } else { "" }
}

/// Returns `true` if the release asset `name` matches the current OS and architecture.
///
/// Matches are case-insensitive and support common naming conventions
/// (e.g. `amd64`/`x86_64`/`x64`, `darwin`/`macos`, `.exe` for Windows).
fn asset_matches(name: &str) -> bool {
    let name = name.to_lowercase();
    let os_match = match std::env::consts::OS {
        "linux" => name.contains("linux"),
        "windows" => name.contains("windows") || name.ends_with(".exe"),
        "macos" => name.contains("darwin") || name.contains("macos"),
        _ => false,
    };
    let arch_match = match std::env::consts::ARCH {
        "x86_64" => name.contains("x86_64") || name.contains("amd64") || name.contains("x64"),
        "aarch64" => name.contains("aarch64") || name.contains("arm64"),
        _ => false,
    };

    os_match && arch_match
}
