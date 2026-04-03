# Feature Specification: Rust Tutorial Documentation

**Feature Branch**: `001-rust-tutorial-docs`  
**Created**: 2026-04-03  
**Status**: Draft  
**Input**: 将此项目的 rust 学习样例实现，构建为 docs/src mdbook 的每个章节的教程文档，让读者可以通过 hello rust 提供的样例，学习 rust 的所有基础及高级知识，还有框架应用学习

## User Scenarios & Testing

### User Story 1 - 掌握 Rust 基础概念 (Priority: P1)

A beginner programmer reads through the "Basic" section and learns fundamental Rust concepts by following executable code examples from `src/basic/*.rs` sample files. Each chapter provides clear explanations with working code snippets that can be run locally.

**Why this priority**: This is the foundation - without solid basics, users cannot progress to advanced topics. It delivers immediate value to beginners starting their Rust journey.

**Independent Test**: User can successfully compile and run all basic examples (`cargo run`) and explain ownership, borrowing, and lifetimes after completing the section. Users should be able to identify and fix common ownership errors.

**Acceptance Scenarios**:

1. **Given** a new Rust learner with basic programming knowledge, **When** they read the "变量表达式" chapter (based on `expression_sample.rs`), **Then** they can write and execute Rust expressions confidently and understand variable binding
2. **Given** a reader studying "了解所有权" (based on `ownership_sample.rs`), **When** they finish the chapter, **Then** they can explain ownership rules, identify borrowing violations, and avoid common mistakes
3. **Given** the "结构体" and "枚举" chapters, **When** completed, **Then** the reader can model real-world data structures using Rust types and pattern match on enum variants
4. **Given** the "特征" chapter (based on `traits_sample.rs`), **When** completed, **Then** the reader understands trait-based polymorphism and can implement custom traits for their types
5. **Given** the "模块" chapter (based on `module_sample.rs`), **When** completed, **Then** the reader can organize code into modules and understand visibility (pub/use)
6. **Given** the "并发" materials (based on `threads_sample.rs`), **When** completed, **Then** the reader can spawn threads and understand basic concurrency patterns

---

### User Story 2 - 掌握异步与系统编程 (Priority: P2)

An intermediate Rust developer progresses through the "Advance" section to learn async programming, database integration, web frameworks, and systems-level patterns using real-world examples from `src/advance/*.rs`.

**Why this priority**: Builds on basics to enable production-ready Rust development. Essential for developers wanting to build real applications with databases, web services, and async code.

**Independent Test**: Developer can create a basic async web service with database connectivity after completing this section. They should understand async/await patterns and be able to work with popular frameworks.

**Acceptance Scenarios**:

1. **Given** the "并发异步" chapter (based on `tokio_sample.rs`, `futures_sample.rs`), **When** studying examples, **Then** the reader can write async/await code, understand executors, and use Tokio runtime effectively
2. **Given** the "数据库" chapters (based on `sqlx_sample.rs`, `diesel_sample.rs`), **When** completed, **Then** the developer can connect to PostgreSQL/SQLite databases and perform CRUD operations safely
3. **Given** the "服务" chapter (based on `axum_sample.rs`, `hyper_sample.rs`), **When** reading examples, **Then** the developer can build REST APIs with proper routing and handlers
4. **Given** the "序列化" chapter (based on `json_sample.rs`, `csv_sample.rs`, `rkyv_sample.rs`), **When** studying examples, **Then** the developer can serialize/deserialize data in JSON, CSV, and zero-copy formats
5. **Given** the "测试" chapter (based on testing patterns in samples), **When** completed, **Then** the developer can write unit tests and integration tests properly
6. **Given** chapters on utilities (based on `tempfile_sample.rs`, `memmap_sample.rs`, `dotenv_sample.rs`), **When** completed, **Then** the developer can work with temporary files, memory mapping, and environment configuration

---

### User Story 3 - 使用 Awesome 框架构建生产应用 (Priority: P3)

An advanced Rust engineer learns production patterns through the `crates/awesome` framework, including dependency injection, service architecture, message queues, gRPC services, and Consul integration.

**Why this priority**: Represents the pinnacle of learning - production-ready patterns for enterprise applications with proper architecture, service lifecycle management, and external integrations.

**Independent Test**: Engineer can scaffold a new service using the awesome framework patterns and integrate with external systems (databases, message queues, service discovery). They understand the service lifecycle and dependency injection.

**Acceptance Scenarios**:

1. **Given** the "服务依赖注入" chapter (based on `crates/awesome/src/services/framework/`), **When** studying the DI implementation, **Then** the engineer can structure large applications with proper dependency management and service lifecycle
2. **Given** the "消息队列" chapter (based on `crates/awesome/src/mq/`), **When** reading MQTT examples, **Then** the developer can implement pub/sub messaging and understand message broker patterns
3. **Given** the "数据库" advanced chapter (based on `crates/awesome/src/database/`), **When** studying integrations, **Then** the developer can use modern databases including SurrealDB and implement repository patterns
4. **Given** the "模板" chapter (based on `crates/awesome/src/templates/`), **When** completed, **Then** the developer can implement template engines (Tera, Askama) for dynamic content generation
5. **Given** the "序列" chapter (based on `crates/awesome/src/sequences/`), **When** completed, **Then** the developer understands sequence generation and ID allocation patterns
6. **Given** the gRPC service examples (based on `crates/awesome/src/services/`), **When** studying tonic implementations, **Then** the developer can build production gRPC services with proper error handling

---

### User Story 4 - 解决算法挑战 (Priority: P4)

A learner practices Rust through algorithm implementations from `src/algo/` and `crates/leetcode/`, including linked lists, pi calculation, and classic LeetCode problems.

**Why this priority**: Provides hands-on practice and interview preparation. Reinforces Rust syntax and patterns through algorithmic problem-solving. Secondary to core language mastery.

**Independent Test**: Learner can implement data structures (linked list) and solve LeetCode #1 (Two Sum) and #2 (Add Two Numbers) independently. Code should be safe, efficient, and idiomatic Rust.

**Acceptance Scenarios**:

1. **Given** the "algo" chapter (based on `linked_list.rs`, `calc_pi_sample.rs`), **When** studying examples, **Then** the learner understands safe linked list implementation in Rust and numerical algorithms
2. **Given** the "LeetCode" chapter (based on `solution_0001.rs`, `solution_0002.rs`), **When** reading solutions, **Then** the learner can apply Rust to algorithmic problem-solving and understand problem-solving patterns

