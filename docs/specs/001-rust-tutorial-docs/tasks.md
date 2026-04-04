# Tasks: Rust Tutorial Documentation

**Branch**: `001-rust-tutorial-docs`
**Input**: Design documents from `/docs/specs/001-rust-tutorial-docs/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅ (in research.md), quickstart.md ✅ (in research.md)

**Tests**: This is a documentation-only feature. Testing = compilation verification + mdBook build validation. No traditional unit tests.

**Organization**: Tasks organized by user story (P1→P2→P3→P4→P5) to enable parallel chapter writing within each story.

## Format Checklist

- [x] Checkbox: `- [ ]`
- [x] Task ID: Sequential (T001, T002...)
- [x] [P] marker: For parallelizable tasks
- [x] [Story] label: [US1], [US2], etc. for user story phases
- [x] File paths: Exact paths for all tasks

## Path Conventions

- **Documentation source**: `docs/src/`
- **Specification**: `docs/specs/001-rust-tutorial-docs/`
- **Code samples**: `src/`, `crates/` (read-only, no modifications)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Environment setup, tooling installation, workspace initialization

- [ ] T001 Install mdBook and plugins: `cargo install mdbook mdbook-admonish mdbook-alerts mdbook-pagetoc`
- [ ] T002 Verify mdBook build: `cd docs && mdbook build` (should succeed with zero warnings)
- [ ] T003 [P] Create chapter写作模板 in `docs/src/templates/chapter-template.md` based on 12-section structure
- [ ] T004 [P] Setup writing environment documentation (README for contributors) in `docs/WRITING_GUIDE.md`
- [ ] T005 Verify workspace compilation: `cargo build --workspace` (all examples must compile)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story chapters can be written

**⚠️ CRITICAL**: No chapter writing can begin until this phase is complete

- [x] T006 Create bilingual terminology glossary in `docs/src/glossary.md` (中英文对照表)
- [x] T007 Write "文档写作原则" chapter in `docs/src/writing-principles.md` covering:
  - 平实的语言 (plain language)
  - 阅读轻松 (easy reading)
  - 鼓励性语气 (encouraging tone)
  - 认知负荷管理 (cognitive load management)
- [x] T008 Create sample quality reference document in `docs/src/SAMPLE_QUALITY.md` documenting:
  - ✅ HIGH QUALITY samples: ownership_sample.rs, datatype_sample.rs, tokio_sample.rs
  - ⚠️ REQUIRES CONTEXT samples: dynmaic_injection_box_sample.rs, threads_sample.rs
  - ✗ CRITICAL ISSUES samples: pointer_sample.rs:17, sqlite_vec_sample.rs:7, tonic_store_server.rs:34
  - Hardcoded values note: IP addresses, socket paths
- [x] T009 Setup cross-reference infrastructure:
  - Link formatting standards
  - External reference format (Rust Book, docs.rs)
  - Code file reference format
- [x] T010 Create mdBook configuration verification (SUMMARY.md structure validation)

**Checkpoint**: Foundation ready - user story chapter writing can now begin in parallel

---

## Phase 3: User Story 1 - 掌握 Rust 基础概念 (Priority: P1) 🎯 MVP

**Goal**: Complete all 14 Basic section chapters covering fundamental Rust concepts

**Independent Test**: User can successfully compile and run all basic examples (`cargo run`) and explain ownership, borrowing, and lifetimes after reading. Can identify and fix common ownership errors.

### Implementation for User Story 1

**Basic Section Chapters** (14 files total):

- [x] T011 [P] [US1] Write `docs/src/basic/basic-overview.md` (section introduction, 500+ characters)
- [x] T012 [P] [US1] Write `docs/src/basic/expression.md` based on `src/basic/expression_sample.rs` (variables, expressions)
- [x] T013 [P] [US1] Expand `docs/src/basic/datatype.md` based on `src/basic/datatype_sample.rs` (1017 lines → split into focused chapters)
- [x] T014 [P] [US1] Expand `docs/src/basic/ownership.md` based on `src/basic/ownership_sample.rs` (核心章节，include borrowing)
- [x] T015 [P] [US1] Write `docs/src/basic/struct.md` based on `src/basic/rectangle.rs` and datatype examples
- [x] T016 [P] [US1] Expand `docs/src/basic/struct-fields.md` (currently 1 line → 500+ characters)
- [x] T017 [P] [US1] Expand `docs/src/basic/struct-methods.md` (currently 1 line → 500+ characters)
- [x] T018 [P] [US1] Expand `docs/src/basic/enums.md` (currently 1 line → 500+ characters)
- [x] T019 [P] [US1] Expand `docs/src/basic/trait.md` based on `src/basic/traits_sample.rs` (69 lines → 500+ characters)
- [x] T020 [P] [US1] Expand `docs/src/basic/module.md` based on `src/basic/module_sample.rs` (20 lines → 500+ characters)
- [x] T021 [P] [US1] Write `docs/src/basic/generic.md` based on `src/basic/generic_sample.rs`
- [x] T022 [P] [US1] Write `docs/src/basic/closure.md` based on `src/basic/closure_sample.rs`
- [x] T023 [P] [US1] Write `docs/src/basic/threads.md` based on `src/basic/threads_sample.rs` ⚠️ Must document unsafe patterns
- [x] T024 [P] [US1] Write `docs/src/basic/cfg_if.md` based on `src/basic/cfg_if_sample.rs`
- [x] T025 [P] [US1] Write `docs/src/basic/pointer.md` based on `src/basic/pointer_sample.rs` ⚠️ Must warn about unsafe code
- [x] T026 [P] [US1] Write `docs/src/basic/logger.md` based on `src/basic/logger_sample.rs`
- [x] T027 [P] [US1] Write `docs/src/basic/tracing.md` based on `src/basic/tracing_sample.rs`
- [x] T028 [P] [US1] Write `docs/src/basic/visiable.md` based on `src/basic/visiable_sample.rs`

**Quality Verification**:
- [x] T029 [US1] Verify all Basic chapters compile: `cargo build --workspace`
- [x] T030 [US1] Verify all Basic chapters mdBook build: `cd docs && mdbook build`
- [x] T031 [US1] Cross-link verification: Ensure all Basic chapters have proper prerequisites and forward links

**Checkpoint**: User Story 1 fully functional - learners can master Rust basics independently

---

## Phase 4: User Story 2 - 掌握异步与系统编程 (Priority: P2)

**Goal**: Complete all 26 Advance section chapters covering async programming, databases, web frameworks, serialization

**Independent Test**: Developer can create a basic async web service with database connectivity. Understands async/await patterns and can work with popular frameworks.

### Implementation for User Story 2

**Advance Section Chapters** (26 files total):

- [ ] T032 [P] [US2] Write `docs/src/advance/advance-overview.md` (section introduction, 500+ characters)
- [x] T033 [P] [US2] Write `docs/src/advance/tokio.md` based on `src/advance/tokio_sample.rs` (509 lines → split: Async Basics + Network Programming)
- [x] T034 [P] [US2] Write `docs/src/advance/futures.md` based on `src/advance/futures_sample.rs`
- [x] T035 [P] [US2] Write `docs/src/advance/sqlx.md` based on `src/advance/sqlx_sample.rs` (async database)
- [x] T036 [P] [US2] Write `docs/src/advance/diesel.md` based on `src/advance/diesel_sample.rs` (ORM)
- [x] T037 [P] [US2] Write `docs/src/advance/axum.md` based on `src/advance/axum_sample.rs` (web framework)
- [x] T038 [P] [US2] Write `docs/src/advance/hyper.md` based on `src/advance/hyper_sample.rs` (lower-level HTTP)
- [x] T039 [P] [US2] Write `docs/src/advance/json.md` based on `src/advance/json_sample.rs` (serde_json)
- [x] T040 [P] [US2] Write `docs/src/advance/csv.md` based on `src/advance/csv_sample.rs` (CSV handling)
- [x] T041 [P] [US2] Write `docs/src/advance/rkyv.md` based on `src/advance/rkyv_sample.rs` (zero-copy serialization)
- [x] T042 [P] [US2] Write `docs/src/advance/tempfile.md` based on `src/advance/tempfile_sample.rs`
- [x] T043 [P] [US2] Write `docs/src/advance/memmap.md` based on `src/advance/memmap_sample.rs`
- [x] T044 [P] [US2] Write `docs/src/advance/dotenv.md` based on `src/advance/dotenv_sample.rs`
- [x] T045 [P] [US2] Write `docs/src/advance/bytes.md` based on `src/advance/bytes_sample.rs`
- [x] T046 [P] [US2] Write `docs/src/advance/cow.md` based on `src/advance/cow_sample.rs`
- [x] T047 [P] [US2] Write `docs/src/advance/getset.md` based on `src/advance/getset_sample.rs`
- [x] T048 [P] [US2] Write `docs/src/advance/rayon.md` based on `src/advance/rayon_sample.rs` (parallelism)
- [ ] T049 [P] [US2] Write `docs/src/advance/mock.md` based on `src/advance/mock_sample.rs`
- [ ] T050 [P] [US2] Write `docs/src/advance/rspec.md` based on `src/advance/rspec_sample.rs`
- [ ] T051 [P] [US2] Write `docs/src/advance/objectstore.md` based on `src/advance/objectstore_sample.rs`
- [ ] T052 [P] [US2] Write `docs/src/advance/ollama.md` based on `src/advance/ollama_sample.rs`
- [ ] T053 [P] [US2] Write `docs/src/advance/process.md` based on `src/advance/process_sample.rs`
- [ ] T054 [P] [US2] Write `docs/src/advance/sysinfo.md` based on `src/advance/sysinfo_sample.rs`
- [ ] T055 [P] [US2] Write `docs/src/advance/includedir.md` based on `src/advance/includedir_sample.rs`
- [x] T056 [P] [US2] Write `docs/src/advance/typealias.md` based on `src/advance/typealias_sample.rs`
- [x] T057 [P] [US2] Write `docs/src/advance/macros.md` based on `src/advance/macros_sample.rs`
- [x] T058 [P] [US2] Write `docs/src/advance/cyclerc.md` based on `src/advance/cyclerc_sample.rs`
- [ ] T059 [P] [US2] Write `docs/src/advance/mio.md` based on `src/advance/mio_sample.rs`

**⚠️ Special Requirements for US2**:
- [ ] T060 [US2] Add anti-pattern warnings: `std::sync::Mutex` in async context (tonic_store_server.rs:34)
- [ ] T061 [US2] Add async best practices callout boxes throughout
- [ ] T062 [US2] Cross-database chapter consistency check (SQLx vs Diesel approaches)

**Quality Verification**:
- [ ] T063 [US2] Verify all Advance chapters compile: `cargo build --workspace`
- [ ] T064 [US2] Verify all Advance chapters mdBook build: `cd docs && mdbook build`
- [ ] T065 [US2] Cross-link verification: Ensure proper progression from Basic → Advance

**Checkpoint**: User Story 2 fully functional - learners can build production async applications

---

## Phase 5: User Story 3 - 使用 Awesome 框架构建生产应用 (Priority: P3)

**Goal**: Complete 7+ Awesome section chapters covering production framework patterns

**Independent Test**: Engineer can scaffold a new service using awesome framework patterns. Understands service lifecycle and dependency injection.

### Implementation for User Story 3

**Awesome Section Chapters** (7+ directories):

- [ ] T066 [P] [US3] Write `docs/src/awesome/awesome-overview.md` (expand from 13 lines → 500+ characters)
- [ ] T067 [P] [US3] Write `docs/src/awesome/services.md` based on `crates/awesome/src/services/framework/` (service lifecycle)
- [ ] T068 [P] [US3] Expand `docs/src/awesome/dependency_injection.md` (371 lines → verify completeness, add DI patterns)
- [ ] T069 [P] [US3] Write `docs/src/awesome/database.md` based on `crates/awesome/src/database/` (SurrealDB, SQLite)
- [ ] T070 [P] [US3] Write `docs/src/awesome/mq.md` based on `crates/awesome/src/mq/rumqtt_sample.rs` (MQTT, expand from 2 lines)
- [ ] T071 [P] [US3] Write `docs/src/awesome/sequences.md` based on `crates/awesome/src/sequences/` (UUID generation)
- [ ] T072 [P] [US3] Write `docs/src/awesome/templates.md` based on `crates/awesome/src/templates/` (Tera, Liquid, Pest)
- [ ] T073 [P] [US3] Write `docs/src/awesome/data.md` based on `crates/awesome/src/data/polars_sample.rs` (data frames)
- [ ] T074 [P] [US3] Expand `docs/src/awesome/plugin.md` (141 lines → verify completeness)

**⚠️ Special Requirements for US3**:
- [ ] T075 [US3] Document typo in filenames: `dynmaic_injection_*.rs` → should be "dynamic"
- [ ] T076 [US3] Document hardcoded values warning (192.168.2.6, 192.168.2.7)
- [ ] T077 [US3] Document two Consul approaches (rs_consul crate vs custom HTTP client)
- [ ] T078 [US3] Add production deployment considerations

**Quality Verification**:
- [ ] T079 [US3] Verify all Awesome chapters compile: `cargo build -p awesome`
- [ ] T080 [US3] Verify all Awesome chapters mdBook build: `cd docs && mdbook build`
- [ ] T081 [US3] Verify service framework prerequisites (Basic + Advance required)

**Checkpoint**: User Story 3 fully functional - learners can use production frameworks

---

## Phase 6: User Story 4 - 解决算法挑战 (Priority: P4)

**Goal**: Complete Algo and LeetCode chapters for algorithm practice

**Independent Test**: Learner can implement data structures (linked list) and solve LeetCode #1 (Two Sum) and #2 (Add Two Numbers) independently.

### Implementation for User Story 4

**Algo Section**:
- [ ] T082 [P] [US4] Write `docs/src/algo/algo.md` based on `src/algo/linked_list.rs` and `calc_pi_sample.rs` (expand from 1 line)
  - Linked list implementation in Rust
  - Pi calculation algorithm
  - Memory safety considerations

**LeetCode Section**:
- [ ] T083 [P] [US4] Expand `docs/src/leetcode/leetcode.md` (52 lines → 500+ characters)
- [ ] T084 [P] [US4] Document LeetCode #1 (Two Sum) based on `crates/leetcode/src/solution_0001.rs`
  - Problem statement
  - Approach explanation
  - Rust-specific considerations (ownership in hash maps)
- [ ] T085 [P] [US4] Document LeetCode #2 (Add Two Numbers) based on `crates/leetcode/src/solution_0002.rs`
  - Problem statement
  - Linked list manipulation
  - Edge cases handling

**Quality Verification**:
- [ ] T086 [US4] Verify all Algo/LeetCode chapters compile: `cargo build -p leetcode`
- [ ] T087 [US4] Verify mdBook build: `cd docs && mdbook build`

**Checkpoint**: User Story 4 complete - algorithm practice resources ready

---

## Phase 7: User Story 5 - 运行 IPC 与分布式示例 (Priority: P4)

**Goal**: Document 15 IPC binary examples for distributed systems learning

**Independent Test**: Developer can run gRPC client/server pairs and understand message passing between processes.

### Implementation for User Story 5

**IPC Section** (integrated across chapters, not separate directory):

- [ ] T088 [P] [US5] Write gRPC integration chapter incorporating:
  - `greeter_server.rs` + `greeter_client.rs`
  - `grpc_hello_server.rs` + `grpc_hello_client.rs`
  - `grpc_store_server.rs` + `grpc_store_client.rs`
  - Location: `docs/src/advance/grpc.md` or integrate into existing chapters

- [ ] T089 [P] [US5] Write UDS (Unix Domain Sockets) chapter incorporating:
  - `uds_server.rs`, `uds_client.rs`, `uds_parent.rs`, `uds_client.rs`
  - Platform note: Unix-only, Windows alternatives
  - Location: `docs/src/advance/uds.md` or integrate

- [ ] T090 [P] [US5] Write stdio IPC chapter incorporating:
  - `stdio_parent.rs` + `stdio_child.rs`
  - Location: `docs/src/advance/stdio.md` or integrate

- [ ] T091 [P] [US5] Write process control chapter incorporating:
  - `app_sys_ctl.rs`, `app_nix_ctl.rs`
  - Location: `docs/src/advance/process-control.md`

**Platform-Specific Notes**:
- [ ] T092 [US5] Add platform compatibility matrix (macOS/Linux/Windows)
- [ ] T093 [US5] Document `protoc` installation for gRPC chapters (29.3+ requirement)
- [ ] T094 [US5] Add socket path configuration note (/tmp/hello.socket → configurable)

**Quality Verification**:
- [ ] T095 [US5] Verify all IPC examples compile: `cargo build --workspace`
- [ ] T096 [US5] Verify IPC examples can run (manual test or scripted verification)
- [ ] T097 [US5] Verify mdBook build: `cd docs && mdbook build`

**Checkpoint**: User Story 5 complete - IPC patterns documented

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, consistency checks, documentation completeness

### Cross-Chapter Consistency
- [ ] T098 Verify bilingual terminology consistency across all chapters (所有权 (ownership) format)
- [ ] T099 Verify 12-section template applied consistently to all chapters
- [ ] T100 Verify cross-references work (internal links, external links)
- [ ] T101 Verify code examples <15 lines constraint (split larger examples)
- [ ] T102 Verify "常见错误" sections include actual compiler error messages

### Build & Deployment Validation
- [ ] T103 Full workspace compilation: `cargo build --workspace` (zero errors)
- [ ] T104 mdBook full build: `cd docs && mdbook build` (zero warnings)
- [ ] T105 Link validation: Check all internal/external links
- [ ] T106 Verify SUMMARY.md completeness (all chapters listed)
- [ ] T107 Verify docs/book/ generated HTML is up-to-date (or gitignored per assumption)

### Quality Gates
- [ ] T108 Verify all chapters have 500+ Chinese characters (spot check or automated)
- [ ] T109 Verify all chapters have learning objectives (FR-006)
- [ ] T110 Verify all chapters have exercises (FR-007)
- [ ] T111 Verify all chapters have prerequisites (FR-011)
- [ ] T112 Verify all chapters have troubleshooting (FR-010)

### Documentation Completeness
- [ ] T113 Verify unsafe code warnings present (pointer_sample.rs, threads_sample.rs, sqlite_vec_sample.rs)
- [ ] T114 Verify anti-pattern callouts (std::sync::Mutex in async context)
- [ ] T115 Verify hardcoded value warnings (IP addresses, socket paths)
- [ ] T116 Verify platform-specific notes (Unix-only for UDS, Windows alternatives)
- [ ] T117 Version information documented (Rust 2024, crate versions)

### Final Deliverables
- [ ] T118 Create README for documentation in `docs/README.md`
- [ ] T119 Update main README.md with documentation links
- [ ] T120 Create changelog entry for documentation completion

---

## Dependencies & Parallel Execution

### User Story Dependency Graph

```
Phase 1 (Setup) → Phase 2 (Foundation)
                        ↓
            ┌───────────────────────────┐
            │                           │
            ↓                           ↓
    Phase 3 (US1: Basic)        Phase 4 (US2: Advance)
    [P1 - MVP Scope]            [P2]
            │                           │
            └───────────┬───────────────┘
                        ↓
              Phase 5 (US3: Awesome)
              [P3 - Requires US1+US2]
                        │
            ┌───────────┴───────────────┐
            │                           │
            ↓                           ↓
    Phase 6 (US4: Algo)         Phase 7 (US5: IPC)
    [P4 - Independent]          [P4 - Independent]
                        │
                        ↓
              Phase 8 (Polish)
