<!--
SYNC IMPACT REPORT
==================
Version Change: 1.1.0 → 1.1.1 (PATCH: Update mdbook version + documentation quality clarifications)
Modified Principles:
  - Technology Stack: mdbook version 0.4.36+ → 0.4.52 (stable compatible version)
  - Added Documentation Quality standards to all principles
  - Clarified Chinese language requirements with English technical terms
Added Sections:
  - Documentation Quality Standards in each principle
Removed Sections: None
Templates Requiring Updates: None (documentation standards already in place)
Follow-up TODOs: None
==================
-->

# Hello Rust Constitution

## Core Principles

### I. Code Quality (NON-NEGOTIABLE)

All code MUST prioritize clarity, maintainability, and idiomatic Rust patterns.

**Requirements:**
- Follow Rust 2024 edition idioms and best practices
- Zero compiler warnings in production builds (`cargo build --release`)
- Mandatory `cargo clippy` with all lints addressed or explicitly allowed with justification
- Maximum cyclomatic complexity: 15 per function (exceptions require architectural review)
- Documentation comments (`///`) on all public APIs with examples
- No `unsafe` code without documented safety invariants and architectural approval

**Documentation Quality:**
- All code examples MUST be from real project code (no fictional examples)
- Code examples MUST include GitHub source links
- All examples MUST compile successfully with `cargo build --workspace`

**Rationale:** Learning resources must demonstrate best practices. Students learn from what they see. Poor code quality compounds as learners replicate patterns.

**Quality Gates:**
- `cargo fmt -- --check` MUST pass
- `cargo clippy -- -D warnings` MUST pass
- `cargo test` MUST pass with >80% code coverage
- All `TODO` and `FIXME` comments MUST have associated issues

### II. Test-First Development (NON-NEGOTIABLE)

Test-driven development is mandatory for all new features and bug fixes.

**Requirements:**
- Tests written and approved BEFORE implementation begins
- Red-Green-Refactor cycle strictly enforced
- Unit tests for all business logic (target: >80% coverage)
- Integration tests for all inter-service communication patterns
- Property-based tests for algorithmic code (using `proptest` crate)
- Performance benchmarks for critical paths (using `criterion` crate)

**Documentation Testing:**
- Documentation code snippets MUST be executable
- Knowledge checkpoint questions MUST validate learning outcomes
- All examples MUST have corresponding test coverage

**Rationale:** Tests serve as executable specifications and living documentation. They catch regressions and validate learning outcomes.

**Testing Tiers:**
1. **Unit Tests**: Fast, isolated, comprehensive (target: thousands of tests, <100ms total)
2. **Integration Tests**: Service boundaries, IPC patterns, database interactions
3. **End-to-End Tests**: Full system workflows using `gstack` browser automation
4. **Performance Tests**: Benchmark critical paths, detect regressions

**Anti-Patterns:**
- `#[ignore]` tests without documented reasons and tracking issues
- Tests that only pass in specific environments without explicit setup/teardown
- Mocking internal implementation details instead of public interfaces

### III. User Experience Consistency

All user-facing interfaces MUST provide intuitive, consistent, and accessible experiences.

**Requirements:**
- CLI interfaces: Consistent argument parsing via `clap`, helpful error messages, progress indicators
- gRPC/HTTP APIs: Semantic versioning, backwards compatibility guarantees, comprehensive error codes
- Documentation: Chinese primary language with English technical terms, searchable, runnable examples
- Error Messages: Actionable, specific, include context and remediation steps
- Response Times: <100ms for interactive operations, <1s for complex queries

**Documentation Language Standards:**
- **Primary Language**: Chinese (Simplified) with English technical terms in parentheses
  - Example: 所有权 (ownership), 借用 (borrowing), 生命周期 (lifetime)
- **Writing Style**: Plain language, avoid academic jargon
- **Chapter Structure**: 12-section template for all tutorial chapters
- **Content Requirements**:
  - Minimum 500 Chinese characters per chapter
  - At least 3 executable code examples
  - At least 3 knowledge checkpoint questions
  - GitHub links to all source code examples

