## Phase 0: Research & Discovery

### Research Findings

**Sample File Quality Assessment**:

**✅ HIGH QUALITY TEACHING SAMPLES**:
- `ownership_sample.rs` - Excellent ownership/borrowing demonstrations
  - Clear move semantics examples
  - Commented error cases (dangling pointers prevention)
  - Lifetime annotation examples
  - Recommendation: Use as primary teaching example
  
- `datatype_sample.rs` (1017 lines) - Comprehensive data types
  - String handling with both `&str` and `String`
  - Collections (Vec, HashMap, BTreeMap, etc.)
  - Date/time with chrono integration
  - BigDecimal for precision math
  - Caution: Large file - split into multiple focused chapters

- `tokio_sample.rs` (509 lines) - Async TCP server/client
  - Complete working async code
  - Proper error handling
  - Recommendation: Split into "Async Basics" + "Network Programming" chapters

**⚠️ SAMPLES REQUIRING CONTEXT**:
- `dynmaic_injection_box_sample.rs` - DI patterns
  - Typo in filename ("dynmaic" → "dynamic")
  - Clean trait architecture
  - Documentation should: Acknowledge typo, focus on patterns
  
- `threads_sample.rs` - Concurrency examples
  - Contains `static mut VAL` (unsafe access pattern)
  - Includes intentional deadlock example (commented)
  - Documentation MUST:
    - Explain why `static mut` is problematic
    - Show safe alternatives (Mutex, Arc)
    - Warn about deadlock code

**✗ CRITICAL ISSUES REQUIRING DOCUMENTATION**:
- `pointer_sample.rs:17` - `str::from_utf8_unchecked`
  - NO validation, unsafe conversion
  - Documentation MUST explain: Why unsafe, when safe to use, safer alternatives

- `sqlite_vec_sample.rs:7` - `std::mem::transmute`
  - Dangerous operation on function pointers
  - Documentation MUST: Strong warning, safe alternatives

- `tonic_store_server.rs:34` - `std::sync::Mutex` in async context
  - Anti-pattern: Blocks async executor
  - Documentation MUST: Explain async-safe alternative (`tokio::sync::Mutex`)

**Hardcoded Values to Note**:
- IP addresses: "192.168.2.6", "192.168.2.7" (greeter services)
- Socket path: `/tmp/hello.socket` (UDS binaries)
- Documentation should note: "Example only - use config in production"

---

### Knowledge Dependency Map

**LEARNING PATH: Basic → Advance → Awesome**

```
BEGINNER TRACK (src/basic/ - 14 files):
├── Variables & Expressions (expression_sample.rs)
│   └── Prerequisites: None
├── Data Types (datatype_sample.rs)  
│   └── Prerequisites: Variables
├── Ownership (ownership_sample.rs)
│   └── Prerequisites: Variables, Types
├── Borrowing & References implied in Ownership
│   └── Prerequisites: Ownership
├── Structs (rectangle.rs, struct examples in datatype)
│   └── Prerequisites: Ownership, Types
├── Enums (implied in basic examples)
│   └── Prerequisites: Structs
├── Traits (traits_sample.rs)
│   └── Prerequisites: Structs, Enums
├── Generics (generic_sample.rs)
│   └── Prerequisites: Traits
├── Modules (module_sample.rs)
│   └── Prerequisites: Any code organization
├── Closures (closure_sample.rs)
│   └── Prerequisites: Functions, Traits
├── Threads (threads_sample.rs) ⚠️ Has unsafe patterns
│   └── Prerequisites: Ownership, Modules
└── Advanced Features (cfg_if, pointer, logger, tracing, visiable)
    └── Prerequisites: Varies by topic

INTERMEDIATE TRACK (src/advance/ - 26 files):
├── Testing (testing patterns across samples)
│   └── Prerequisites: Basic Rust
├── File & Directory I/O (directory.md, CSV handling)
│   └── Prerequisites: Error handling, Traits
├── Database (sqlx_sample.rs, diesel_sample.rs)
│   └── Prerequisites: Async, Error handling
├── Async Programming (tokio_sample.rs, futures_sample.rs)
│   └── Prerequisites: Closures, Traits, Result types
├── Web Frameworks (axum_sample.rs, hyper_sample.rs)
│   └── Prerequisites: Async, Traits
├── Serialization (json_sample.rs, csv_sample.rs, rkyv_sample.rs)
│   └── Prerequisites: Structs, Enums, Traits
└── Utilities (tempfile, memmap, dotenv, bytes, etc.)
    └── Prerequisites: Varies

ADVANCED TRACK (crates/awesome/ - 7+ modules):
├── Service Framework (framework/lifecycle.rs, registry.rs)
│   └── Prerequisites: Async, Traits, Modules
├── Dependency Injection (dynmaic_injection_*.rs)
│   └── Prerequisites: Generics, Traits, Type system
├── Message Queues (rumqtt_sample.rs)
│   └── Prerequisites: Async, Error handling
├── Template Engines (tera, liquid, pest samples)
│   └── Prerequisites: String handling, Error handling
├── Database Advanced (surrealdb_sample.rs, sqlite_vec) ⚠️ Has unsafe
│   └── Prerequisites: Databases, Unsafe Rust basics
└── Data Processing (polars_sample.rs)
    └── Prerequisites: Collections, Iterators

SUPPLEMENTAL:
├── Algorithms (src/algo/linked_list.rs, calc_pi_sample.rs)
│   └── Prerequisites: Basic Rust, Data structures
└── LeetCode Solutions (crates/leetcode/solution_0001/0002.rs)
    └── Prerequisites: Basic Rust, Data structures

IPC EXAMPLES (src/bin/ - 15 binaries):
├── gRPC pairs (server/client for greeter, store)
├── Unix Domain Sockets (uds_server/client/parent)
├── Stdio IPC (stdio_parent/child)
└── Process Control (app_sys_ctl, app_nix_ctl)
```