```

### Parallel Execution Opportunities

**Within phases (maximum parallelization)**:
- **Phase 3 (US1)**: T011-T028 can run in PARALLEL (18 tasks across different files)
- **Phase 4 (US2)**: T033-T059 can run in PARALLEL (27 tasks across different files)
- **Phase 5 (US3)**: T067-T074 can run in PARALLEL (8 tasks across different files)
- **Phase 6 (US4)**: T082-T085 can run in PARALLEL (4 tasks)
- **Phase 7 (US5)**: T088-T091 can run in PARALLEL (4 tasks)

**Across phases (sequential dependencies)**:
- Foundation must complete before ANY user story work
- US1 (MVP) should complete before US2 (builds on basics)
- US3 requires US1+US2 completion (production patterns assume knowledge)
- US4 and US5 are independent of each other and can run in parallel

### Suggested MVP Scope

**Minimum Viable Product**: **Phase 1 → Phase 2 → Phase 3 (US1 only)**

This delivers:
- Complete Basic section (14 chapters)
- Learners can master Rust fundamentals
- Independent verification: `cargo run` succeeds, learners explain ownership
- Can ship independently while US2-US5 continue

---

## Implementation Strategy

**Recommended approach**: MVP first, incremental delivery

1. **Round 1**: Setup + Foundation (T001-T010)
2. **Round 2**: User Story 1 - MVP (T011-T031) → **SHIPPABLE**
3. **Round 3**: User Story 2 (T032-T065) → **SHIPPABLE**
4. **Round 4**: User Story 3 (T066-T081) → **SHIPPABLE**
5. **Round 5**: User Stories 4+5 (T082-T097) parallel → **SHIPPABLE**
6. **Round 6**: Polish (T098-T120) → **FINAL**

---

## Task Summary

| Phase | Description                          | Task Count | Parallel?  |
|-------|--------------------------------------|------------|------------|
| P1    | Setup                                | 5          | Partial    |
| P2    | Foundational                         | 5          | Partial    |
| P3    | User Story 1 (Basic)                 | 18         | ✅ Yes     |
| P4    | User Story 2 (Advance)               | 28         | ✅ Yes     |
| P5    | User Story 3 (Awesome)               | 16         | ✅ Yes     |
| P6    | User Story 4 (Algo/LeetCode)         | 6          | ✅ Yes     |
| P7    | User Story 5 (IPC)                   | 10         | ✅ Yes     |
| P8    | Polish & Cross-Cutting               | 23         | Partial    |
| **TOTAL** |                                    | **111**    |            |

**Format Validation**: ✅ All 111 tasks follow checklist format with checkbox, ID, [P] marker (where parallel), [Story] label (user story phases), and file paths.

---

## Phase 6: Extended Features (SC-010 to SC-018)

**Purpose**: Implement enhanced learning features based on competitive analysis

### 延伸阅读 (Extended Reading) - SC-010

- [ ] T120 [P] Add "延伸阅读" section to all completed chapters (expression, ownership, struct)
  - Include at least 3 related libraries per chapter
  - Add selection guidelines (performance vs compatibility vs size)
  - Reference: `docs/QUALITY_RUBRICS.md#library-selection`

