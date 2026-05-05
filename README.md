# lightwallet-protocol

Rust-specific bindings for the Zcash lightwallet protocol.

## Overview

This crate provides automatically generated Rust bindings derived from the language-agnostic Protocol Buffer definitions maintained at [zcash/lightwallet-protocol](https://github.com/zcash/lightwallet-protocol). These `.proto` files define the gRPC interface between Zcash light clients and `indexer` servers.

## Features

- Complete Rust type definitions for all protocol messages
- gRPC client implementation for connecting to `indexer` servers
- gRPC server traits for implementing custom lightwallet servers
- Automatic code generation from `.proto` files via `tonic-build`

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
lightwallet-protocol = "0.1.0"
```

Then use the generated types in your code:

```rust
use lightwallet_protocol::{BlockId, CompactBlock, RawTransaction};
use lightwallet_protocol::CompactTxStreamerClient;

// Connect to an indexer server
let client = CompactTxStreamerClient::connect("http://localhost:9067").await?;
```

## Protocol Source

The `.proto` definitions in `lightwallet-protocol/` are tracked via `git subtree` from the canonical upstream:

> [zcash/lightwallet-protocol](https://github.com/zcash/lightwallet-protocol)

The current pinned version is **v0.4.1**.

### Updating to a new upstream release

```sh
make update-proto UPSTREAM_VERSION=v0.4.2
```

If the new release changes any `.proto` files, regenerate the Rust bindings:

```sh
make rebuild-proto
```

Then commit both the subtree merge commit and any changes to `src/generated/`.

## Building

The pre-generated Rust bindings in `src/generated/` are committed to the repository, so a normal `cargo build` requires no extra tools. To regenerate from the `.proto` files (requires `protoc`):

```sh
cargo build --features rebuild-proto
```

## License

This project is distributed under the MIT license. See the original Zcash project for additional licensing information.