**Learning Path Validation**: ✅ BASIC → ADVANCE → AWESOME is optimal progression.

**Gaps Found**:
- No dedicated "Error Handling" sample file (pattern scattered across files)
- Lifetimes not explicitly taught (embedded in ownership/trait examples)
- Some SUMMARY.md entries lack corresponding sample files (e.g., "特征" has traits_sample.rs ✓)

---

## Phase 1: Design & Contracts

### Data Model: Tutorial Chapter Structure

**Entity: 教程章节 (Tutorial Chapter)**

```yaml
chapter:
  id: "ownership"  # SUMMARY.md entry identifier
  title: "了解所有权"  # Chinese title
  english_term: "Ownership"  # For bilingual glossary
  
  # Metadata
  status: "draft | in_progress | complete"
  last_updated: "2026-04-03"
  version: "1.0"  # Sync with book version
  
  # Content mapping
  source_files:  # Map to one or more .rs samples
    - "src/basic/ownership_sample.rs"
    - "src/basic/datatype_sample.rs"  # If cross-referencing
  
  # Prerequisites
  requires:  # Links to other chapters
    - "variables"
    - "datatype"
  required_by:  # Chapters that build on this
    - "borrowing"
    - "lifetimes"
    
  # Chapter structure (12 sections)
  sections:
    - id: "opening_story"
      title: "开篇故事"
      type: "analogy"
      word_count: "50-100 Chinese characters"
      
    - id: "who_this_is_for"
      title: "本章适合谁"
      type: "prerequisites"
      word_count: "30-50 characters"
      
    - id: "learning_objectives"  
      title: "你会学到什么"
      type: "objectives"
      count: "3-5 specific outcomes"
      
    - id: "first_example"
      title: "第一个例子"
      type: "code"
      constraints:
        max_lines: 15
        must_compile: true
        source_ref: "ownership_sample.rs:lines_11-29"
        
    - id: "how_it_works"
      title: "原理解析"
      type: "explanation"
      format: "one_concept_per_paragraph"
      word_count: "200-300 characters"
      
    - id: "common_mistakes"
      title: "常见错误"  
      type: "examples"
      requirements:
        - "compiler_error_shown"
        - "explanation_provided"
        - "solution_provided"
      count: "2-3 errors"
      
    - id: "try_it_yourself"
      title: "动手练习"
      type: "exercises"
      difficulty: "increasing"
      count: "2-3 challenges"
      
    - id: "troubleshooting"
      title: "故障排查"
      type: "faq"
      format: "symptom_based_solutions"
      
    - id: "going_deeper"
      title: "知识扩展"
      type: "optional_advanced"
      marked_as: "OPTIONAL"
      
    - id: "summary"
      title: "小结"
      type: "bullet_points"
      count: "3-5 takeaways"

  # Quality attributes
  quality_metrics:
    min_word_count: 500  # Chinese characters
    max_code_lines: 15   # Per example
    required_sections: 11
    glossary_required: "complex_chapters_only"
    
  # Links
  internal_refs:  # Links to other chapters
    - "[变量表达式](expression.md)"
    - "[结构体](struct.md)"
  external_refs:  # Links to external resources
    - "https://doc.rust-lang.org/std/string/struct.String.html"
    - "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html"
```

**Entity: 学习路径 (Learning Path)**

```yaml
learning_paths:
  beginner:
    name: "Rust 基础入门 (Basic)"
    chapters: 14  # From src/basic/
    outcomes:
      - "理解所有权和借用系统"
      - "能编写安全的 Rust 代码"
      - "理解基本数据类型和结构"
    prerequisites: "基础编程知识 (变量，循环，函数)"
    
  intermediate:
    name: "高级进阶 (Advance)"  
    chapters: 26  # From src/advance/
    outcomes:
      - "能编写异步代码"
      - "能连接数据库"
      - "能构建 Web 服务"
    prerequisites: "完成基础章节"
    
  advanced:
    name: "精选实战 (Awesome)"
    chapters: 7+  # From crates/awesome/
    outcomes:
      - "理解生产级架构模式"
      - "能使用依赖注入"
      - "能集成消息队列"
    prerequisites: "完成中级章节"
```

---

### Quickstart Guide: Documentation Workflow