- [ ] T121 [P] Create library selection rubric in `docs/QUALITY_RUBRICS.md`
  - Criteria: Last update (<2 years), Stars (>100), Active maintenance, Documentation quality
  - Rating scale: 🟢 Recommended, 🟡 Use with caution, 🔴 Avoid (abandoned)

### 代码片段速查 (Code Snippets Quick Reference) - SC-011

- [ ] T122 [P] Create `docs/src/quick_reference/snippets.md`
  - Categories (10 minimum):
    1. File I/O operations
    2. Collection operations (Vec, HashMap)
    3. String manipulation
    4. Error handling patterns
    5. Option/Result utilities
    6. Iteration patterns
    7. Trait implementations
    8. Concurrency primitives
    9. Testing patterns
    10. Common algorithms (sorting, searching)

### ASCII Diagrams - SC-012

- [x] T123 [P] Add ASCII diagrams to ownership.md
  - Memory layout (stack vs heap)
  - Ownership transfer visualization
  - Borrowing rules diagram

- [x] T124 [P] Add ASCII diagrams to struct.md
  - Struct memory layout
  - Field access patterns

- [x] T125 [P] Add ASCII diagrams to generic.md  
  - Monomorphization process
  - Trait bound relationships

### Learning Path Skill Tree - SC-013

