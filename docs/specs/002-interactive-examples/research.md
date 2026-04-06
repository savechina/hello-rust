# Research: Compile-Time Sample Registration

**Feature**: 002-interactive-examples  
**Date**: 2026-04-05

## Decision 1: Sample Registration Mechanism

**Decision**: Use `inventory` crate for compile-time automatic registration of sample functions.

**Rationale**:
- Minimal boilerplate: 3 lines per sample (`inventory::submit! { ... }`)
- Works across workspace crates (root crate + `crates/awesome/`)
- Type-safe, compile-time checked
- Zero runtime overhead — iterator over static items in the binary
- Production-proven in major Rust projects (cargo-expand, strum, etc.)
- Extensible for future proc-macro documentation generation
- Learner-friendly: adding a new sample is declarative, not structural

**Alternatives considered**:

| Approach | Boilerplate | Cross-crate | Complexity | Verdict |
|----------|-------------|-------------|------------|---------|
| **`inventory` crate** | 3 lines | ✅ Yes | Low | ✅ **Selected** |
| `linkme` crate | 6 lines | ✅ Yes | Medium | Rejected — overkill for educational tool |
| `build.rs` code gen | 0 lines | ✅ Yes | High | Rejected — build complexity, harder to debug |
| Manual routing table | N lines per topic | N/A | Low | Rejected — violates FR-013 |
| Proc-macro attribute | 1 line (`#[register]`) | ✅ Yes | Medium | Future enhancement — inventory is simpler for v1 |

## Decision 2: Tutorial Content Embedding

**Decision**: Use `include_dir!` macro (already in Cargo.toml) to embed `docs/src/` markdown at compile time.

**Rationale**:
- Already a project dependency — no new crate to add
- Zero runtime I/O — content is in the binary
- Works reliably in `cargo run` contexts (no file path assumptions)
- Supports glob patterns for selective embedding

**Alternatives considered**:
- Runtime file reads — rejected due to path fragility and latency
- `include_str!` per file — rejected due to manual maintenance burden

## Decision 3: CLI Framework

**Decision**: Use `clap` with derive macros (already in Cargo.toml).

**Rationale**:
- Already present with `derive` feature enabled
- Provides `--help`, subcommand validation, error messages out of the box
- Familiar to Rust developers — matches project's existing usage in `src/bin/advance.rs`

## Decision 4: Async Sample Handling

**Decision**: Async samples will be wrapped in `tokio::runtime::Runtime::new().unwrap().block_on(async_fn)`.

**Rationale**:
- Tokio is already a dependency with `full` features
- The CLI itself is synchronous; async samples need explicit runtime creation
- Matches existing pattern in `src/advance/tokio_sample.rs`

## Sample Inventory

From codebase exploration, the following samples are available for registration:

### basic (13 files, ~35 entry functions)
expression, ownership, datatype, pointer, generic, closure, threads, module, logger, cfg_if, traits, visiable, tracing

### advance (15 files with public entry points, ~50 functions)
bytes, futures, json, memmap, tempfile, tokio, axum, sqlx, diesel, rkyv, process, sysinfo, ollama, includedir, csv

### algo (1 file, 6 functions)
calc_pi (leibniz, functional, parallel, bbp, bigfloat, bigdecimal)

### awesome (14 files across 6 subdirectories)
services (6), database (2), templates (3), mq (1), data (1), sequences (1)

### Standalone binaries (15 files — listed, not subcommand-runnable)
gRPC servers/clients, UDS, stdio, process control — these are long-running or multi-process and will be listed with "requires separate terminal" annotation.
