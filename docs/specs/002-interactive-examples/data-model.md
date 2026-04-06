# Data Model: Interactive CLI Registry

**Feature**: 002-interactive-examples  
**Date**: 2026-04-05

## Core Entities

### Topic

The fundamental unit of learning. Each topic maps to a runnable sample function.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `&'static str` | CLI subcommand identifier (kebab-case, e.g. `"ownership"`, `"smart-pointers"`) |
| `description` | `&'static str` | One-line Chinese description with English technical terms |
| `category` | `&'static str` | Parent category: `"basic"`, `"advance"`, `"awesome"`, `"algo"` |
| `func` | `fn()` | Zero-argument function pointer to execute the sample |
| `dependencies` | `&'static [&'static str]` | External services required (empty slice if none). Valid values: `"Consul"`, `"MySQL"`, `"SQLite"`, `"MQTT"`, `"Ollama"`, `"gRPC"` |
| `doc_path` | `Option<&'static str>` | Path to embedded markdown tutorial (e.g. `"basic/ownership.md"`) |
| `sample_type` | `SampleType` | `Function` (calls existing fn) or `Server` (long-running, requires separate terminal) |

### Category

Top-level grouping of topics.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `&'static str` | CLI subcommand identifier (e.g. `"basic"`) |
| `description` | `&'static str` | Category description in Chinese |
| `topics` | Computed | Derived from `inventory::iter::<Topic>` filtered by `category` field |

### SampleRegistry

Static collection of all registered topics.

| Field | Type | Description |
|-------|------|-------------|
| `topics` | `inventory::iter::<Topic>` | Compile-time collected iterator |
| `categories` | Computed | Derived unique category names from topics |

### SampleType (Enum)

| Variant | Description |
|---------|-------------|
| `Function` | Synchronous or async function that runs and returns |
| `Server` | Long-running server process (listed but marked as requiring separate terminal) |

## Validation Rules

1. **Uniqueness**: `(category, name)` tuples must be unique across all registered topics
2. **Non-empty description**: Every topic must have a non-empty description string
3. **Valid dependencies**: Each dependency string must match a known external service
4. **Doc path consistency**: If `doc_path` is `Some(path)`, the file must exist in `docs/src/`
5. **Function signature**: All sample functions must be `fn()` (zero arguments, no return)

## State Transitions

N/A — all entities are static, compile-time defined.

## Relationships

```
Category (1) ──→ (*) Topic
                    ├── func() ──→ Sample code (existing module)
                    └── doc_path ──→ Markdown file (docs/src/)
```