- [x] T126 [P] Create `docs/src/learning_path.md`
  - Visual skill tree with all 15+ chapters
  - Difficulty levels (🟢🟡🔴)
  - Prerequisites clearly marked
  - Progress tracking section
  - Estimated completion time per section

### Project Implementation - SC-014

**Project 1: 命令行待办事项** (Beginner 🟢)
- [ ] T127 [P] Create `projects/todo-cli/README.md`
- [ ] T128 [P] Implement basic CLI structure (~50 lines)
- [ ] T129 [P] Implement add/list commands (~100 lines)
- [ ] T130 [P] Implement done/delete commands (~150 lines)
- [ ] T131 [P] Add JSON persistence (~200 lines total)
- [ ] T132 [P] Add checkpoints and completion criteria

**Project 2: 简易 HTTP 服务器** (Intermediate 🟡)
- [ ] T133 [P] Create `projects/http-server/README.md`
- [ ] T134 [P] Implement TCP listener (~100 lines)
- [ ] T135 [P] Implement HTTP request parsing (~250 lines)
- [ ] T136 [P] Implement routing and handlers (~400 lines)
- [ ] T137 [P] Add static file serving (~500 lines total)
- [ ] T138 [P] Add checkpoints and completion criteria

**Project 3: 多线程爬虫** (Advanced 🔴)
- [ ] T139 [P] Create `projects/web-scraper/README.md`
- [ ] T140 [P] Implement basic HTTP client (~200 lines)
- [ ] T141 [P] Add HTML parsing (~400 lines)
- [ ] T142 [P] Implement thread pool (~600 lines)
- [ ] T143 [P] Add channel-based communication (~800 lines)
- [ ] T144 [P] Implement rate limiting and error recovery (~1000 lines)
- [ ] T145 [P] Add checkpoints and completion criteria

