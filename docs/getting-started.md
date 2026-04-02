# Getting Started

## Installation

```bash
cargo install runpumpkin
```

### Building from source

Clone and build manually:

```bash
git clone https://github.com/XodiumSoftware/runpumpkin
cd runpumpkin
cargo build --release
# binary is at target/release/runpumpkin
```

## Usage

Run `runpumpkin` from the root of your Pumpkin plugin workspace:

```bash
runpumpkin
```

This will:

1. Create a `.pumpkin-server/` directory in the current folder.
2. Download the latest Pumpkin server binary into your OS cache (only on first run or when a new version is released).
3. Build your plugin with `cargo build --release --target wasm32-wasip2`.
4. Copy the resulting `.wasm` into `.pumpkin-server/plugins/`.
5. Start the Pumpkin server.

## Prerequisites

Your plugin workspace must be a valid Cargo project that compiles for `wasm32-wasip2`. Add the target if you haven't already:

```bash
rustup target add wasm32-wasip2
```