**UX Principles:**
- **Discoverability**: Every feature accessible via `--help` or API documentation
- **Predictability**: Consistent naming, argument order, and output formats
- **Recoverability**: Clear error messages with suggested fixes, no silent failures
- **Accessibility**: WCAG 2.1 AA compliance for web interfaces, screen reader compatible

**gstack Integration:**
- Use `/browse` for manual UX validation before deployment
- Use `/qa` for automated accessibility testing
- Use `/design-review` for visual consistency audits

**Documentation Build Quality:**
- `mdbook build` MUST pass with zero errors and warnings
- All links MUST be valid (no 404 errors)
- All code examples MUST render correctly with syntax highlighting

### IV. Performance Requirements

All code MUST meet defined performance standards and resource constraints.

**Requirements:**
- Memory: No unbounded allocations, explicit limits on data structures
- CPU: No blocking operations in async contexts (use `tokio::spawn_blocking`)
- I/O: Streaming for large datasets (no full materialization in memory)
- Network: Connection pooling, retry with exponential backoff, circuit breakers
- Database: Prepared statements, query plan validation, <10ms query latency

**Performance Standards:**
- API endpoints: p95 <100ms, p99 <500ms
- gRPC streaming: <50ms latency, >10k messages/sec throughput
- CLI startup: <50ms cold start, <10ms warm start
- Memory footprint: <100MB for demo applications, <500MB for production services
- Binary size: <20MB for CLI tools, <50MB for server binaries (release builds)

**Documentation Performance:**
- mdbook build time MUST be <5 minutes for full documentation
- Individual chapter builds MUST be <30 seconds
- Hot reload during development MUST be <2 seconds

**Performance Anti-Patterns:**
- `thread::sleep()` in polling loops (use async primitives instead)
- Synchronous I/O in async contexts (use `tokio::fs`, `tokio::net`)
- Unbounded channels (use bounded channels with backpressure)
- Clone-on-read for large data structures (use Arc, references, or Cow)

**Profiling Requirements:**
- Use `cargo flamegraph` for CPU profiling before optimization
- Use `heaptrack` or `valgrind` for memory leak detection
- Benchmark critical paths with `criterion`, track in CI
- Document performance characteristics in AGENTS.md

### V. SDD Harness Engineering

Specification Driven Development (SDD) workflows MUST follow the **8-Phase Development Lifecycle** with triple quality gates (Metis + Momus + GStack).

**Development Phases:**

**Phase 0: Product Strategy & Requirements**
- `/office-hours` — Product discovery (YC 6-question forcing framework)
- `/plan-ceo-review` — Scope challenge (4 modes: SCOPE EXPANSION/SELECTIVE/HOLD/REDUCTION)
- `/speckit.specify` — Generate feature specifications
- **Quality Gate**: Metis intent analysis + Momus spec review (≥8/10)

**Phase 1: Technical Architecture & Design**
- `/speckit.plan` — Technical design with constitution check
- `/plan-eng-review` — Engineering review (architecture/data flow/performance)
- `/design-consultation` + `/plan-design-review` — Design system (UI projects)
- **Quality Gate**: Metis deep planning + Momus plan review (≥8/10)

**Phase 2: Task Decomposition**
- `/speckit.tasks` — Granular task breakdown (<4hr per task)
- `/speckit.analyze` — Cross-artifact consistency analysis
- **Quality Gate**: No CRITICAL/HIGH inconsistencies

**Phase 3: Quality Checklists**
- `/speckit.checklist` — Multi-domain checklists (test/security/ux/performance/code-quality/architecture/ai-safety)
- **Quality Gate**: 100% checklist coverage

**Phase 4: Implementation**
- `/speckit.implement` — Test-first execution with task delegation
- **Quality Gate**: `cargo clippy` + `cargo fmt` + compilation success
- **Manual Review**: Changes MUST be manually reviewed before commit
- **Manual Commit**: ALL commits MUST be manually committed and pushed by user
- **Prohibited**: NO automatic commits or pushes to remote repositories

**Phase 5: Testing & Validation**
- `cargo test && cargo nextest run` — Automated testing
- `/review` — Pre-landing PR review
- `/qa` — End-to-end QA testing with browser automation
- **Quality Gate**: 100% tests pass + no CRITICAL issues

