<img src=".github/comet-eventbus.svg" alt="comet-eventbus" />

[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)](
https://github.com/lightsing/comet-eventbus#license)
[![crates.io](https://img.shields.io/crates/v/comet-eventbus.svg)](
https://crates.io/crates/comet-eventbus)
[![docs.rs](https://img.shields.io/badge/docs-docs.rs-green)](https://docs.rs/comet-eventbus/)
[![Documentation](https://img.shields.io/badge/docs-latest-green)](
https://lightsing.github.io/comet-eventbus/comet_eventbus/index.html)

A strong typed sync and asynchronous eventbus implementation.

Also provide grpc eventbus bridge for asynchronous implementation.

### Notice: This crate is under highly active development. I won't recommend you to use before the api becomes stable.

## Get Started

### Async Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
comet-eventbus = "0.1.0-pre-alpha.3"
```

### Sync Usage
Add this to your `Cargo.toml`:
```toml
[dependencies.comet-eventbus]
version = "0.1.0-pre-alpha.3"
features = ["sync", "sync_parallel"]
default-features = false
```

## Example

checkout examples in [`examples`](examples)