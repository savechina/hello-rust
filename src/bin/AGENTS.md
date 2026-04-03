# Binary Executables

15 standalone binaries demonstrating 4 inter-process communication patterns (UDS, gRPC, stdio, process control).

## BINARY INVENTORY

| Pattern | Binaries | Purpose |
|---------|----------|---------|
| **CLI Examples** | `basic.rs`, `advance.rs` | Clap argument parsing demos |
| **Process Control** | `app_sys_ctl.rs`, `app_nix_ctl.rs` | PID file + signal handling |
| **gRPC Basic** | `grpc_hello_server/client.rs` | Simple tonic gRPC on :9001 |
| **gRPC Store** | `grpc_store_server/client.rs` | Inventory CRUD + streaming watch |
| **gRPC Production** | `greeter_server/client.rs` | Consul service discovery |
| **UDS** | `uds_server/client.rs`, `uds_parent.rs` | Unix Domain Socket IPC |
| **Stdio** | `stdio_parent.rs`, `stdio_child.rs` | Pipe-based parent/child |

## WHERE TO LOOK

| Task | Binaries | Notes |
|------|----------|-------|
| gRPC basics | `grpc_hello_*` | Minimal tonic server/client wrapper |
| gRPC production | `greeter_*` | Consul registry at 192.168.2.6:8500 |
| Unix sockets | `uds_*` | Custom 4-byte length-prefix protocol |
| Process spawning | `uds_parent`, `stdio_parent` | `Command` with `Stdio::piped()` |
| Signal handling | `app_nix_ctl` | SIGTERM → timeout → SIGKILL pattern |
| HTTP control | `app_sys_ctl` | Port 3600 with PID file management |

## CONVENTIONS

**Server/client pairs:** Each gRPC service has separate `*_server.rs` and `*_client.rs` binaries

**Process orchestrators:** `*_parent.rs` spawns child processes via `Command`

**Port assignments:**
- gRPC services: :9001 (basic), :50052 (production greeter)
- HTTP control: :3600 (app_sys_ctl)

**Socket paths:**
- UDS: `/tmp/hello.socket`

## ANTI-PATTERNS (THIS DIRECTORY)

**Blocking:**
- `thread::sleep()` in loops (15+ occurrences) — polling anti-pattern

**Hardcoded:**
- Ports and IP addresses not configurable via environment
- Socket paths hardcoded to `/tmp/`

**Error handling:**
- Multiple `.expect()` calls without proper error propagation

## IPC PROTOCOLS

**UDS:** Custom binary format — 4-byte big-endian length prefix + UTF-8 payload

**gRPC:** Standard tonic/prost with protobuf definitions in `crates/awesome/proto/`

**Stdio:** Line-delimited text over piped stdin/stdout

## RUNNING

```bash
# gRPC pairs (run server first)
cargo run --bin grpc_hello_server &
cargo run --bin grpc_hello_client

# UDS via orchestrator
cargo run --bin uds_parent

# Stdio parent/child
cargo run --bin stdio_parent
```