**Phase 6: Delivery & Release**
- `/document-release` — Update all documentation
- `/ship` — Merge, version bump, create PR (with user approval)
- **Quality Gate**: All quality gates passed
- **Manual Verification**: User MUST verify all changes before deployment

**Phase 7: Retrospective**
- `/retro` — Engineering retro with trend analysis
- **Output**: Improvement action items for next iteration

**Triple Quality Gates:**

| Gate | Role | Timing | Purpose |
|------|------|--------|---------|
| **Metis** | Pre-planning consultant | Before each phase | Intent analysis, ambiguity detection, AI failure prediction, routing strategy |
| **Momus** | Post-delivery reviewer | After each phase | Clarity/verifiability/completeness/context evaluation, AI failure mode detection |
| **GStack** | Professional specialist | During execution | Domain-specific expertise (CEO review, eng review, design review, QA, PR review) |

**Skill Integration Matrix:**

| Phase | Speckit Commands | GStack Skills | OhMyOpenCode Agents |
|-------|------------------|---------------|---------------------|
| Phase 0 | `specify` | `office-hours`, `plan-ceo-review` | `metis`, `librarian` |
| Phase 1 | `plan` | `plan-eng-review`, `design-consultation`, `plan-design-review` | `metis`, `oracle`, `explore` |
| Phase 2 | `tasks`, `analyze` | - | `metis`, `momus` |
| Phase 3 | `checklist` | - | `momus` |
| Phase 4 | `implement` | - | `task()` delegation |
| Phase 5 | - | `review`, `qa`, `browse` | `momus` |
| Phase 6 | - | `document-release`, `ship` | `momus` |
| Phase 7 | - | `retro` | `momus` |

**Workflow Requirements:**
- Feature specifications via `/speckit.specify` (mandatory for all features)
- Implementation plans via `/speckit.plan` (mandatory before coding)
- Constitution check at Phase 1 (verify all 5 principles)
- Test-first development enforced in Phase 4
- All quality gates MUST pass before proceeding to next phase
- Document all decisions in `docs/specs/{N}-{feature}/`
- **Manual Control**: User MUST manually review, commit, and push all changes

**Workflow Enforcement:**
- No direct commits to `main` branch (use feature branches with PRs)
- All PRs MUST reference a spec document in `docs/specs/`
- All code changes MUST have corresponding test updates
- Breaking changes MUST update version according to semver and migration guide
- **CRITICAL**: NO automatic commits or pushes - user maintains full control

**Automation Standards:**
- CI pipeline: Build → Test → Lint → Benchmark → Deploy
- Deployment: Automated via GitHub Actions, rollback procedures documented
- Monitoring: Structured logging (`tracing`), metrics (`prometheus`), tracing (`jaeger`)
- Incident Response: Runbooks in `docs/runbooks/`, on-call rotation documented
- **Manual Gates**: User approval required at all deployment stages

**Tool Stack:**
- **Speckit Framework**: 8-phase SDD workflow (`specify`, `plan`, `tasks`, `analyze`, `checklist`, `implement`)
- **GStack Skills**: Quality automation (`office-hours`, `plan-ceo-review`, `plan-eng-review`, `design-consultation`, `plan-design-review`, `review`, `qa`, `browse`, `ship`, `retro`)
- **OhMyOpenCode Agents**: Triple quality gates (`metis` pre-planning, `momus` post-review, `oracle` architecture, `explore` codebase, `librarian` external research)
- **Rust Tooling**: `cargo-nextest`, `cargo-tarpaulin`, `cargo-deny`, `cargo-audit`
- **Manual Commit Policy**: ALL commits require user review and manual execution

## Technology Stack

**Core:**
- Language: Rust 2024 Edition
- Async Runtime: Tokio 1.x with full features
- Serialization: Serde (JSON, TOML, YAML support)
- Logging/Tracing: `tracing` with `tracing-subscriber`

**Frameworks:**
- Web: Axum 0.8+ (HTTP), Tonic 0.13+ (gRPC)
- Database: SQLx 0.8+ (async native), Diesel 2.2+ (ORM)
- Messaging: rumqttc (MQTT), custom Consul client

**Testing:**
- Unit/Integration: `cargo test`, `cargo-nextest`
- Property-Based: `proptest`
- Mocking: `mockall` with `async-trait`
- Coverage: `cargo-tarpaulin` (target: >80%)
- Benchmarks: `criterion`

