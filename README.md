# mooxide-core

A modern Rust reimplementation of [MOOS](https://oceanai.mit.edu/moos-ivp/pmwiki/pmwiki.php?n=Main.HomePageFull)
(Mission Orientated Operating Suite) — the publish/subscribe middleware for robotics —
wire-protocol compatible with the original C++ implementation.

**Status: early skeleton.** The workspace layout, CI, testing, and FFI scaffolding are
in place; protocol work has not started yet.

## Goals

- **Wire-protocol compatibility**: `mooxide-db` accepts connections from original
  C++ MOOS clients, and `mooxide-comms` clients talk to an original `MOOSDB`.
- **Modern Rust**: tokio-based async networking, `tracing` logging, `thiserror`/`anyhow`
  error handling.
- **Cross-platform**: Linux and macOS, first-class.

## Workspace layout

| Crate | Kind | Purpose |
|---|---|---|
| `mooxide-proto` | lib | Wire-protocol message types, encode/decode, `.moos` mission-file parsing |
| `mooxide-comms` | lib | Async client library (counterpart of MOOSLib) |
| `mooxide-db` | bin | The server, `mooxide-db` (counterpart of MOOSDB) |
| `mooxide-apps` | bins | `mooxide-antler` (process launcher), `mooxide-logger` |
| `mooxide-ffi` | cdylib/staticlib | C ABI bindings; header at `include/mooxide/mooxide.h` |
| `mooxide-log` | lib | Shared logging init with pluggable backends (stdout / file / database) |
| `mooxide-test` | lib | In-memory mock server/client and message replay for tests |
| `xtask` | bin | Task runner (`cargo xtask lint \| test \| coverage \| ffi-header`) |

## Development

```sh
cargo build --workspace
cargo test --workspace
cargo xtask lint        # fmt + clippy + cargo-deny + typos
cargo xtask ffi-header  # regenerate include/mooxide/mooxide.h
```

Testing layers: unit tests (codec round-trips), integration tests (real TCP against a
spawned `mooxide-db`), and interoperability tests against recorded wire transcripts from
the original implementation (fixtures; live interop is verified manually).

## License

[GPL-3.0-or-later](LICENSE), with library crates additionally available under
[LGPL-3.0-or-later](COPYING). © yuhaoxin
