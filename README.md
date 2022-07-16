<img src=".github/comet-eventbus.svg" alt="comet-eventbus" />

[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue.svg)](
https://github.com/lightsing/comet-eventbus#license)
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
comet-eventbus = { git = "https://github.com/lightsing/comet-eventbus.git" }
```

### Sync Usage
Add this to your `Cargo.toml`:
```toml
[dependencies.comet-eventbus]
git = "https://github.com/lightsing/comet-eventbus.git"
features = ["sync", "sync_parallel"]
default-features = false
```

## Example

checkout examples in [`examples`](examples)