---

### User Story 5 - 运行 IPC 与分布式示例 (Priority: P4)

A developer explores inter-process communication patterns through the 15 binary examples in `src/bin/`, including gRPC servers/clients, Unix domain sockets, stdio communication, and process management.

**Why this priority**: Demonstrates real-world IPC patterns but is specialized knowledge. Valuable for distributed systems work and understanding process boundaries in Rust.

**Independent Test**: Developer can run gRPC client/server pairs and understand message passing between processes. They should be able to choose appropriate IPC mechanisms for different scenarios.

**Acceptance Scenarios**:

1. **Given** the gRPC examples (based on `greeter_server.rs`, `greeter_client.rs`, `grpc_hello_server.rs`, etc.), **When** running server and client, **Then** the developer can build RPC-based services with Protocol Buffers
2. **Given** the UDS examples (based on `uds_server.rs`, `uds_client.rs`, `uds_parent.rs`), **When** running Unix socket examples, **Then** the developer understands Unix domain socket communication for local IPC
3. **Given** the stdio examples (based on `stdio_parent.rs`, `stdio_child.rs`), **When** studying parent/child process communication, **Then** the developer can spawn and communicate with child processes via stdin/stdout
4. **Given** the process control examples (based on `app_sys_ctl.rs`, `app_nix_ctl.rs`), **When** studying system interaction patterns, **Then** the developer can build system administration tools in Rust
5. **Given** the gRPC store examples (based on `grpc_store_server.rs`, `grpc_store_client.rs`), **When** running key-value store examples, **Then** the developer understands stateful gRPC services

---

### Edge Cases

- **What happens when users encounter compilation errors?** Each chapter includes a "常见错误" (Common Errors) section with typical compiler errors and their solutions, extracted from actual learner mistakes
- **How does documentation handle different Rust versions?** All examples target Rust 2024 edition. Chapters clearly state minimum Rust version required (e.g., "Requires Rust 1.75+"). Migration notes provided for older editions.
- **What if users lack system dependencies (protoc for gRPC, PostgreSQL for database chapters)?** Each chapter includes "前置要求" (Prerequisites) section listing system dependencies with installation commands for macOS, Linux, and Windows
- **How are platform-specific examples handled?** UDS examples clearly marked as Unix-only. Windows alternatives documented (named pipes). Feature flags explained where platform detection is automatic
- **What if code examples break with future crate versions?** Documentation pinned to specific crate versions matching `Cargo.toml`. Upgrade notes encourage reporting breakages and provide migration guidance
- **How does documentation handle unsafe code examples?** Chapters covering unsafe Rust (`pointer_sample.rs`, advanced patterns) include clear warnings, safety invariants, and emphasize "safe interfaces to unsafe operations" pattern

## Requirements

### Functional Requirements

- **FR-001**: Documentation MUST map 1:1 with all existing `src/basic/*.rs` sample files (14 files: expression, datatype, ownership, struct, enums, traits, module, threads, closure, generic, cfg_if, pointer, logger, tracing, rectangle, visiable)
- **FR-002**: Documentation MUST map 1:1 with all existing `src/advance/*.rs` sample files (26 files: tokio, sqlx, diesel, axum, hyper, futures, json, csv, rkyv, tempfile, memmap, dotenv, bytes, cow, getset, rayon, mock, rspec, objectstore, ollama, process, sysinfo, includedir, typealias, macros, cyclerc, mio)
- **FR-003**: Documentation MUST cover all `crates/awesome/src/` module directories (services, database, sequences, mq, templates, ui, data)
- **FR-004**: Documentation MUST include executable code snippets that compile and run with `cargo run` or specific binary targets
- **FR-005**: All documentation MUST be written in Chinese (matching existing docs/src/ language convention)
- **FR-006**: Each chapter MUST include "学习目标" (Learning Objectives) section listing 3-5 specific learning outcomes
- **FR-007**: Each chapter MUST include "动手练习" (Try It Yourself) section with exercises or challenges
- **FR-008**: Documentation MUST include source code location reference (e.g., "完整示例：src/basic/ownership_sample.rs")
- **FR-009**: All existing mdBook structure (SUMMARY.md) MUST be preserved and enhanced, not restructured
- **FR-010**: Chapters MUST include "常见错误" (Troubleshooting) sections for common compilation/runtime errors
- **FR-011**: Documentation MUST include "前置知识" (Prerequisites) section listing required knowledge before each chapter
- **FR-012**: Code examples MUST preserve existing `#[cfg(test)]` test modules and encourage TDD
- **FR-013**: Documentation MUST leverage the macros crate (`crates/macros/`) to explain procedural macros
- **FR-014**: LeetCode chapter MUST document both existing solutions (0001, 0002) with problem statements and approach explanations

*Marked unclear requirements:*

- **FR-015**: Resolution: **Text-only documentation for now**. Video/screencast links will NOT be included in this phase. Focus is on high-quality written content. Video enhancement can be added as future work after documentation foundation is complete.

- **FR-016**: Resolution: **Document existing 2 LeetCode problems only**. This feature converts existing code → documentation. Currently existing: LeetCode #1 (Two Sum), #2 (Add Two Numbers). Expanding to more problems is a separate content creation feature, not a documentation conversion feature.

### Key Entities

- **教程章节 (Tutorial Chapter)**: A structured learning unit corresponding to one `.md` file in `docs/src/`. Contains learning objectives, concept explanations, code examples from source files, exercises, troubleshooting guides, and summaries. Maps to specific `.rs` sample files.
- **代码样例 (Code Sample)**: Runnable Rust source files (`src/**/*.rs`, `crates/*/*/src/*.rs`) demonstrating specific concepts. Each sample file is the basis for one or more tutorial chapters.
- **学习路径 (Learning Path)**: Ordered sequence of chapters following the mdBook hierarchy: Basic (基础入门) → Advance (高级进阶) → Awesome (精选实战) → Algo/LeetCode. Represents progressive skill development.
- **mdBook 结构 (mdBook Structure)**: `SUMMARY.md` defines navigation hierarchy with 4 main sections and nested subsections. Documentation must align with this existing structure.

## Success Criteria

### Measurable Outcomes

