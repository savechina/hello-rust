---

description: "Task list for Interactive CLI Subcommands implementation"
---

# Tasks: Interactive CLI Subcommands

**Input**: Design documents from `/docs/specs/002-interactive-examples/`
**Prerequisites**: plan.md, spec.md, data-model.md, research.md, contracts/cli-schema.md

**Tests**: Tests are OPTIONAL for this feature - omitted per spec (focus on runnable samples, not TDD)

**Organization**: Tasks grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: User story label (US1, US2, US3)
- File paths are absolute from repository root

## Path Conventions

- Single project: `src/`, `Cargo.toml` at repository root
- Workspace crate: `crates/awesome/src/`
- Documentation: `docs/src/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and CLI framework setup

- [x] T001 Add `inventory = "0.3"` dependency to `Cargo.toml` (root crate)
- [x] T002 [P] Create `src/cli/` directory structure
- [x] T002 [P] Create `src/cli/mod.rs` with `Commands` enum (basic, advance, awesome, algo, list)
- [x] T003 [P] Create `src/cli/registry.rs` with `Topic` struct and `SampleType` enum
- [x] T004 [P] Create `src/cli/docs_embed.rs` using `include_dir!` for `docs/src/` content

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core registry and CLI dispatcher that MUST be complete before ANY user story

**⚠️ CRITICAL**: No sample registration work can begin until this phase is complete

- [x] T005 Implement `Topic` struct with fields: `name`, `description`, `category`, `func`, `dependencies`, `doc_path`, `sample_type`
- [x] T006 Implement `SampleRegistry` using `inventory::iter::<Topic>` for compile-time collection
- [x] T007 Implement `list_all_topics()` function that iterates registry and displays formatted output
- [x] T008 Implement CLI dispatcher in `src/cli/mod.rs` with clap derive macros
- [x] T009 [P] Add `--help` flag support at all command levels (top-level, category, topic)
- [x] T010 [P] Add `--force` and `--no-docs` options for topic execution
- [x] T011 Implement error handling with helpful suggestions for unknown categories/topics
- [x] T012 Implement dependency check system for external services (Consul, MySQL, gRPC, etc.)

**Checkpoint**: Foundation ready - sample registration can now begin in parallel across categories

---

## Phase 3: User Story 1 - Run Individual Sample via Subcommand (P1) 🎯 MVP

**Goal**: Users can run any individual sample from `basic`, `advance`, or `algo` categories via `hello <category> <topic>`

**Independent Test**: Can run `hello basic ownership`, `hello advance async`, `hello algo calc-pi` and see correct sample output

### Implementation for User Story 1

#### basic category registration

- [x] T013 [P] [US1] Create `src/basic/mod.rs` declaring all 15 modules (expression, ownership, datatype, pointer, generic, closure, threads, module, logger, cfg_if, traits, visiable, tracing, rectangle, lifetimes)
- [ ] T014 [P] [US1] Add `pub` visibility to all sample functions in `src/basic/expression_sample.rs`
- [ ] T015 [P] [US1] Add `pub` visibility to all sample functions in `src/basic/ownership_sample.rs`
- [ ] T016 [P] [US1] Add `pub` visibility to all sample functions in `src/basic/datatype_sample.rs`
- [ ] T017 [P] [US1] Add `pub` visibility to all sample functions in `src/basic/generic_sample.rs`
- [ ] T018 [P] [US1] Add `pub` visibility to all sample functions in `src/basic/closure_sample.rs`
- [ ] T019 [P] [US1] Add `pub` visibility to all sample functions in `src/basic/threads_sample.rs`
- [ ] T020 [US1] Add `inventory::submit!` registration in `src/basic/mod.rs` for all basic topics (35+ functions)
- [ ] T021 [US1] Map each topic to its tutorial doc path (e.g., `"basic/ownership.md"`)

#### advance category registration

- [ ] T022 [P] [US1] Change all `mod` declarations to `pub mod` in `src/advance/mod.rs` (25 modules)
- [ ] T023 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/tokio_sample.rs`
- [ ] T024 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/futures_sample.rs`
- [ ] T025 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/bytes_sample.rs`
- [ ] T026 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/ollama_sample.rs`
- [ ] T027 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/rkyv_sample.rs`
- [ ] T028 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/sysinfo_sample.rs`
- [ ] T029 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/process_sample.rs`
- [ ] T030 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/includedir_sample.rs`
- [ ] T031 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/json_sample.rs`
- [ ] T032 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/tempfile_sample.rs`
- [ ] T033 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/memmap_sample.rs`
- [ ] T034 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/sqlx_sample.rs`
- [ ] T035 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/diesel_sample.rs`
- [ ] T036 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/csv_sample.rs`
- [ ] T037 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/getset_sample.rs`
- [ ] T038 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/hyper_sample.rs`
- [ ] T039 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/mio_sample.rs`
- [ ] T040 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/axum_sample.rs`
- [ ] T041 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/objectstore_sample.rs`
- [ ] T042 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/rayon_sample.rs`
- [ ] T043 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/cyclerc_sample.rs`
- [ ] T044 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/typealias_sample.rs`
- [ ] T045 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/dotenv_sample.rs`
- [ ] T046 [P] [US1] Change visibility from `private` to `pub` for all sample functions in `src/advance/cow_sample.rs`
- [ ] T047 [US1] Add `inventory::submit!` registration in `src/advance/mod.rs` for all advance topics (80+ functions)
- [ ] T048 [US1] Map each advance topic to its tutorial doc path (e.g., `"advance/async/async.md"`)

