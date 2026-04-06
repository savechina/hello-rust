# CLI Command Contract: hello

**Feature**: 002-interactive-examples  
**Date**: 2026-04-05  
**Version**: 1.0.0

## Command Schema

### Top-Level

```
hello [SUBCOMMAND] [OPTIONS]
```

| Subcommand | Description | Required Args |
|------------|-------------|---------------|
| *(none)* | Show top-level help | — |
| `list` | Show all categories and topics | — |
| `basic` | List or run basic topics | `[<topic>]` |
| `advance` | List or run advance topics | `[<topic>]` |
| `awesome` | List or run awesome topics | `[<topic>]` |
| `algo` | List or run algo topics | `[<topic>]` |

### Global Options

| Flag | Description |
|------|-------------|
| `--help`, `-h` | Show help message |
| `--version`, `-V` | Show version |

### Topic-Level Options

| Flag | Description |
|------|-------------|
| `--force` | Skip external dependency checks |
| `--no-docs` | Run sample without showing tutorial content |

## Response Formats

### Help Output (no subcommand)

```
Hello Rust — Interactive Learning CLI

Usage: hello [COMMAND]

Commands:
  list       Show all available topics
  basic      Rust fundamentals (ownership, generics, threads, ...)
  advance    Advanced patterns (async, databases, web, ...)
  awesome    Production-grade samples (gRPC, DI, templates, ...)
  algo       Algorithm implementations (pi calculation, ...)

Run `hello <command> --help` for more information on a command.
```

### Category List (e.g., `hello basic`)

```
Rust 基础 (Basic) — 选择主题运行: hello basic <topic>

TOPIC              DESCRIPTION                          DEPENDENCIES
─────────────────  ───────────────────────────────────   ──────────────
ownership          所有权规则、移动语义、借用与引用        无
generics           泛型函数、单态化、trait 约束            无
threads            线程创建、通道、Mutex、Arc              无
...
```

### Topic Run (e.g., `hello basic ownership`)

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📖 所有权 (Ownership)

所有权是 Rust 最独特的功能之一...

[教程内容...]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
▶ 运行样例:

[样例代码输出...]
```

### Error: Unknown Topic

```
Error: Unknown topic 'foo' in 'basic'.

Available topics:
  ownership      所有权规则、移动语义、借用与引用
  generics       泛型函数、单态化、trait 约束
  threads        线程创建、通道、Mutex、Arc
  ...
```

### Error: Missing Dependency

```
⚠ Warning: Topic 'grpc' requires external services: Consul, gRPC server.
   These services may not be available.

   Run with --force to skip this check.

▶ Running sample anyway...
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Unknown category or topic |
| 2 | Missing required external dependency (without --force) |
| 3 | Sample execution error (panic, compile error in sample) |
