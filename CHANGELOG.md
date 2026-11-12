# Changelog
All notable changes to the `lightwallet-protocol` crate will be documented in
this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this crate adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file records changes to the Rust crate. The changes to the protocol
definitions themselves are recorded upstream, in
[lightwallet-protocol/CHANGELOG.md](lightwallet-protocol/CHANGELOG.md). That
file arrives through `git subtree` and must not be edited here.

## [Unreleased]

## [0.5.0] - 2026-11-11

### Added
- `service.CompactTxStreamer` has added rpc `GetStandardFee`, returning the
  new `service.StandardFee` message (`feeZat`, `estimatorVersion`): the ZIP
  317 marginal fee in zatoshis per logical action and the backend
  fee-estimator version. Servers whose backend lacks the `getstandardfee`
  JSON-RPC return `Unimplemented`. New since the `0.4.0` release of this
  crate; the subtree has been updated from upstream `v0.5.0` to
  [v0.6.0](https://github.com/zcash/lightwallet-protocol/releases/tag/v0.6.0).

### Changed
- **Breaking for server implementors:** the generated
  `compact_tx_streamer_server::CompactTxStreamer` trait has a new required
  method, `get_standard_fee`, with no default body. Every implementation of
  the trait must add the method before it compiles against this release. An
  implementation whose backend lacks the `getstandardfee` JSON-RPC returns
  `tonic::Status::unimplemented`. Users of `CompactTxStreamerClient` are
  unaffected.
