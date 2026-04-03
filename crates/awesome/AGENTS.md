# Awesome Crate

Production framework demonstrations covering microservices, databases, messaging, templates, and data processing.

## STRUCTURE

```
crates/awesome/src/
├── services/         # gRPC services, DI patterns, custom framework
│   └── framework/    # ApplicationFramework with lifecycle management
├── database/         # SQLite (with vec extension), SurrealDB
├── mq/               # MQTT client (rumqttc)
├── templates/        # Tera, Liquid, Pest parser
├── sequences/        # UUID generation
└── data/             # Polars data frames
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Service framework | `services/framework/lifecycle.rs` | `ApplicationFramework<S>` + `RunnableService` trait |
| Consul integration | `services/framework/registry.rs` | Custom HTTP client for service discovery |
| DI patterns | `services/dynmaic_injection_*.rs` | Arc vs Box approaches (typo in filenames) |
| Plugin system | `services/inventory_sample.rs` | Compile-time registration with `inventory` crate |
| gRPC production | `services/greeter_service.rs` | Dual HTTP+gRPC servers with Consul registry |
| Service discovery | `services/greeter_consume.rs` | Round-robin client with caching |

## CONVENTIONS

**File naming:**
- `*_sample.rs` — Demonstration patterns (DI, Consul usage)
- `*_service.rs` — Production service implementations
- `dynmaic_injection_*.rs` — Typo ("dynamic" misspelled)

**Module organization:**
- Flat declarations in `services/mod.rs` (13 modules at same level)
- Proto modules embedded in service files via `tonic::include_proto!`

**Dependencies:**
- 43 production crates (most in workspace)
- `tonic-build` in `[build-dependencies]` for protobuf compilation
- Build requires `protoc` installed

## ANTI-PATTERNS (THIS CRATE)

**Async:**
- `std::sync::Mutex` in async gRPC context (`services/tonic_store_server.rs:34`) — use `tokio::sync::Mutex`

**Hardcoded:**
- IP addresses "192.168.2.6", "192.168.2.7" in greeter services — not configurable
- Socket path `/tmp/hello.socket` in UDS binaries

**Typos:**
- `dynmaic_injection_*.rs` files — should be "dynamic"

**Inconsistency:**
- Two Consul approaches: `rs_consul` crate (`consul_sample.rs`) vs custom client (`framework/registry.rs`)

## UNIQUE STYLES

**ApplicationFramework pattern:** Generic lifecycle manager spawning services as tokio tasks with status tracking and graceful shutdown via oneshot channels

**Service Locator:** TypeId-based HashMap for type-safe service resolution across three DI implementations

**Dual Consul:** Both `rs_consul` crate and custom HTTP client implementation exist

**Inventory plugin:** Compile-time plugin registration using `inventory::submit!` macro (requires linker magic)

## BUILD

```bash
# Requires protoc 29.3
cargo build -p awesome
```