# Rust Yew Tauri Template

A template webview desktop application made with Rust, using Yew and Tauri.

## Requirements

The only required tools besides `cargo` (obviously), are [`trunk`](https://trunkrs.dev/) and the [`tauri CLI`](https://tauri.studio/):

```sh
# Install trunk
$ cargo install --locked trunk

# Install the tauri CLI
$ cargo install tauri-cli --locked --version "^1.0.0-rc"

# Add the WASM target
$ rustup target add wasm32-unknown-unknown
```

## Build and Run

The tauri CLI makes building and running the project simple:

```sh
# Run in development mode, watching for file changes
$ cargo tauri dev

# Build for distribution
$ cargo tauri build
```
