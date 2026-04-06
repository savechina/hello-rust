# Implementation Plan: Interactive CLI Subcommands

**Branch**: `002-interactive-examples` | **Date**: 2026-04-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/docs/specs/002-interactive-examples/spec.md`

## Summary

Transform the existing hello-rust project into an interactive Rustlings-like CLI learning experience. Replace the monolithic `main.rs` demo runner with a unified `hello` binary using `clap` subcommands (`hello basic <topic>`, `hello advance <topic>`, etc.). Use the `inventory` crate for compile-time automatic sample registration — new samples require only a 3-line `inventory::submit!` declaration to become discoverable. Tutorial content from existing `docs/src/` markdown files will be embedded at compile time and displayed alongside sample output.

## Technical Context

**Language/Version**: Rust 2024 Edition  
**Primary Dependencies**: `clap` 4.5+ (derive, already present), `inventory` (new — compile-time registration), `include_dir` (already present — embed docs)  
**Storage**: N/A (in-memory, compile-time embedded)  
**Testing**: `cargo test` (existing inline test pattern)  
**Target Platform**: macOS, Linux (development machines)  
**Project Type**: CLI tool with embedded tutorial content  
**Performance Goals**: CLI startup <50ms cold, <10ms warm (per Constitution IV); sample execution within 3s  
**Constraints**: <100MB memory footprint; no `unsafe` without documented invariants; `cargo clippy -- -D warnings` must pass  
**Scale/Scope**: ~50 sample functions across 4 categories (basic, advance, awesome, algo), 15 standalone IPC binaries

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| **I. Code Quality** | ✅ PASS | Rust 2024 idioms enforced; `inventory` is safe (no `unsafe`); all new public APIs will have `///` doc comments |
| **II. Test-First** | ✅ PASS | All sample registration and CLI routing will have unit tests; existing inline tests preserved |
| **III. UX Consistency** | ✅ PASS | `clap` for consistent CLI; Chinese docs with English technical terms (existing convention); helpful error messages with topic suggestions |
| **IV. Performance** | ✅ PASS | CLI startup <50ms (clap derive is fast); compile-time registration has zero runtime overhead; embedded docs via `include_dir` |
| **V. SDD Harness** | ✅ PASS | Following 8-phase workflow; this is Phase 1 planning output |

**No violations detected.** Proceeding to Phase 0 research (already completed via background agents).

## Project Structure

### Documentation (this feature)

```text
docs/specs/002-interactive-examples/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── main.rs                  # REPLACED: New hello CLI entry point with clap subcommands
├── cli/
│   ├── mod.rs               # CLI module: Commands enum, topic registry
│   ├── registry.rs          # Sample registry using `inventory` crate
│   └── docs_embed.rs        # Embedded tutorial content via `include_dir!`
├── basic/                   # EXISTING: Unchanged sample code
│   ├── expression_sample.rs
│   ├── ownership_sample.rs
│   ├── datatype_sample.rs
│   ├── ... (13 files, 35 entry functions)
│   └── mod.rs               # UPDATED: Expose pub functions + inventory::submit!
├── advance/                 # EXISTING: Unchanged sample code
│   ├── bytes_sample.rs
│   ├── futures_sample.rs
│   ├── ... (15 files with public entry points)
│   └── mod.rs               # UPDATED: Expose pub functions + inventory::submit!
├── algo/                    # EXISTING: Unchanged sample code
│   ├── calc_pi_sample.rs
│   └── mod.rs               # UPDATED: Expose pub functions + inventory::submit!
└── bin/                     # EXISTING: 15 IPC binaries (unchanged, listed via CLI)

crates/awesome/src/          # EXISTING: Unchanged sample code
├── services/                # (6 sample files — need pub wrapper functions added)
├── database/                # (2 sample files)
├── templates/               # (3 sample files)
├── mq/                      # (1 sample file)
├── data/                    # (1 sample file)
├── sequences/               # (1 sample file)
└── lib.rs                   # UPDATED: Re-export samples + inventory::submit!

Cargo.toml                   # UPDATED: Add `inventory` dependency, rename binary to `hello`
```

**Structure Decision**: Single-project CLI with existing module layout. The `inventory` crate enables cross-module registration without restructuring. The `main.rs` is replaced with a clap-based CLI dispatcher. Sample code files remain untouched — only `mod.rs` files gain `inventory::submit!` declarations and public function exposure.

## Complexity Tracking

> No constitution violations. Complexity is justified by the `inventory` crate choice:

| Decision | Why Needed | Simpler Alternative Rejected Because |
|----------|------------|-------------------------------------|
| `inventory` crate | Enables zero-boilerplate sample discovery across workspace crates | Manual routing table requires updating CLI code for every new sample — violates FR-013 (compile-time auto-registration) |
| `include_dir!` for docs | Embeds tutorial markdown at compile time, no runtime file I/O | Runtime file reads would break in `cargo run` contexts and add latency |

## Phase 0: Research Findings

**research.md** — Compile-time sample registration patterns

### Decision: Use `inventory` crate for sample registration

**Rationale**:
- Minimal boilerplate: 3 lines per sample (`inventory::submit! { ... }`)
- Works across workspace crates (root + `crates/awesome/`)
- Type-safe, compile-time checked
- Zero runtime overhead — iterator over static items
- Production-proven in major Rust projects
- Extensible for future proc-macro documentation generation

**Alternatives considered**:
1. **`linkme` crate** — Linker-distributed slices. More complex (6 lines/sample), better for embedded/zero-cost scenarios. Overkill for educational tool.
2. **`build.rs` code generation** — Zero boilerplate but adds build-time complexity, harder to debug, less transparent for learners.
3. **Manual routing table** — Simple but violates FR-013. Every new sample requires CLI code changes.

### Decision: Use `include_dir!` for embedded tutorial content

**Rationale**: Already a dependency in Cargo.toml. Compiles `docs/src/` markdown into the binary. No runtime file I/O overhead.

### Decision: Clap derive for CLI structure

**Rationale**: Already in Cargo.toml with derive feature. Provides `--help`, subcommand validation, and error messages out of the box.

## Phase 1: Design & Contracts

### Data Model

**data-model.md** — CLI registry entities

| Entity | Fields | Description |
|--------|--------|-------------|
| **Category** | `name: &'static str`, `description: &'static str`, `topics: Vec<TopicRef>` | Top-level grouping (basic, advance, awesome, algo) |
| **Topic** | `name: &'static str`, `description: &'static str`, `category: &'static str`, `func: fn()`, `dependencies: &'static [&'static str]` | Individual learnable unit with sample function pointer |
| **SampleRegistry** | `inventory::iter<Topic>` | Compile-time collected iterator over all registered topics |
| **DocContent** | `path: &'static str`, `content: &'static str` | Embedded markdown from `docs/src/` via `include_dir!` |

**Validation rules** (from spec requirements):
- Topic names must be unique within a category
- Every category must have at least one topic
- Topic descriptions must be non-empty
- Dependency annotations must match known external services (Consul, MySQL, SQLite, MQTT, Ollama)

### CLI Command Contracts

**contracts/cli-schema.md** — Command interface specification

```
hello                          → Show top-level help with all categories
hello list                     → Show all categories and topics with descriptions + dependency annotations
hello basic                    → List all basic topics with descriptions
hello basic <topic>            → Run specific basic sample + show tutorial content
hello advance                  → List all advance topics
hello advance <topic>          → Run specific advance sample + show tutorial content
hello awesome                  → List all awesome topics
hello awesome <topic>          → Run specific awesome sample + show tutorial content
hello algo                     → List all algo topics
hello algo <topic>             → Run specific algo sample + show tutorial content
hello <category> <topic> --help → Show topic description + usage instructions
```

**Error behavior**:
- Unknown category → `"Unknown category 'X'. Available: basic, advance, awesome, algo"`
- Unknown topic → `"Unknown topic 'X' in 'basic'. Available: ownership, generics, ..."`
- Missing dependency → `"⚠ Topic 'grpc' requires Consul. Run with --force to skip check."`

### Quickstart

**quickstart.md** — Developer guide for adding new samples

```markdown
# Adding a New Sample

## 1. Create the sample file

```rust
// src/basic/my_topic_sample.rs
pub fn my_topic_sample() {
    println!("Hello from my topic!");
}
```

## 2. Register in mod.rs

```rust
// src/basic/mod.rs
pub mod my_topic_sample;

inventory::submit! {
    crate::cli::registry::Topic {
        name: "my-topic",
        description: "Demonstrates my topic concept",
        category: "basic",
        func: my_topic_sample::my_topic_sample,
        dependencies: &[],  // or &["Consul"] if external service needed
    }
}
```

## 3. (Optional) Add tutorial content

Create `docs/src/basic/my-topic.md` with your tutorial. It will be embedded automatically.

## 4. Verify

```bash
cargo build
cargo run -- hello basic my-topic
```
```