- **SC-001**: 100% of sample files in `src/basic/` (14 files) have corresponding detailed documentation chapters (minimum 500 Chinese characters each)
- **SC-002**: 100% of sample files in `src/advance/` (26 files) have corresponding detailed documentation chapters (minimum 500 Chinese characters each)
- **SC-003**: All currently empty/minimal documentation files (20+ files with 1 line or <50 characters: async.md, database.md, services.md, basic-overview.md, advance-overview.md, awesome-overview.md, struct-fields.md, struct-methods.md, enums.md, etc.) expanded to substantive content
- **SC-004**: Documentation covers 100% of awesome crate modules (7 directories: services, database, sequences, mq, templates, ui, data) with at least one chapter per directory
- **SC-005**: All code examples in documentation compile successfully with `cargo build --workspace` (zero compilation errors)
- **SC-006**: mdBook builds successfully with `cd docs && mdbook build` (zero warnings, clean build)
- **SC-007**: Each chapter includes at least 2 executable code snippets with explanations and 1 troubleshooting tip
- **SC-008**: All 15 binary examples in `src/bin/` documented across relevant chapters (gRPC, UDS, stdio, process control)
- **SC-009**: Documentation includes all source locations (src/, crates/, crates/awesome/src/, etc.) proportionally to code volume

## Assumptions