### 知识检查 (Knowledge Checkpoints) - SC-015

- [x] T146 [P] Add "知识检查" section to expression.md
  - 3 quiz questions (mix of prediction, selection, fix-it)
  - Answers with detailed explanations
  - Difficulty distribution: 50% 🟢, 40% 🟡, 10% 🔴

- [x] T147 [P] Add "知识检查" section to ownership.md
  - 3 quiz questions focused on ownership rules
  - Include compiler error prediction
  - Include fix-it exercises

- [x] T148 [P] Add "知识检查" section to struct.md
  - 3 quiz questions on struct patterns
  - Include field access scenarios
  - Include visibility questions

- [ ] T149 [P] Create quiz template in `docs/src/templates/quiz-template.md`
  - Question types standardization
  - Answer format with explanations
  - Difficulty marking guidelines

### Community Contribution - SC-016

- [ ] T150 [P] Create `docs/CONTRIBUTING.md`
  - Contribution guidelines (Issue, PR, translation)
  - Contributor benefits
  - Contribution standards checklist

- [ ] T151 [P] Create `docs/QUALITY_AUDIT.md`
  - Pre-publish checklist
  - Quarterly review checklist
  - Quality metrics (compilation rate, link validity, exercise coverage)

### Difficulty Levels - SC-017

