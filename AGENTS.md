# PROJECT KNOWLEDGE BASE

**Generated:** 2026-04-03
**Commit:** 3aa166b
**Branch:** main

## OVERVIEW

Rust learning resource with 3-crate workspace demonstrating basic→advanced→production patterns. 15 binaries showcase IPC (gRPC, UDS, stdio). Hybrid structure: root crate + workspace members.

## STRUCTURE

```
hello-rust/
├── src/
│   ├── main.rs           # Demo runner (calls basic/advance/leetcode examples)
│   ├── basic/            # Foundation: ownership, threads, generics, closures
│   ├── advance/          # Ecosystem: tokio, tonic, sqlx, diesel, rkyv, axum
│   ├── algo/             # Algorithms: linked list, pi calculation
│   └── bin/              # 15 IPC binaries (gRPC servers/clients, UDS, stdio)
├── crates/
│   ├── awesome/          # Production framework: DI, Consul, gRPC, databases
│   ├── leetcode/         # Algorithm solutions (2 problems)
│   └── macros/           # #[log] proc-macro for function tracing
└── docs/                 # mdBook documentation (Chinese, deployed to GH Pages)
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Learn basics | `src/basic/` | Ownership, threads, generics, closures - each topic in `*_sample.rs` |
| Framework examples | `src/advance/` | tokio, tonic, sqlx, diesel, rkyv, axum - each crate in `*_sample.rs` |
| IPC patterns | `src/bin/` | 15 binaries: gRPC pairs, UDS client/server, stdio parent/child |
| Production patterns | `crates/awesome/src/services/` | Service framework, DI patterns, Consul integration |
| LeetCode solutions | `crates/leetcode/src/` | `solution_0001.rs` (Two Sum), `solution_0002.rs` (Add Two Numbers) |
| Proc-macro example | `crates/macros/src/lib.rs` | Simple #[log] attribute for entry/exit tracing |
| Docs source | `docs/src/` | Chinese mdBook with getting-started, basic, advance, awesome sections |

## CONVENTIONS

**File naming:**
- `*_sample.rs` — Example/demonstration code
- `*_service.rs` — Production service implementations
- `solution_XXXX.rs` — LeetCode problem solutions (4-digit numbering)
- `app_*_ctl.rs` — Process control binaries

**Module organization:**
- Flat modules in `src/`: `basic.rs` (file) coexists with `basic/` (directory) — non-standard
- Workspace pattern: Root crate imports workspace members (`use leetcode;` in main.rs)
- No `lib.rs` in root crate — binary-only

**Edition:**
- Uses Rust 2024 edition (latest)
- Uses `extern crate` declarations (2015 style) alongside modern patterns

**Dependencies:**
- 60+ crates in root Cargo.toml
- No `[workspace.dependencies]` — each crate manages its own versions
- Heavy feature flag usage (serde/derive, tokio/full, sqlx multiple backends)

## ANTI-PATTERNS (THIS PROJECT)

**Safety:**
- `static mut VAL` in `src/basic/threads_sample.rs:243` — requires unsafe access
- `str::from_utf8_unchecked` in `src/basic/pointer_sample.rs:17` — no validation
- `std::mem::transmute` on function pointers in `crates/awesome/src/database/sqlite_vec_sample.rs:7`
- Commented deadlock example in `src/basic/threads_sample.rs:231-234` — "以下代码会阻塞发生死锁"

**Async:**
- `std::sync::Mutex` in async gRPC context (`crates/awesome/src/services/tonic_store_server.rs:34`) — use `tokio::sync::Mutex`

**Typos:**
- `dynmaic_injection_*.rs` files — should be "dynamic"

**Hardcoded:**
- IP addresses "192.168.2.6", "192.168.2.7" in greeter services — not configurable

**Blocking:**
- 15+ `thread::sleep()` calls in loops — polling anti-pattern

## UNIQUE STYLES

**Hybrid workspace:** Root defines `[package]` AND `[workspace]` together — unusual (typically virtual manifest OR package, not both)

**Binary demo runner:** `main.rs` calls library functions for demonstration, not a real application

**Inline tests:** No `tests/` directory — all tests in `#[cfg(test)]` modules within source files

**Chinese documentation:** Comments and docs in Chinese, English technical terms

**gRPC pairs:** Each service has separate client/server binaries (`grpc_hello_server.rs` + `grpc_hello_client.rs`)

**Dual Consul approaches:** `rs_consul` crate vs custom `ConsulClient` implementation

## COMMANDS

```bash
# Build workspace
cargo build

# Run main demo
cargo run

# Run specific binary
cargo run --bin grpc_hello_server
cargo run --bin uds_parent

# Test
cargo test

# Docs (local)
cd docs && mdbook serve
```

## NOTES

**Build requirements:**
- `protoc` 29.3 required (for `crates/awesome` tonic-build)
- CI installs via `arduino/setup-protoc@v2`

**Workspace quirks:**
- `macros` crate available but not directly used by `awesome` or `leetcode`
- `leetcode` has zero external dependencies (deliberately minimal)
- Root crate has no `lib.rs` — it's binary-only

**Framework location:**
- `crates/awesome/src/services/framework/` — custom service lifecycle manager
- `ApplicationFramework<S>` spawns services as tokio tasks
- `RunnableService` trait with graceful shutdown via oneshot channel

**Documentation:**
- mdBook with 3 plugins: admonish, alerts, pagetoc
- Deployed to `savechina.github.io/hello-rust/`
- Pre-built HTML in `docs/book/` is committed (unusual — typically gitignored)