#### algo category registration

- [ ] T049 [P] [US1] Change `mod linked_list` to `pub mod linked_list` in `src/algo/mod.rs`
- [ ] T050 [P] [US1] Change `mod calc_pi_sample` to `pub mod calc_pi_sample` in `src/algo/mod.rs`
- [ ] T051 [P] [US1] Change visibility to `pub` for all 6 `calculate_pi_*` functions in `src/algo/calc_pi_sample.rs`
- [ ] T052 [P] [US1] Change visibility to `pub` for all linked list functions in `src/algo/linked_list.rs`
- [ ] T053 [US1] Add `inventory::submit!` registration in `src/algo/mod.rs` for all algo topics (8+ functions)
- [ ] T054 [US1] Map algo topics to tutorial doc paths (e.g., `"algo/algo.md"`)

#### CLI integration

- [ ] T055 [US1] Update `src/main.rs` to use clap-based CLI dispatcher instead of direct function calls
- [ ] T056 [US1] Implement `hello <category> <topic>` execution that looks up topic in registry and calls `func()`
- [ ] T057 [US1] Implement async sample wrapper using `tokio::runtime::Runtime::block_on()` for async functions
- [ ] T058 [US1] Add topic listing when category invoked without topic (e.g., `hello basic` shows all basic topics)

**Checkpoint**: User Story 1 ready - can run any sample from basic/advance/algo categories independently

---

## Phase 4: User Story 2 - Interactive Tutorial Experience (P2)

**Goal**: Running a sample displays tutorial documentation alongside sample output

**Independent Test**: Running `hello basic ownership` shows tutorial markdown from `docs/src/basic/ownership.md` before sample output

### Implementation for User Story 2

- [ ] T059 [P] [US2] Implement `display_tutorial(doc_path: &str)` function in `src/cli/docs_embed.rs`
- [ ] T060 [P] [US2] Add formatted output separator between tutorial content and sample output
- [ ] T061 [US2] Implement `--no-docs` flag to skip tutorial display and run sample directly
- [ ] T062 [US2] Add emoji/formatting headers for tutorial sections (📖 for tutorial, ▶ for running sample)
- [ ] T063 [US2] Implement `hello <category> <topic> --help` to show topic description without running
- [ ] T064 [US2] Test all basic category topics display correct tutorial content
- [ ] T065 [US2] Test all advance category topics display correct tutorial content
- [ ] T066 [US2] Test all algo category topics display correct tutorial content

**Checkpoint**: User Story 2 ready - samples display tutorial content alongside output

---

