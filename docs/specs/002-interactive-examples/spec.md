# Feature Specification: Interactive CLI Subcommands

**Feature Branch**: `002-interactive-examples`  
**Created**: 2026-04-05  
**Status**: Draft  
**Input**: User description: 将 basic advance ,awesome 等工程样例代码，修改为可以通过hello 的子命令，可以调用运行样例代码，这样有教程也有样例实现运行直接看效果，类似 rustling 一样可与代码交互进行学习

## Clarifications

### Session 2026-04-05

- Q: `hello basic <topic>` topic 如何知道所有主题样例代码？→ A: 主题名称 + 一句话描述 + 外部依赖标注
- Q: 类似 basic 增加 xxx_sample 方法，如何动态发现？→ A: 编译时自动注册 — 通过宏或代码生成在编译期收集所有 xxx_sample 函数，新增方法只需在注册表中声明

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Run Individual Sample via Subcommand (Priority: P1)

A learner wants to run a single specific sample (e.g. ownership, closures, async) without running the entire demo suite. They invoke `hello basic ownership` and see the sample output immediately.

**Why this priority**: This is the core value proposition — learners should be able to pick any topic and see its output on demand, rather than scrolling through all examples at once.

**Independent Test**: Can be fully tested by running `hello basic <topic>` and verifying the correct sample code executes and prints expected output.

**Acceptance Scenarios**:

1. **Given** the user has built the project, **When** they run `hello basic ownership`, **Then** only the ownership sample code executes and its output is displayed.
2. **Given** the user runs `hello basic` with no sub-topic, **Then** a list of all available basic topics is shown with brief descriptions.
3. **Given** the user runs `hello advance async`, **Then** only the async programming sample executes.

---

### User Story 2 - Interactive Tutorial Experience (Priority: P2)

A learner runs a sample and wants to understand what the code does. The CLI displays the relevant tutorial documentation alongside or before the sample output, similar to how Rustlings shows exercise descriptions before code.

**Why this priority**: The goal is not just to run code but to learn — pairing tutorial text with runnable examples creates the Rustlings-like experience.

**Independent Test**: Can be fully tested by running any sample subcommand and verifying that tutorial content (from existing docs) is displayed along with the sample output.

**Acceptance Scenarios**:

1. **Given** the user runs `hello basic ownership --help`, **Then** the tutorial description for ownership is displayed along with usage instructions.
2. **Given** the user runs `hello basic ownership`, **Then** the tutorial introduction is shown first, followed by the sample code output.
3. **Given** the user runs `hello list`, **Then** a categorized menu of all available samples across basic, advance, and awesome is displayed.

---

### User Story 3 - Unified Entry Point for All Examples (Priority: P3)

A learner wants a single `hello` command to access all example categories — basic, advance, awesome, algo, and the 15 IPC binaries — without needing to remember different `cargo run --bin` invocations.

**Why this priority**: Consolidates the fragmented entry points (main.rs, 15 bin stubs, workspace crates) into one coherent CLI surface.

**Independent Test**: Can be fully tested by running `hello <category> <topic>` for each category and verifying the correct sample executes.

**Acceptance Scenarios**:

1. **Given** the user runs `hello awesome grpc`, **Then** the gRPC sample from the awesome crate executes.
2. **Given** the user runs `hello algo linked-list`, **Then** the linked list algorithm sample executes.
3. **Given** the user runs `hello` with no arguments, **Then** a top-level help menu shows all categories and how to explore each.

---

### Edge Cases

- What happens when a user requests a topic that does not exist? The CLI should display a clear error message and suggest similar topics.
- How does the system handle samples that require external dependencies (e.g., database connections, Consul, network ports)? The CLI should display prerequisite warnings and skip gracefully if dependencies are unavailable.
- What happens when a sample requires async runtime but the subcommand is synchronous? The CLI must properly initialize the async runtime before executing async samples.
- How are long-running samples (e.g., servers that block indefinitely) handled? The CLI should provide a timeout or signal-based exit mechanism.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a single `hello` binary as the unified entry point for all sample code execution.
- **FR-002**: System MUST support `hello basic <topic>` subcommand to run individual basic topic samples (e.g., ownership, closures, generics, threads).
- **FR-003**: System MUST support `hello advance <topic>` subcommand to run individual advance topic samples (e.g., async, smart-pointers, error-handling, database, web).
- **FR-004**: System MUST support `hello awesome <topic>` subcommand to run individual awesome crate samples (e.g., gRPC, DI, Consul, message queues).
- **FR-005**: System MUST support `hello algo <topic>` subcommand to run algorithm samples (e.g., linked-list, pi-calculation).
- **FR-006**: System MUST display available topics when a category is invoked without a specific topic (e.g., `hello basic` lists all basic topics). Each topic listing MUST include the topic name, a one-line description, and an external dependency annotation (e.g., "需要 Consul" or "无外部依赖").
- **FR-007**: System MUST display tutorial documentation (from existing docs/src content) before or alongside sample output when running a topic.
- **FR-008**: System MUST provide `hello list` to show all available categories and topics in a structured overview. Each topic entry MUST include the topic name, a one-line description, and an external dependency annotation.
- **FR-009**: System MUST display a helpful error message with available topic suggestions when an invalid topic name is provided.
- **FR-010**: System MUST handle samples requiring external dependencies (databases, network services) by displaying prerequisite warnings and allowing graceful skip.
- **FR-011**: System MUST properly initialize async runtime before executing async samples.
- **FR-012**: System MUST support `--help` flag on every subcommand level showing topic descriptions and usage.
- **FR-013**: System MUST use compile-time automatic registration to discover all sample functions — new `xxx_sample` methods are collected at compile time via macros or code generation, requiring only a registry declaration to be discoverable.

### Key Entities

- **Category**: A top-level grouping of samples (basic, advance, awesome, algo). Each category has a name, description, and list of topics.
- **Topic**: A specific learnable unit within a category (e.g., "ownership", "async", "grpc"). Each topic has a name, a one-line description, associated sample code, optional external dependencies, and a dependency status annotation.
- **Sample**: The executable code associated with a topic. A sample may be a function call from an existing module or a standalone binary invocation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Learners can run any individual sample topic in a single command and see output within 3 seconds of command execution.
- **SC-002**: 100% of existing basic, advance, and awesome sample topics are accessible via the new subcommand interface.
- **SC-003**: Users can discover all available topics via `hello list` or category-level listing without consulting external documentation.
- **SC-004**: Tutorial content is displayed alongside sample output for every topic run, providing a combined read-and-execute learning experience.
- **SC-005**: Invalid topic names produce helpful error messages that include a list of valid topics for the given category.

## Assumptions

- The existing `clap` dependency (already in Cargo.toml with derive feature) will be used for CLI argument parsing.
- Target users are Chinese-speaking developers learning Rust (consistent with existing documentation language).
- The existing `docs/src/` markdown content will serve as the tutorial text source — no new documentation needs to be authored from scratch.
- Samples requiring external services (Consul, databases, gRPC servers) will display a prerequisite warning but remain runnable if the service is available.
- The `hello` binary name maps to the existing `hello-rust` package (set via `[[bin]]` entry in Cargo.toml).
- The existing module structure (`src/basic/`, `src/advance/`, `crates/awesome/`) will be reused — sample code itself does not need to be rewritten, only the invocation mechanism changes.
- Long-running server samples (e.g., gRPC servers) are out of scope for the interactive tutorial mode — they will be listed but marked as "server" type requiring separate terminal.