- [ ] T152 [P] Define difficulty rubric in `docs/QUALITY_RUBRICS.md`
  - 🟢 Beginner: Fundamental concepts, no prerequisites
  - 🟡 Intermediate: Requires chapter prerequisites, combines concepts
  - 🔴 Advanced: Complex topics, boundary cases, expert-level

- [ ] T153 [P] Mark all completed chapters with difficulty levels
  - expression.md: 🟢
  - ownership.md: 🟡 (requires expression)
  - struct.md: 🟡 (requires ownership)
  - threads.md: 🔴 (advanced)
  - pointer.md: 🔴 (unsafe, advanced)

### Cross-Chapter Review - SC-018

- [ ] T154 [P] Create review section after chapter 5
  - Comprehensive quiz (variables through enums)
  - Integrated exercise: Build simple game character system
  - Self-assessment checklist

- [ ] T155 [P] Create review section after chapter 10
  - Comprehensive quiz (traits through async)
  - Integrated exercise: Build HTTP server
  - Self-assessment checklist

- [ ] T156 [P] Create review section after chapter 15
  - Final comprehensive exam
  - Capstone project: Multithreaded scraper
  - Graduation checklist

---

## Quality Rubrics Appendix

### Library Selection Criteria (for SC-010)

Evaluate each library on:

1. **Maintenance** (Weight: 40%)
   - Last update: <1 year ✅, <2 years ⚠️, >2 years ❌
   - Open issues: <50 ✅, <200 ⚠️, >200 ❌
   - Response time: <1 week ✅, <1 month ⚠️, >1 month ❌

2. **Quality** (Weight: 30%)
   - Stars: >500 ✅, >100 ⚠️, <100 ❌
   - Documentation: Complete ✅, Partial ⚠️, Missing ❌
   - Tests: >80% coverage ✅, >50% ⚠️, <50% ❌

3. **Compatibility** (Weight: 20%)
   - Rust edition: 2021 ✅, 2018 ⚠️, older ❌
   - MSRV: <1.70 ✅, <1.60 ⚠️, older ❌

4. **Community** (Weight: 10%)
   - Contributors: >10 ✅, >5 ⚠️, <5 ❌
   - Downloads: >10k/month ✅, >1k ⚠️, <1k ❌

**Rating**:
- 🟢 Recommended: Score >80%
- 🟡 Use with caution: Score 50-80%
- 🔴 Avoid: Score <50%

### Operation Categories (for SC-011)

10 required categories with 5+ snippets each:

1. **File I/O**: read, write, append, copy, delete
2. **Collections**: Vec creation, push, iterate, filter, map
3. **String ops**: create, concatenate, split, replace, parse
4. **Error handling**: Result, Option, ?, unwrap, expect
5. **Iteration**: for, while, iterators, fold, collect
6. **Traits**: define, implement, derive, blanket impls
7. **Concurrency**: thread::spawn, channels, Mutex, Arc
8. **Testing**: unit tests, integration tests, benchmarks
9. **Generics**: type parameters, trait bounds, where clauses
10. **Macros**: declarative macros, procedural hints