## Phase 5: User Story 3 - Unified Entry Point for All Examples (P3)

**Goal**: Single `hello` command provides access to all categories including awesome crate and IPC binaries

**Independent Test**: Can run `hello awesome grpc`, `hello list`, and see all categories in help menu

### awesome crate registration

- [ ] T067 [P] [US3] Add `pub mod` declarations for all awesome submodules in `crates/awesome/src/lib.rs`
- [ ] T068 [P] [US3] Change visibility to `pub` for all sample functions in `crates/awesome/src/services/*.rs` (14 files)
- [ ] T069 [P] [US3] Change visibility to `pub` for all sample functions in `crates/awesome/src/database/*.rs` (2 files)
- [ ] T070 [P] [US3] Change visibility to `pub` for all sample functions in `crates/awesome/src/templates/*.rs` (3 files)
- [ ] T071 [P] [US3] Change visibility to `pub` for all sample functions in `crates/awesome/src/mq/*.rs` (1 file)
- [ ] T072 [P] [US3] Change visibility to `pub` for all sample functions in `crates/awesome/src/data/*.rs` (1 file)
- [ ] T073 [P] [US3] Change visibility to `pub` for all sample functions in `crates/awesome/src/sequences/*.rs` (1 file)
- [ ] T074 [US3] Add `inventory::submit!` registration in `crates/awesome/src/lib.rs` for all awesome topics (22+ functions)
- [ ] T075 [US3] Add `SampleType::Server` annotation for long-running server samples (gRPC servers)
- [ ] T076 [US3] Map awesome topics to tutorial doc paths (e.g., `"awesome/awesome-overview.md"`)

### IPC binaries listing

- [ ] T077 [P] [US3] Create `src/bin/` scanner that discovers all 15 IPC binaries at compile time
- [ ] T078 [US3] Add IPC binaries to `hello list` output with "requires separate terminal" annotation
- [ ] T079 [US3] Skip IPC binaries from direct subcommand execution (list-only, not runnable via `hello bin <name>`)

### CLI polish

- [ ] T080 [US3] Implement `hello list` command showing all categories with topic counts
- [ ] T081 [US3] Add external dependency annotations to topic listings (e.g., "需要 Consul", "无外部依赖")
- [ ] T082 [US3] Implement fuzzy matching/suggestions for typos in topic names (e.g., "ownership" suggested for "ownrship")
- [ ] T083 [US3] Add top-level help menu with category descriptions when `hello` run without arguments

**Checkpoint**: User Story 3 ready - unified CLI with all categories and binaries accessible

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements, validation, and documentation

- [ ] T084 [P] Run `cargo clippy -- -D warnings` and fix all warnings
- [ ] T085 [P] Run `cargo fmt` and commit formatted code
- [ ] T086 [P] Update `README.md` with new `hello` CLI usage examples
- [ ] T087 [P] Create `QUICKSTART.md` with step-by-step guide for adding new samples
- [ ] T088 Test all error scenarios: unknown category, unknown topic, missing dependencies
- [ ] T089 [P] Add dependency check warnings for topics requiring Consul, MySQL, MQTT, Ollama
- [ ] T090 [P] Implement graceful skip for samples with missing dependencies (with `--force` override)
- [ ] T091 [P] Verify CLI startup time <50ms cold, <10ms warm
- [ ] T092 [P] Verify all existing inline tests in samples still pass
- [ ] T093 [P] Add `#[cfg(test)]` unit tests for CLI registry and dispatcher
- [ ] T094 [P] Document `inventory::submit!` pattern in code comments for future contributors

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Blocks all user stories - MUST complete before T013+
- **User Story 1 (Phase 3)**: Depends on T012 completion - enables MVP
- **User Story 2 (Phase 4)**: Depends on US1 completion - adds tutorial UX
- **User Story 3 (Phase 5)**: Depends on US1 completion - adds awesome crate
- **Polish (Phase 6)**: Depends on all user stories complete

### Within User Story 1

