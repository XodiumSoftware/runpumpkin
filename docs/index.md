# runpumpkin

`runpumpkin` is a CLI tool that automates local [Pumpkin](https://github.com/Pumpkin-MC/Pumpkin) plugin development. Run it from your plugin workspace and it will:

1. Download the latest Pumpkin server binary (cached by version).
2. Build your plugin as a `wasm32-wasip2` WASM binary.
3. Copy the built `.wasm` into the server's `plugins/` folder.
4. Launch the server so you can test immediately.

## Requirements

- Rust toolchain with the `wasm32-wasip2` target installed
- Internet access on first run (to download Pumpkin)

## Quick links

- [Getting Started](getting-started.md)
- [GitHub](https://github.com/XodiumSoftware/runpumpkin)