### Difficulty Levels (for SC-017)

**🟢 Beginner**:
- No prerequisites beyond previous chapters
- Single concept focus
- Straightforward examples
- Minimal error handling

**🟡 Intermediate**:
- Requires 2+ previous chapters
- Combines multiple concepts
- Real-world scenarios
- Proper error handling

**🔴 Advanced**:
- Requires 5+ previous chapters
- Complex interactions
- Edge cases and pitfalls
- Performance considerations
- Unsafe code (when applicable)


---

## Task Order Refinements

### Verification Checkpoint Consolidation

**Previous**: T002, T005, T029, T030 scattered

**Consolidated Quality Gates**:

- [ ] T157 [Checkpoint] Phase 1 Verification
  - ✅ T001 Install mdBook
  - ✅ T002 First build test
  - ✅ T005 Workspace compilation

- [ ] T158 [Checkpoint] US1 Completion Verification
  - Run: `cargo build --workspace` (zero errors)
  - Run: `cd docs && mdbook build` (zero warnings)
  - Checklist: All 18 chapters have 12-section structure
  - Checklist: All chapters have GitHub links

- [ ] T159 [Checkpoint] Extended Features Verification
  - Checklist: All chapters have "延伸阅读" (SC-010)
  - Checklist: All chapters have ASCII diagrams where applicable (SC-012)
  - Checklist: All chapters have "知识检查" (SC-015)
  - Checklist: All chapters marked with difficulty (SC-017)
  - Verify: `docs/src/quick_reference/snippets.md` created (SC-011)
  - Verify: `docs/src/learning_path.md` created (SC-013)
  - Verify: 3 projects created with checkpoints (SC-014)
  - Verify: `docs/CONTRIBUTING.md` created (SC-016)
  - Verify: `docs/QUALITY_AUDIT.md` created (SC-016)

### Macros Crate Documentation (FR-013)

- [ ] T160 [US2] Document `crates/macros/` proc-macro crate
  - Explain #[log] attribute macro
  - Show before/after code expansion
  - Provide use cases and limitations
  - Link to Rust Book macro chapter