**Parallel opportunities (all can start after T012)**:
- T013-T021: basic category registration (can parallelize visibility changes across files)
- T022-T048: advance category registration (can parallelize across 25 modules)
- T049-T054: algo category registration (small, sequential)
- T055-T058: CLI dispatcher integration

**Within each category**:
1. First: Change module declarations to `pub mod`
2. Second: Change function visibility to `pub`
3. Third: Add `inventory::submit!` registrations
4. Fourth: Map to tutorial doc paths

### User Story Dependencies

- **US1 (P1)**: Foundation → MVP (run samples without docs)
- **US2 (P2)**: US1 → Tutorial display
- **US3 (P3)**: US1 → Awesome crate + binaries listing

---

## Parallel Examples

### After Foundation (T012 complete) - Maximum Parallelism

```bash
# Developer A: basic category
task(description="T013-T021: Register basic samples", prompt="...")

# Developer B: advance category  
task(description="T022-T048: Register advance samples", prompt="...")

# Developer C: algo category + CLI integration
task(description="T049-T058: Register algo + CLI dispatcher", prompt="...")
```

### Within Advance Category (T022-T046 are independent file edits)

```bash
# Launch 25 parallel tasks for visibility changes
task(description="T023: tokio_sample visibility", prompt="...")
task(description="T024: futures_sample visibility", prompt="...")
task(description="T025: bytes_sample visibility", prompt="...")
# ... launch all 24 visibility tasks in parallel
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete **Phase 1: Setup** (T001-T004) - 1 day
2. Complete **Phase 2: Foundational** (T005-T012) - 2 days
3. Complete **Phase 3: US1 basic only** (T013-T021) - 1 day
4. **STOP and TEST**: `cargo run -- basic ownership` should run sample
5. **Deploy MVP**: basic category works!

### Incremental Delivery

1. Foundation → basic category works → Demo to team
2. Add advance category → More samples → Demo
3. Add algo category → All core samples → Demo
4. Add tutorial display (US2) → Better UX → Demo
5. Add awesome crate (US3) → Complete → Ship

### Parallel Team Strategy

With 3 developers:
- **Dev A**: Phase 1 + Phase 2 (foundation) → Then US1 basic
- **Dev B**: US1 advance (25 modules, most work)
- **Dev C**: US1 algo → US2 tutorial display → US3 awesome

After foundation done, all devs work in parallel on their story.

---

## Task Count Summary

| Phase | Task Count | Description |
|-------|-----------|-------------|
| Setup | 4 tasks | CLI framework setup |
| Foundational | 8 tasks | Registry and dispatcher |
| User Story 1 | 46 tasks | Sample registration (basic: 9, advance: 26, algo: 6, CLI: 5) |
| User Story 2 | 8 tasks | Tutorial display UX |
| User Story 3 | 17 tasks | Awesome crate + binaries |
| Polish | 11 tasks | Cleanup and validation |
| **Total** | **94 tasks** | |

### Task Breakdown by Category

- **Infrastructure**: 12 tasks (Setup + Foundational)
- **basic registration**: 9 tasks
- **advance registration**: 26 tasks (largest block - 25 modules)
- **algo registration**: 6 tasks
- **awesome registration**: 10 tasks
- **awesome sub-registries**: 7 tasks
- **CLI integration**: 7 tasks
- **Tutorial UX**: 8 tasks
- **Polish**: 11 tasks

---

## Format Validation Checklist

✅ All tasks follow format: `- [ ] T### [P?] [US#] Description with file path`
✅ All user story tasks have [US1], [US2], [US3] labels
✅ Setup and Foundational tasks have NO story label
✅ Parallelizable tasks marked with [P]
✅ File paths included in every task description
✅ Tasks are atomic and independently completable

---

## Notes

- `inventory` crate chosen over `linkme` for simplicity (3 lines per sample)
- Visibility changes (`private` → `pub`) are safe - samples are demo code
- Async samples need special handling in registry (store as async fn + wrap with runtime)
- 15 IPC binaries are **list-only** - not runnable via subcommands (require separate terminals)
- Tutorial doc paths must match existing structure in `docs/src/`
- `cargo clippy -- -D warnings` must pass before merging