**For Documentation Writers**:

```markdown
# 文档编写快速指南

## 前置准备

1. **设置环境**
   ```bash
   # 安装 mdBook
   cargo install mdbook mdbook-admonish mdbook-alerts mdbook-pagetoc
   
   # 克隆仓库
   git clone <repo_url>
   cd hello-rust
   git checkout 001-rust-tutorial-docs
   ```

2. **验证构建**
   ```bash
   cd docs
   mdbook build
   # 应该：零警告，成功生成 docs/book/
   ```

## 编写流程

### 第一步：选择要文档化的代码样例

例如：`src/basic/ownership_sample.rs`

### 第二步：创建/更新对应文档

文件:`docs/src/basic/ownership.md`

### 第三步：跟随 12 部分章节模板

1. **开篇故事** - "想象你有一本书..."
2. **本章适合谁** - "如果你刚学完变量和类型..."
3. **你会学到什么** - "学完本章，你可以..."
4. **前置要求** - "需要理解：[变量](expression.md), [类型](datatype.md)"
5. **第一个例子** - 展示所有权转移的代码
6. **原理解析** - 用图书馆比喻解释
7. **常见错误** - 展示编译器错误："error[E0382]"
8. **动手练习** - "尝试修复借用冲突..."
9. **故障排查** - "遇到生命周期错误？看这里..."
10. **知识扩展** - "高级：Rc 和 Arc 类型..."
11. **小结** - 3-5 要点
12. **术语表** - "所有权 (ownership): Rust 的核心概念..."

### 第四步：验证代码示例

```bash
# 确保所有示例编译
cargo build --workspace
cargo test

# 本地预览 mdBook
cd docs && mdbook serve
# 打开 http://localhost:3000
```

### 第五步：提交

```bash
git add docs/src/basic/ownership.md
git commit -m "docs: 添加所有权章节"
```

## 写作提示

**✅ DO (推荐)**:
- 使用"我们"而不是"你" - "让我们运行代码"
- 庆祝小成就 - "太棒了！你完成了第一个所有权示例"
- 展示真实错误 - "你可能遇到这个错误..."
- 每段一个概念 - 不要在同一段解释所有权 + 借用

**❌ DON'T (避免)**:
- 说教词汇 - "显然"、"简单"、"如你所知"
- 长段落 - 控制在 4 句话以内
- 中英混杂 - 技术术语首次：所有权 (ownership)
- 说"只需" - 对学习者来说没有什么是简单的

## 质量标准

- **代码**: 必须编译，最好有测试
- **解释**: 零基础友好，有类比
- **错误**: 展示真实编译器输出
- **术语**: 首次出现时双语

## 帮助资源

- [Rust 指南中文版](https://kaisery.github.io/trpl-zh-cn/)
- [Rust by Example 中文版](https://rustwiki.org/zh-CN/rust-by-example/)
- [Chinese Rust User Group](https://rustcc.cn/)
```

---

### Agent Context Update

**Running agent context update script...**

```bash
bash .specify/scripts/bash/update-agent-context.sh opencode
```

**Added Technologies to Agent Context**:
- mdBook 0.4.36+ with plugins
- Documentation generation workflow
- Chinese technical writing patterns  
- Diataxis framework principles
- Bilingual documentation strategies

---

## Constitution Check (Post-Design)

**Re-evaluating constitution compliance after Phase 1 design:**

### I. Code Quality
✅ **VERIFIED** - Documentation approach ensures code quality:
- All examples compile (`cargo build --workspace`)
- References existing clippy/fmt standards
- No modifications to source samples

### II. Test-First Development  
✅ **ADAPTED** - For documentation:
- "Test" = compilation verification + reader exercises
- Examples drawn from samples with existing `#[cfg(test)]`
- Each chapter includes hands-on practice

### III. User Experience Consistency
✅ **EXCEEDED** - Comprehensive UX strategy:
- 12-section consistent template
- Bilingual terminology throughout  
- Error messages include remediation steps
- Chinese language with accessibility considerations

### IV. Performance Requirements  
✅ **N/A** - No runtime concerns; build performance validated

### V. SDD Harness Engineering
✅ **ON TRACK** - Following workflow:
- Phase 0 complete: ✅ Research.md
- Phase 1 complete: ✅ Design, data-model, quickstart
- Phase 2 next: `/speckit.tasks`

**Post-Design Verdict**: ✅ **PASS** - All constitution principles honored. Proceed to Phase 2.

---

## Phase 2: Task Breakdown Required

**Next Step**: Run `/speckit.tasks` command to decompose documentation writing into granular tasks (<4hr each).

**Expected Output**: `tasks.md` with ~50+ tasks covering:
- 14 Basic chapters
- 26 Advance chapters  
- 7+ Awesome chapters
- 4 supplemental chapters (Algo, LeetCode, Binaries)

**Complexity Summary**:
- ✅ No constitution violations
- ✅ Clear technical approach  
- ✅ Proven patterns (Diataxis, bilingual docs)
- ⏳ Implementation complexity: Medium (many chapters, straightforward pattern)
