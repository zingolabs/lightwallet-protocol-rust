# lightwallet-protocol

Rust-specific bindings for the Zcash lightwallet protocol.

## Overview

This crate provides automatically generated Rust bindings derived from the language-agnostic Protocol Buffer definitions maintained at [zcash/lightwalletd](https://github.com/zcash/lightwalletd). These `.proto` files define the gRPC interface between Zcash light clients and `lightwalletd` servers.

## Features

- Complete Rust type definitions for all protocol messages
- gRPC client implementation for connecting to `lightwalletd` servers
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

// Connect to a lightwalletd server
let client = CompactTxStreamerClient::connect("http://localhost:9067").await?;
```

## Protocol Source

The `.proto` definitions in this repository are derived from the canonical protocol definitions maintained by the Zcash project. These define the standard interface for:

- Retrieving compact blocks and transactions
- Submitting transactions to the network
- Querying address balances and UTXOs
- Accessing mempool data
- Managing note commitment tree states

## Building

The protobuf code is generated automatically during the build process using `tonic-build`. The `build.rs` script compiles the `.proto` files into Rust code that is then included in the library.

## License

This project is distributed under the MIT license. See the original Zcash project for additional licensing information.