- **Target audience has basic programming knowledge** (variables, functions, control flow, basic algorithms) but may be completely new to Rust and systems programming
- **Existing code samples are functional but may contain unsafe patterns for educational purposes** - documentation will explicitly warn about unsafe code (`static mut`, `transmute`, `from_utf8_unchecked`, `std::sync::Mutex` in async context) and provide safe alternatives where appropriate. Documentation focuses on explaining patterns while highlighting best practices.
- **Primary language: Chinese (简体中文)** - All documentation will be written in Chinese first, targeting Chinese-speaking Rust learners
- **Future English translation planned** - After Chinese version is complete, English translation will be created as a separate phase (not included in this feature scope)
- **Rust 2024 edition** is used throughout the project (as per `Cargo.toml` crate edition)
- **System dependencies available or installable**: protoc 29.3+ for gRPC chapters, PostgreSQL for database chapters, system tools for UDS examples
- **Existing mdBook structure in SUMMARY.md** represents the desired final organization - documentation fills gaps but doesn't restructure
- **Focus is on pedagogy and explanation** over adding new code examples or modifying existing samples
- **LeetCode and algo sections** document only existing solutions (2 LeetCode problems + 2 algo implementations) as teaching examples
- **Documentation should be practical** - readers should be able to clone repo, run `cargo run`, and follow along with examples
- **Binary examples in src/bin/** are documented as part of relevant chapters (gRPC, IPC, processes) rather than as separate standalone sections
- **Workspace crates** (awesome, leetcode, macros) are treated as advanced topics after mastering basics and intermediate concepts
- **Pre-built documentation**: `docs/book/` is gitignored (generated content only) - source `.md` files in `docs/src/` are authoritative and deployed to GitHub Pages via CI/CD pipeline

## Documentation Writing Principles

### Core Philosophy: Avoid the Curse of Knowledge

The "curse of knowledge" cognitive trap occurs when writers forget what it's like to be a beginner. Documentation must be written from the perspective of someone who has NOT yet learned Rust, not from the perspective of an expert explaining to other experts.

**Research-Backed Foundation**: These principles synthesize:
- **Diataxis Framework** - Four documentation types: tutorials (learning-oriented), how-to guides (goal-oriented), reference (information-oriented), explanation (understanding-oriented)
- **Cognitive Load Theory** - Working memory is limited; documentation must manage intrinsic, extraneous, and germane load
- **Industry Best Practices 2025** - User-centric, docs-as-code, version-controlled, feedback-driven documentation

### Language Strategy: Chinese First, English Later

**Phase 1: Chinese Documentation (Current Scope)**
- All chapters written in Simplified Chinese (简体中文)
- Technical terms in English with Chinese explanations on first use
- Code comments and examples remain in English (Rust convention)
- Target audience: Chinese-speaking developers learning Rust

**Phase 2: English Translation (Future Scope - NOT included in this feature)**
- After Chinese version complete, create English translations
- Maintain consistency between language versions
- Potentially use AI-assisted translation with human review
- Separate feature branch and planning required

**Why Chinese First:**
- Existing docs/src/ structure is primarily Chinese
- Target audience is Chinese Rust learners
- Consistent with project's current documentation style
- English can be added later without restructuring

### Writing Principles (Chinese Language)

**Principle 1: Write for the "Almost Beginner" (零基础友好)**
- Assume reader has some programming knowledge (variables, loops, functions) but zero Rust experience
- Never assume familiarity with Rust-specific concepts (ownership, borrowing, lifetimes)
- Explain every new term the first time it appears
- Use analogies to concepts from other languages (Python, JavaScript, Java) when possible
- Include "本章适合谁 (Who This Is For)" section at chapter start so readers can self-qualify

**Principle 2: Simple Language Over Technical Jargon (平实的语言)**
- Use plain Chinese (平实的语言) instead of academic or overly formal language
- Prefer "你存储数据到结构体" over "结构体是聚合值的复合数据类型"
- Avoid phrases like "trivially", "obviously", "simply" - nothing is obvious to a learner
- 避免使用"显然"、"简单"、"只需"等词汇 - 对学习者来说没有什么是一目了然的
- Define technical terms in parentheses on first use: "所有权 (ownership) - Rust 的核心概念，控制资源使用的规则"
- Use present tense and active voice: "更新配置文件" not "配置文件应该被更新"
- Write short sentences and paragraphs (max 2-3 sentences per paragraph for screen readability)

**Principle 3: Knowledge Stories and Analogies (小知识故事)**
- Each abstract concept should have a concrete real-world analogy
- Example: Ownership = "就像借图书馆的书 - 一次只能借给一个人"
- Example: Borrowing = "就像把你的书借给朋友 - 你还是拥有者，但他们可以阅读"
- Example: Lifetimes = "就像图书馆的还书日期 - 告诉你借的书可以保留多久"
- Use short stories (小知识故事) to illustrate concepts before diving into code
- Stories should be 2-4 sentences, culturally relevant, and memorable
- Chinese cultural context preferred for analogies

**Principle 4: Progressive Disclosure (渐进式披露)**
- Start with the simplest working example, then add complexity
- Never show the "complete" version first
- Example progression: 简单变量 → 类型注解 → 函数调用 → 结构体中的变量
- Each step should build on the previous one, not introduce multiple new concepts simultaneously
- Use layered approach: 概述 → 基础教程 → 进阶指南 → 参考

**Principle 5: Mistake-Driven Learning (错误驱动学习)**
- Show common errors BEFORE they happen
- Include "什么不该做 (What NOT to do)" examples alongside correct code
- Example: Show the compilation error for borrowing violation, then explain WHY Rust prevents it
- Use actual Rust compiler error messages (with Chinese explanations)
- Organize troubleshooting by symptom: "遇到这个错误？→ 这样解决"
- Include FAQ sections sourced from real learner questions

**Principle 6: Concrete Before Abstract (具体先于抽象)**
- Always show working code first, then explain the concept
- Never start with theory - start with "this code works" then "here's why"
- Example order: 1) 可运行的代码示例 2) 运行结果 3) 原理解释 4) 变体和边界情况
- Provide copy-paste-ready code examples that are tested and validated
- Show both success cases AND error cases (what 200 OK looks like vs 401 Unauthorized)

**Principle 7: Bilingual Terminology (双语术语对照)**
- First occurrence: "所有权 (ownership)" - Chinese with English in parentheses
- Subsequent occurrences: Just "所有权"
- Technical terms maintain English for searchability and reference
- Create glossary section: 术语表 (Glossary) with Chinese ↔ English mappings
- Consistent terminology across all chapters (no synonyms)

**Principle 8: Structured Consistency (Diataxis-Inspired) (结构一致性)**
- Each chapter follows the same predictable structure so learners know where to find information
- Templates ensure no critical sections are missed
- Consistent formatting reduces cognitive load - readers focus on content, not navigation
- Use standard templates for different content types:
  - **Tutorial chapters** (学习导向): 目标 → 前置要求 → 逐步指导 → 总结
  - **How-to chapters** (目标导向): 问题 → 解决方案 → 步骤 → 故障排查
  - **Reference chapters** (信息导向): API → 参数 → 示例 → 错误
  - **Explanation chapters** (理解导向): 概念 → 为什么重要 → 如何工作 → 替代方案

### Writing Guidelines (Applied to Each Chapter)

Each chapter MUST follow this structure:

1. **开篇故事 (Opening Story)** - 2-4 句话的类比或现实场景，设置上下文
2. **本章适合谁 (Who This Is For)** - 1-2 句帮助读者自我评估是否适合本章
3. **你会学到什么 (What You'll Learn)** - 3-5 个具体的学习目标，使用平实语言
4. **前置要求 (Prerequisites)** - 学习本章前需要的知识/技能 (链接到前置章节)
5. **第一个例子 (First Example)** - 最小的可运行代码 (<15 行) 演示概念
6. **原理解析 (How It Works)** - 使用简单语言和类比的解释，每段一个概念
7. **常见错误 (Common Mistakes)** - 2-3 个学习者常犯的错误，包含编译器输出和解决方案
8. **动手练习 (Try It Yourself)** - 2-3 个小挑战，难度递增，强化概念
9. **故障排查 (Troubleshooting/FAQ)** - 基于症状的常见问题解决方案
10. **知识扩展 (Going Deeper)** - 选学的高级提示 (明确标记为可选)
11. **小结 (Summary)** - 3-5 个要点总结关键收获
12. **术语表 (Glossary)** - 本章涉及的技术术语中英文对照 (选学，复杂章节需要)

### Tone and Voice (Chinese Language)

**语气基调:**
- **Supportive and encouraging (鼓励性)** - "你可能会遇到这个错误 - 这很正常！" "恭喜你迈出了第一步！"
- **Never condescending (不要说教)** - 避免"如你所知"、"显然"、"简单"、"只需"、"easy"
- **Acknowledge difficulty (认可难度)** - "Rust 的所有权系统与你的习惯不同 - 这很正常，需要练习" "这个概念很有挑战性 - 很多学习者在这里遇到困难"
- **Use "we" language (使用"我们")** - "让我们运行这段代码看看会发生什么" 而不是"你应该运行这个"
- **Celebrate small wins (庆祝小进步)** - "太棒了！你已经成功编译了第一个 Rust 程序！" "现在你理解了所有权 - 这是一个重要的里程碑！"
- **Be specific and direct (具体直接)** - "运行 `cargo build`" 而不是"你可以尝试构建一下"
- **Use "你可以" for options (选项用"你可以")** - "你也可以试试..." 用于可选路径，不是必需步骤

**Chinese Writing Best Practices:**
- 使用第二人称"你"而不是"用户"或"开发者"
- 避免过长的复合句，每句控制在 20 字以内
- 技术术语首次出现：中文 (英文) 格式，如"所有权 (ownership)"
- 代码中的变量名、函数名保持英文 (Rust 惯例)
- 使用中文标点符号 (,.)
- 避免中英混杂的句子结构
- 段落间使用空行分隔

### Cognitive Load Management (认知负荷管理)

- **One concept per paragraph (每段一个概念)** - 永远不要在同一段解释所有权和借用
- **Code examples under 15 lines (代码示例≤15 行)** - 将复杂示例分解成更小、更专注的片段
- **Visual separation (视觉分隔)** - 使用水平线 (---) 分隔主要章节
- **Progressive complexity (渐进复杂度)** - 每个示例只比前一个示例多一个概念
- **Summaries at every level (各级总结)** - 章节总结、部分总结、全书总结
- **Use headings liberally (多用标题)** - 每 200-300 字使用描述性标题
- **White space is your friend (留白是朋友)** - 不要创建文字墙;将内容分成易消化的块
- **Chinese paragraph length (段落长度)** - 每段 2-4 句话，避免长段落

### Documentation Quality Standards (文档质量标准)

**Working Code Guarantee (可运行代码保证):**
- ALL code examples MUST compile with `cargo build --workspace`
- Code examples are tested in CI/CD pipeline (future enhancement)
- Each example includes expected output (预期输出)
- Version information clearly stated (Rust 2024 edition, crate versions)

**Version Control (版本控制):**
- Documentation lives with code in same repository
- Each chapter has "Last Updated" timestamp (最后更新时间)
- Chapter changes tracked in git alongside code changes
- Version-specific notes for features that changed across editions

**Visual Aids (可视化辅助):**
- Use ASCII diagrams for data flow, state machines, and architecture
- Annotated code examples (comments explaining key lines)
- Tables for comparing options (e.g., String vs &str, Vec vs Array)
- Clear visual hierarchy with heading levels (H1 → H2 → H3)

**Feedback Mechanisms (反馈机制):**
- Each chapter ends with "Was this helpful?" section (future enhancement)
- Encourage readers to report unclear sections via GitHub issues
- Regular reviews based on learner feedback and support questions

**Bilingual Support (未来双语支持):**
- Chinese version is authoritative (当前中文版本是权威)
- English translation planned as separate phase (英文版计划为独立阶段)
- Maintain structure consistency across languages (跨语言保持结构一致)
- Terminology glossary: 中文 ↔ English mappings (术语表中英文对照)

### Cross-Reference Strategy (交叉引用策略)

**Internal Linking (内部链接):**
- Link to prerequisite chapters: "继续之前，请先阅读 [了解所有权](ownership.md)"
- Link to related concepts: "这个模式在 [特征](trait.md) 中有 further 扩展"
- Link to reference material: "完整 API 文档请看 [std::String](https://doc.rust-lang.org/std/string/struct.String.html)"
- Use consistent link format throughout (保持链接格式一致)

**External References (外部引用):**
- Link to official Rust Book (Rust 指南中文版) for complementary explanations
- Link to Rust by Example for additional practice problems
- Link to crate documentation (docs.rs) for API reference
- Link to Rust user forums (Rust 用户论坛) for community support
- Link to Chinese Rust community resources (中国 Rust 社区资源)

**Code File References (代码文件引用):**
- Each chapter references the source sample file: "完整示例：src/basic/ownership_sample.rs"
- Encourage readers to read both docs and source code (鼓励读者同时阅读文档和源代码)
- Show file path structure for navigation (显示文件路径结构便于导航)

---

## 代码示例规范

### 原则

**所有文档代码示例必须**：
1. ✅ 来自项目实际实现的代码
2. ✅ 提供可访问的 GitHub 链接
3. ✅ 标记代码在源码中的位置
4. ✅ 可独立编译运行

### GitHub 链接格式

```markdown
完整源码：[src/basic/ownership_sample.rs](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs)
```

**链接结构**：
```
https://github.com/savechina/hello-rust/blob/<commit-hash>/<path-to-file>
```

**当前版本**：
- Commit: `81f7441a4d9dc7912cc18d4170077653655d335d`
- Branch: `main` (or `001-rust-tutorial-docs` for work-in-progress)

### 代码示例标注

每章末尾必须包含：

```markdown
---

## 源码位置

- **基础示例**: [`src/basic/ownership_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs)
- **完整实现**: [`crates/awesome/src/services/`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/)
- **测试代码**: [`src/basic/ownership_sample.rs#L89-L110`](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs#L89-L110) (测试模块)
```

### 代码映射表

每个章节对应的源码文件：

| 章节 | 源码文件 | GitHub 链接 |
|------|---------|------------|
| 变量与表达式 | `src/basic/expression_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/expression_sample.rs) |
| 所有权 | `src/basic/ownership_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs) |
| 数据类型 | `src/basic/datatype_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/datatype_sample.rs) |
| 结构体 | `src/basic/rectangle.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/rectangle.rs) |
| 枚举 | `src/basic/traits_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/traits_sample.rs) |
| 特征 | `src/basic/traits_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/traits_sample.rs) |
| 模块 | `src/basic/module_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/module_sample.rs) |
| 泛型 | `src/basic/generic_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/generic_sample.rs) |
| 闭包 | `src/basic/closure_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/closure_sample.rs) |
| 线程 | `src/basic/threads_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/threads_sample.rs) |
| 指针 | `src/basic/pointer_sample.rs` | [View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/pointer_sample.rs) |

### 禁止的做法

❌ **不要使用虚构的代码**：
```rust
// 错误示例 - 项目中没有的代码
struct FakeExample {
    foo: String,  // 这不是项目中的真实代码
}
```

❌ **不要提供死链接**：
```markdown
// 错误 - 链接到不存在的文件
[View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/fake_file.rs)
```

### 验证检查清单

编写每个章节时，确保：

- [ ] 代码示例可以在项目中找到对应源码
- [ ] GitHub 链接可以访问（404 检查）
- [ ] 链接包含 commit hash 或branch 名
- [ ] 代码片段保持原样，不随意修改
- [ ] 如果简化代码，注明"为简洁省略..."
- [ ] 提供完整的源码路径

### 示例章节格式

```markdown
# 章节名称

## 开篇故事
...

## 第一个例子

```rust
// src/basic/ownership_sample.rs
fn gives_ownership() -> String {
    let some = String::from("hello");
    some
}
```

**完整源码**：[View on GitHub](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs#L35-L45)

---

## 源码位置

- **本章源码**: [`src/basic/ownership_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/ownership_sample.rs)
- **相关源码**: 
  - [`src/basic/datatype_sample.rs`](https://github.com/savechina/hello-rust/blob/main/src/basic/datatype_sample.rs) (数据类型)
  - [`crates/awesome/src/services/`](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/) (高级示例)
```

---

**目标**：读者可以边看文档，边在 GitHub 上查看真实源码，学完就能用！

---

## Extended Features (Based on Competitive Analysis)

### Knowledge Enhancement Features

**Based on**: The Rust Book, Rust By Example, Rusty Book best practices

#### Extended Library References

**Requirement**: Each chapter MUST include "延伸阅读" (Extended Reading) section:

```markdown
## 延伸阅读：更多序列化库

学习完本章的 `serde_json`，你可能还想了解：

- [`rkyv`](https://github.com/rkyv/rkyv) - 零拷贝序列化，性能极佳
- [`bincode`](https://github.com/bincode-org/bincode) - 紧凑二进制格式
- [`simd-json`](https://github.com/simd-lite/simd-json) - SIMD 加速 JSON 解析

**选择建议**:
- 追求性能 → rkyv
- 追求兼容 → serde_json
- 追求体积 → bincode
```

**Purpose**:
- Expose learners to ecosystem beyond project code
- Enable comparison and informed decision-making
- Bridge gap between learning and real-world library selection

#### Code Snippet Quick Reference

**Requirement**: Create "代码片段速查" (Code Snippets Quick Reference) page:

```markdown
# 常用操作速查

## 文件操作

// 读取整个文件
let content = std::fs::read_to_string("file.txt")?;

// 写入文件
std::fs::write("output.txt", "data")?;

// 逐行读取
for line in std::fs::read_lines("file.txt")? {
    println!("{}", line?);
}

// 复制文件
std::fs::copy("src.txt", "dst.txt")?;

## 集合操作

// Vector 排序
let mut nums = vec![3, 1, 4, 1, 5];
nums.sort();

// HashMap 计数
use std::collections::HashMap;
let mut counts = HashMap::new();
for word in words {
    *counts.entry(word).or_insert(0) += 1;
}
```

**Location**: `docs/src/quick_reference/snippets.md`

**Purpose**:
- Enable copy-paste for common operations
- Provide quick lookup during development
- Complement deep learning with practical utility

### Visual Learning Aids

**Based on**: Rust Book visualizations, cognitive load research

#### ASCII Diagrams for Key Concepts

**Requirement**: Each chapter MUST include ASCII diagrams for:

1. **Memory Layout** (所有权，borrowing):
```
栈 (Stack)              堆 (Heap)
+-----------+          +----------------+
| pointer   |--------->| "hello"        |
| length: 5 |          |                |
| capacity: 5|         +----------------+
+-----------+
```

2. **Ownership Transfer** (移动语义):
```
移动前:
s1 ──→ [堆内存："hello"]

s1 = s2 后:
s1      [堆内存："hello"] ←── s2
  ❌         ↑              ✅
  无效      owned by s2
```

3. **Lifetimes** (生命周期):
```
'a: [==========]
      b:    [====]
      
fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
where 'b: 'a  // b 存活时间 >= a
```

**Format Requirements**:
- Use code blocks with `text` language
- Include Chinese annotations
- Keep diagrams under 20 lines
- Reference in chapter text ("如图 X 所示")

#### Learning Path Skill Tree

**Requirement**: Create visual skill tree at `docs/src/learning_path.md`:

```markdown
# Rust 学习技能树

## 基础 (第 1-5 章) 🟢
- [x] 变量与表达式 ✅
- [x] 数据类型 ✅
- [x] 所有权系统 ✅
- [x] 结构体 ✅
- [x] 枚举 ✅
      ↓
## 进阶 (第 6-10 章) 🟡
- [ ] 特征 ⏳
- [ ] 泛型 ⏳
- [ ] 闭包 ⏳
- [ ] 模块 ⏳
- [ ] 错误处理 ⏳
      ↓
## 高级 (第 11-15 章) 🔴
- [ ] 线程与并发 ⏳
- [ ] 异步编程 ⏳
- [ ] 宏 ⏳
- [ ] Unsafe ⏳
- [ ] FFI ⏳

**当前进度**: 30% (5/15 章完成)
```

**Features**:
- Visual progress tracking
- Difficulty levels (🟢🟡🔴)
- Prerequisites clearly marked
- Estimated completion time per section

### Project-Based Learning

**Based on**: The Rust Book "Guessing Game", project-based learning research

#### Graduated Projects

**Requirement**: Implement 3 graduated projects:

**Project 1: 命令行待办事项** (After Chapter 5)
```markdown
**目标**: 实现支持添加、列表、完成、删除的 TODO 工具

**技能点**:
- 变量与数据类型 (第 1 章)
- 所有权与借用 (第 2 章)
- 结构体与枚举 (第 3-4 章)
- Vec 集合操作 (第 5 章)

**功能**:
- `todo add "Buy milk"` - 添加任务
- `todo list` - 显示所有任务
- `todo done 1` - 标记任务完成
- `todo delete 1` - 删除任务
- 保存到 JSON 文件

**代码量**: ~200 行
**难度**: 🟢 入门

**源码**: `projects/todo-cli/`
```

**Project 2: 简易 HTTP 服务器** (After Chapter 10)
```markdown
**目标**: 不使用框架，用标准库实现 HTTP 服务器

**技能点**:
- 所有权系统 (复习)
- 特征与泛型 (第 6 章)
- 错误处理 (第 7 章)
- TCP/IO (第 8 章)
- 线程基础 (第 9 章)

**功能**:
- 监听指定端口
- 处理 GET/POST 请求
- 返回静态文件
- 简单路由
- 日志记录

**代码量**: ~500 行
**难度**: 🟡 中级

**源码**: `projects/http-server/`
```

**Project 3: 多线程爬虫** (After Chapter 15)
```markdown
**目标**: 并发爬取网页提取数据

**技能点**:
- 线程与并发 (第 11 章)
- 通道消息传递 (第 12 章)
- Arc/Mutex (第 13 章)
- 异步编程 (第 14 章)
- HTML 解析 (自选库)

**功能**:
- 并发爬取多个 URL
- 提取标题和链接
- 保存到 CSV/数据库
- 速率限制
- 错误恢复

**代码量**: ~1000 行
**难度**: 🔴 高级

**源码**: `projects/web-scraper/`
```

#### Project Checkpoints

**Requirement**: Each project MUST include:

```markdown
## 检查点

### 检查点 1：基础框架 (20 行代码)
- [ ] 创建项目结构
- [ ] 实现 CLI 解析
- [ ] 添加空命令处理

### 检查点 2：核心功能 (100 行代码)
- [ ] 实现添加任务
- [ ] 实现列表显示
- [ ] 添加错误处理

### 检查点 3：数据持久化 (50 行代码)
- [ ] 保存到 JSON 文件
- [ ] 启动时加载
- [ ] 处理文件不存在

### 检查点 4：完善功能 (30 行代码)
- [ ] 实现删除任务
- [ ] 添加任务完成标记
- [ ] 改进输出格式
```

**Purpose**:
- Break large projects into manageable chunks
- Provide clear completion criteria
- Enable incremental testing

### Knowledge Checkpoints

**Based on**: Cognitive science, spaced repetition research

#### Chapter-end Quizzes

**Requirement**: Each chapter MUST include "知识检查" section:

```markdown
## 知识检查

**快速测验**（答案在下方）：

1. 这段代码会编译通过吗？为什么？
   ```rust
   let s1 = String::from("hello");
   let s2 = s1;
   println!("{}", s1);
   ```

2. 所有权转移后，原变量还能用吗？

3. 如何在不转移所有权的情况下借用变量？

<details>
<summary>点击查看答案与解析</summary>

1. ❌ 不会通过 - s1 的所有权已移动给 s2
2. 不能 - 会导致编译错误 (`borrow after move`)
3. 使用引用：`&s1`（不可变借用）或`&mut s1`（可变借用）

**关键理解**:
所有权转移后，原变量失效。这是 Rust 防止内存错误的核心机制。
</details>
```

**Question Types**:
- 预测编译结果 (40%)
- 选择正确答案 (30%)
- 修复错误代码 (20%)
- 简答概念 (10%)

**Difficulty Distribution**:
- 🟢 Easy: 直接应用概念 (50%)
- 🟡 Medium: 组合多个概念 (40%)
- 🔴 Hard: 边界情况/陷阱 (10%)

#### Spaced Repetition Review

**Requirement**: Create review sections at learning milestones:

```markdown
# 阶段复习 (第 5 章后)

**复习范围**: 第 1-5 章（变量、数据类型、所有权、结构体、枚举）

## 综合练习：设计一个简单的游戏角色系统

1. 用结构体表示角色（属性：名字、生命值、等级、职业）
2. 用枚举表示职业（战士、法师、游侠）
3. 实现以下功能：
   - 创建角色
   - 显示角色信息
   - 角色升级（等级 +1，生命值 +10）
   - 角色受伤（生命值减少）
   - 检查角色是否存活

**提示**:
- 使用可变变量存储角色状态
- 使用 match 匹配不同职业
- 注意所有权转移
- 考虑使用引用来避免不必要的移动

**挑战**（选做）:
- 添加角色对战功能
- 使用 Vec 存储多个角色
- 保存到 JSON 文件
```

### Community Contribution Framework

**Based on**: Rusty Book community model

#### Contribution Guidelines

**Requirement**: Create `docs/CONTRIBUTING.md`:

```markdown
# 贡献指南

## 如何改进文档

### 1. 发现错误
- 提交 Issue，描述错误内容
- 提供错误位置和复现方式
- 建议修复方案（如有）

### 2. 改进示例
- 提交 PR，包含改进代码
- 确保代码可编译运行
- 解释改进理由

### 3. 补充知识点
- 提交"小知识"PR
- 符合章节主题
- 提供真实案例或类比

### 4. 翻译工作
- 加入翻译组
- 负责英文章节翻译
- 保持术语一致性

## 贡献者权益

- GitHub 贡献者榜单
- Rust 中文社区推荐
- 优秀贡献者获得"锈书贡献者"徽章
- 年度优秀贡献者奖励

## 贡献标准

### 代码要求
- [ ] 代码可编译运行
- [ ] 通过`cargo build --workspace`
- [ ] 无`clippy`警告
- [ ] 包含测试（如适用）

### 文档要求
- [ ] 解释清晰准确
- [ ] 符合文档风格
- [ ] 包含中英文术语
- [ ] 提供代码示例

### 审核流程
1. 提交 PR
2. CI 自动检查
3. 维护者审核（48 小时内）
4. 反馈修改或合并
```

#### Quality Audit Checklist

**Requirement**: Create `docs/QUALITY_AUDIT.md`:

```markdown
# 内容审核清单

## 发布前检查

每个章节发布前必须通过以下检查：

### 基础检查
- [ ] 代码可编译运行 (`cargo build`)
- [ ] 示例来自真实项目
- [ ] GitHub 链接有效且指向正确文件
- [ ] 无拼写/语法错误

### 教学检查
- [ ] 包含至少 2 个常见错误
- [ ] 包含至少 3 个练习题
- [ ] 有"知识检查"小节
- [ ] 有"工业界应用"小节

### 技术检查
- [ ] 术语有中英文对照
- [ ] 难度标记准确 (🟢🟡🔴)
- [ ] 前置章节链接正确
- [ ] unsafe 代码有明确警告

### 视觉检查
- [ ] 关键概念有 ASCII 图表
- [ ] 代码格式正确 (rustfmt)
- [ ] 标题层级清晰
- [ ] 链接无 404

## 定期复审

每季度复审所有章节：
- [ ] 代码仍然可运行
- [ ] Crate 版本仍然兼容
- [ ] 链接仍然有效
- [ ] 无过时内容

## 质量指标

- 编译通过率：100%
- 链接有效率：>98%
- 练习覆盖率：每章≥3 个
- 图表覆盖率：关键概念 100%
```

---

## Updated Success Criteria

### Additional Measurable Outcomes

**SC-010**: Each chapter MUST include "延伸阅读" section with at least 3 related libraries and selection guidelines

**SC-011**: Documentation MUST include "代码片段速查" page covering at least 10 common operation categories

**SC-012**: Key concepts (ownership, borrowing, lifetimes) MUST have ASCII diagrams with Chinese annotations

**SC-013**: Learning path skill tree MUST visualize all 15+ chapters with difficulty levels and progress tracking

**SC-014**: Documentation MUST include 3 graduated projects (200/500/1000 lines) with clear checkpoints

**SC-015**: Each chapter MUST include "知识检查" section with at least 3 quiz questions and answers

**SC-016**: `docs/CONTRIBUTING.md` MUST be created with contribution guidelines and quality audit checklist

**SC-017**: Chapter difficulty levels (🟢🟡🔴) MUST be marked consistently across all chapters

**SC-018**: Cross-chapter review sections MUST be provided after every 5 chapters for spaced repetition


---

## 补充：项目实战部分

### 实战项目应该使用项目实际样例

根据用户反馈，教程的"项目实战"部分应该直接使用项目中已有的工程代码作为教学示例，而不是创建虚构的项目。

**项目中已有的样例工程**：

#### 基础部分项目 (Basic Projects)

1. **Hello Rust 基础演示** (`src/bin/basic.rs`)
   - 代码量：~15 行
   - 难度：🟢 入门
   - 学习目标：运行第一个 Rust 程序

2. **进阶演示** (`src/bin/advance.rs`)
   - 代码量：~60 行
   - 难度：🟡 中级
   - 学习目标：理解基本与进阶概念

#### 高级部分项目 (Advanced Projects)

3. **gRPC 服务器** (`src/bin/greeter_server.rs`)
   - 代码量：~80 行
   - 难度：🟡 中级
   - 学习目标：gRPC 服务实现
   - 相关章节：[gRPC 服务](grpc.md)

4. **gRPC 客户端** (`src/bin/greeter_client.rs`)
   - 代码量：~50 行
   - 难度：🟡 中级
   - 学习目标：gRPC 客户端调用
   - 相关章节：[gRPC 客户端](grpc-client.md)

5. **Unix Domain Socket 服务器** (`src/bin/uds_server.rs`)
   - 代码量：~30 行
   - 难度：🟡 中级
   - 学习目标：IPC 通信

6. **Unix Domain Socket 客户端** (`src/bin/uds_client.rs`)
   - 代码量：~25 行
   - 难度：🟢 入门
   - 学习目标：IPC 客户端

7. **Stdio IPC 父进程** (`src/bin/stdio_parent.rs`)
   - 代码量：~25 行
   - 难度：🟡 中级
   - 学习目标：标准输入输出 IPC

8. **Stdio IPC 子进程** (`src/bin/stdio_child.rs`)
   - 代码量：~10 行
   - 难度：🟢 入门
   - 学习目标：子进程创建

#### 算法项目 (Algo Projects)

9. **PI 计算** (`src/algo/calc_pi_sample.rs`)
   - 代码量：~100 行
   - 难度：🟡 中级
   - 学习目标：数值算法、并行计算
   - 相关章节：[算法实战](algo-pi.md)

10. **LeetCode 题解** (`crates/leetcode/`)
    - #1: 两数之和 → 哈希表
    - #2: 两数相加 → 链表
    - 难度：🟢 入门
    - 学习目标：数据结构与算法

#### 框架实战 (Awesome Framework)

11. **服务框架** (`crates/awesome/src/services/`)
    - 代码量：~500 行
    - 难度：🔴 高级
    - 学习目标：生产级服务框架

12. **数据库操作** (`crates/awesome/src/database/`)
    - 代码量：~300 行
    - 难度：🟡 中级
    - 学习目标：数据库集成

### 修订后的成功标准

**SC-019【新增】**: 教程项目部分 MUST 使用项目中已有的实际样例工程

- Basic chapters → `src/bin/basic.rs`, `src/bin/advance.rs`
- IPC chapters → `src/bin/`, `src/bin/uds_*`, `src/bin/stdio_*`
- Algo chapters → `src/algo/calc_pi_sample.rs`, `crates/leetcode/`
- Framework chapters → `crates/awesome/src/`

**删除** SC-014（3 个虚构项目）的强制要求，替换为：

"Documentation SHOULD organize teaching around existing project samples, with clear runnable instructions and checkpoints for each binary example."

**新增任务**：

T157: 创建"项目实战指南"页面 → `docs/src/projects/README.md`
T158: 为每个 binary 样例创建学习指南 → `docs/src/projects/binaries/`
T159: 创建算法题解指南 → `docs/src/projects/algorithms/`
T160: 创建框架使用指南 → `docs/src/projects/awesome/`

**要求**：
每个项目指南包含：
- 运行说明 (`cargo run --bin xxx`)
- 代码讲解（链接到对应章节）
- 修改建议（"试着改一下..."）
- 扩展练习

---

## 更新日志：项目实战部分

**日期**: 2026-04-04  
**范围**: 使用项目真实样例代替虚构项目

### 变更内容

1. **创建 docs/src/projects/README.md**
   - 列出 8 个实际项目示例
   - 提供运行说明和学习目标
   - 链接到相关章节

2. **创建 docs/src/learning_path.md**
   - 技能树可视化
   - 进度追踪
   - 学习建议

3. **更新 SC-014 (Project Success Criteria)**
   - 从"创建 3 个虚构项目"改为"文档化项目实际样例"
   - 新增 SC-019: 实战项目应该使用项目实际存在的代码
   - 删除 T127-T145 (虚构项目实现任务)
   - 新增 T157-T160 (项目指南编写任务)

4. **完成状态**
   - ✅ T126: 技能树创建 (`docs/src/learning_path.md` - 170+ 行)
   - ✅ T157: 项目实战指南 (`docs/src/projects/README.md` - 300+ 行)
   - 已完成使用项目真实样例

### 项目示例列表

| 项目 | 文件路径 | 代码量 | 难度 | 状态 |
|------|----------|--------|------|------|
| Hello Rust 基础 | `src/bin/basic.rs` | ~15 行 | 🟢 | ✅ 已文档化 |
| gRPC 服务器 | `src/bin/greeter_server.rs` | ~100 行 | 🟡 | ✅ 已文档化 |
| gRPC 客户端 | `src/bin/greeter_client.rs` | ~50 行 | 🟡 | ✅ 已文档化 |
| UDS IPC | `src/bin/uds_*.rs` | ~60 行 | 🟡 | ✅ 已文档化 |
| Stdio IPC | `src/bin/stdio_*.rs` | ~40 行 | 🟡 | ✅ 已文档化 |
| PI 计算 | `src/algo/calc_pi_sample.rs` | ~100 行 | 🟡 | ✅ 已文档化 |
| LeetCode 题解 | `crates/leetcode/` | ~50 行/题 | 🟢 | ✅ 已文档化 |
| 框架实战 | `crates/awesome/src/` | ~2000 行 | 🔴 | ✅ 已文档化 |

### Wave 1 + Wave 4 完成统计

**总计**:
- 文档章节：19/19 基础章节 ✅
- 特色功能：技能树、项目指南 ✅
- 新增文档：~500+ 行
- 总代码行数：~20K+ 行

**完成列表**:
- [x] T018: enums.md (6.3K)
- [x] T019: trait.md (4.4K)
- [x] T020: module.md (2.6K)
- [x] T021: generic.md (8.5K)
- [x] T022: closure.md (7.3K)
- [x] T026: logger.md (5.5K)
- [x] T027: tracing.md (5.5K)
- [x] T025: pointer.md (1.9K)
- [x] T024: cfg_if.md (1.1K)
- [x] T023: threads.md (2.3K)
- [x] T028: visiable.md (6.8K)
- [x] T126: learning_path.md (6.3K)
- [x] T157: projects/README.md (5K)

**Wave 1 完成率**: 11/11 章节 = **100%** ✅

---

**下一个步骤**:
- Wave 2: ASCII diagram (T123-T125) - 待执行
- Wave 3: 知识检查 (T146-T148) - 待执行
- mdBook 构建：待验证
- Git 提交：待执行
