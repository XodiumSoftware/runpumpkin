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

**`src/main.rs`** — top-level binary logic:

1. Creates `.pumpkin-server/plugins/` in the current directory.
2. Calls `download::get_pumpkin()` to obtain a cached or freshly downloaded Pumpkin binary.
3. Runs `cargo build --release --target wasm32-wasip2` on the current workspace.
4. Locates the built `.wasm` file via `cargo metadata` and copies it into `plugins/`.
5. Launches the Pumpkin server from `.pumpkin-server/`.

**`src/download.rs`** — Pumpkin binary management:

- Queries the GitHub Releases API for the latest Pumpkin release.
- Caches the binary in the OS cache directory (`<cache>/runpumpkin/pumpkin-<version>[.exe]`).
- Selects the correct asset by matching OS and architecture against the release asset names.
- Sets executable permissions on Unix after download.

### Package Structure

| Path              | Contents                             |
|-------------------|--------------------------------------|
| `src/main.rs`     | CLI entry point, build orchestration |
| `src/download.rs` | Pumpkin binary download and caching  |

### Key Conventions

- All Clippy warnings are enabled (`[lints.clippy] all = "warn"`).
- `unsafe_code` is forbidden project-wide (`[lints.rust] unsafe_code = "forbid"`).
- The tool uses `reqwest::blocking` for HTTP; no async runtime.
- The Pumpkin binary cache is keyed by release tag, so updates are picked up automatically on the next run when a new release is published.
