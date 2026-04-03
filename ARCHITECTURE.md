# ARCHITECTURE.md

This file provides guidance when working with code in this repository.

## Project Overview

`runpumpkin` is a native CLI tool that automates local Pumpkin plugin development. It downloads the latest Pumpkin server binary, builds the current workspace as a `wasm32-wasip2` plugin, copies it into the server's `plugins/` directory, and launches the server.

## Build & Run Commands

```bash
# Build the tool
cargo build --release

# Run the tool (from a plugin workspace)
./target/release/runpumpkin
```

## Architecture

### Entry Point

**`src/main.rs`** — orchestration only:

1. Creates `.server/plugins/` in the current directory.
2. Calls `download::Downloader::new()?.get_pumpkin()` to obtain a cached or freshly downloaded Pumpkin binary.
3. Calls `build::Builder::build()` to compile the plugin and locate the built `.wasm`.
4. Copies the `.wasm` into `.server/plugins/`.
5. Launches the Pumpkin server from `.server/`.

**`src/download.rs`** — Pumpkin binary management (`Downloader`):

- Queries the GitHub Releases API for the latest Pumpkin release.
- Caches the binary in the OS cache directory (`<cache>/runpumpkin/pumpkin-<version>[.exe]`).
- Selects the correct asset by matching OS and architecture against the release asset names.
- Sets executable permissions on Unix after download.

**`src/build.rs`** — plugin compilation and artifact location (`Builder`):

- Runs `cargo build --release --target wasm32-wasip2` on the current workspace.
- Uses `cargo metadata` to derive the package name and locate the built `.wasm` file.

### Package Structure

| Path              | Contents                                |
|-------------------|-----------------------------------------|
| `src/main.rs`     | CLI entry point, orchestration          |
| `src/download.rs` | Pumpkin binary download and caching     |
| `src/build.rs`    | Plugin compilation and `.wasm` location |

### Key Conventions

- All Clippy warnings are enabled (`[lints.clippy] all = "warn"`).
- `unsafe_code` is forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`).
- The tool uses `reqwest::blocking` for HTTP; no async runtime.
- The Pumpkin binary cache is keyed by release tag, so updates are picked up automatically on the next run when a new release is published.