**Documentation:**
- mdBook 0.4.52 (stable version) with plugins (admonish, alerts, pagetoc)
- Primary language: Chinese (Simplified) with English technical terms
- Deployed to: GitHub Pages
- Build verification: `mdbook build` MUST pass with zero errors

**Build/CI:**
- Protobuf: `protoc` 29.3 (for tonic-build)
- CI: GitHub Actions (rust.yml, mdbook.yml)
- Multi-arch: Docker with cross-compilation support

## Development Workflow

### Feature Development Lifecycle

1. **Specification** (`/speckit.specify`)
   - Create feature spec in `docs/specs/<###-feature-name>/spec.md`
   - Define user stories, acceptance criteria, success metrics
   - Quality checklist validation

2. **Planning** (`/speckit.plan`)
   - Technical design document
   - Architecture decisions with rationale
   - Constitution check (verify compliance with all 5 principles)
   - Dependency analysis

3. **Implementation** (`/speckit.tasks` → `/speckit.implement`)
   - Granular task breakdown (<4hr per task)
   - Test-first: Write tests → Tests fail → Implement → Tests pass
   - **Manual Review**: ALL changes MUST be manually reviewed
   - **Manual Commit**: User MUST execute all git commits
   - **Manual Push**: User MUST execute all git pushes
   - **Prohibited**: NO automatic commits or pushes

4. **Quality Assurance** (`/qa`)
   - Automated testing (unit, integration, e2e)
   - Visual validation (`/browse`, `/design-review`)
   - Performance benchmarks
   - Security scan (`cargo audit`, `cargo deny`)

5. **Review** (`/review`)
   - Pre-landing code review
   - Constitution compliance check
   - Performance regression check
   - Documentation completeness

6. **Deploy**
   - Merge to `main` via PR
   - Automated CI/CD pipeline
   - Post-deploy monitoring
   - **Manual approval required at all stages**

### Branch Strategy

- `main`: Production-ready code, protected
- `<###-feature-name>`: Feature branches (sequential numbering from speckit)
- All branches MUST have associated spec document
- **Manual Control**: User decides when to create branches and merge

### Commit Conventions
 
 **DO NOT COMMIT and PUSH**
 **DO NOT COMMIT and PUSH**
 **DO NOT COMMIT and PUSH**

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`

**Manual Commit Examples:**
```bash
# User manually reviews and commits
git add <files>
# git commit -m "feat(docs): Update constitution to v1.1.1"
# git push origin main
```

**Prohibited:**
```bash
# DO NOT automatically commit or push
# git commit -m "auto: ..."  # FORBIDDEN
# git push origin main       # FORBIDDEN without user approval
```

## Governance

**Authority:**
This constitution supersedes all other development practices and guides. In case of conflict with team conventions, constitution principles take precedence.

**Amendment Process:**
1. Propose amendment via GitHub issue with rationale
2. Architectural review for impact assessment
3. Team discussion and approval (consensus required)
4. Update constitution with version bump (MAJOR.MINOR.PATCH)
5. Propagate changes to all dependent templates and documentation
6. Announce changes to all contributors
7. **Manual Execution**: All constitutional amendments require manual user approval and commit

**Versioning Policy:**
- **MAJOR**: Backward incompatible principle removals or redefinitions
- **MINOR**: New principle/section added or materially expanded guidance
- **PATCH**: Clarifications, wording improvements, typo fixes

**Compliance Review:**
- All PRs MUST verify constitution compliance via `/review` command
- Complexity exceptions MUST be justified in PR description with architectural approval
- Violations of NON-NEGOTIABLE principles block merge
- **Manual Review**: User MUST verify all compliance checks before merge

**Runtime Guidance:**
- Use `AGENTS.md` for project-specific technical guidance
- Use `.specify/templates/` for workflow templates
- Use `docs/` for user-facing documentation

**Enforcement:**
- CI checks for linting, formatting, testing, security
- Mandatory code review for all changes
- Quarterly constitution review and update cycle
- **Manual Control**: User has final approval on all changes to main branch

---

**Version**: 1.1.1 | **Ratified**: 2026-04-03 | **Last Amended**: 2026-04-04
