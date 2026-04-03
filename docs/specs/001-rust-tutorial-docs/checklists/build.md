# Checklist: Build & Deployment Requirements Quality

**Purpose**: Validate requirements for mdBook build process, GitHub Pages deployment, and tooling infrastructure
**Created**: 2026-04-03
**Spec**: [spec.md](../spec.md), [plan.md](./plan.md)

## Requirement Completeness

- [ ] CHK178 - Are mdBook build requirements defined (mdbook build with zero warnings)? [Spec §SC-006]
- [ ] CHK179 - Are mdBook plugin requirements defined (admonish, alerts, pagetoc versions)? [Spec §Technology]
- [ ] CHK180 - Are GitHub Pages deployment requirements defined (deployment trigger, branch, process)? [Spec §Technology]
- [ ] CHK181 - Are local development requirements defined (mdbook serve, preview workflow)? [Research §Quickstart]
- [ ] CHK182 - Are build validation requirements defined in CI/CD pipeline? [Gap]
- [ ] CHK183 - Are link checking requirements defined (internal and external link validation)? [Gap]
- [ ] CHK184 - Are requirements defined for build performance (build time limits for documentation changes)? [Gap]
- [ ] CHK185 - Are requirements defined for pre-built HTML in docs/book/ directory (update on every build?)? [Spec §Assumptions]

## Requirement Clarity

- [ ] CHK186 - Is "zero warnings" scope defined (mdbook warnings, plugin warnings, browser console?)? [Clarity, Spec §SC-006]
- [ ] CHK187 - Are mdBook plugin versions pinned or "latest stable"? [Ambiguity, Spec §Technology]
- [ ] CHK188 - Is GitHub Pages deployment triggered by merge, manual, or scheduled? [Ambiguity, Gap]
- [ ] CHK189 - Are link validation requirements defined (ignore which external domains, check which internal paths)? [Clarity, Gap]
- [ ] CHK190 - Is "docs/book/ committed" requirement defined (regenerate every build, commit only source?)? [Clarity, Spec §Assumptions]

## Requirement Consistency

- [ ] CHK191 - Are build requirements consistent between spec §SC-006 and plan §Technical Context? [Consistency]
- [ ] CHK192 - Are plugin requirements aligned across spec §Technology and research §Quickstart? [Consistency]
- [ ] CHK193 - Is the "pre-built documentation in docs/book/ is gitignored" assumption consistent with repository state? [Conflict, Spec §Assumptions]
- [ ] CHK194 - Are mdBook structure requirements consistent with SUMMARY.md preservation requirement (FR-009)? [Consistency]

## Acceptance Criteria Quality

- [ ] CHK195 - Can "mdbook build with zero warnings" be objectively verified in CI? [Measurability, Spec §SC-006]
- [ ] CHK196 - Can GitHub Pages deployment be verified programmatically? [Measurability, Gap]
- [ ] CHK197 - Can link validation be automated with specific tooling? [Measurability, Gap]
- [ ] CHK198 - Can documentation structure be verified against SUMMARY.md automatically? [Measurability, Spec §FR-009]

## Scenario Coverage

- [ ] CHK199 - Are requirements defined for mdBook build failures (what blocks merge, what's warning only)? [Coverage, Gap]
- [ ] CHK200 - Are requirements defined for plugin version conflicts or breaking changes? [Coverage, Gap]
- [ ] CHK201 - Are requirements defined for broken links in external references (Rust Book docs.rs links)? [Coverage, Gap]
- [ ] CHK202 - Are requirements defined for large documentation builds (timeout handling)? [Coverage, Gap]
- [ ] CHK203 - Are requirements defined for concurrent documentation updates (merge conflicts in generated HTML)? [Coverage, Gap]

## Edge Case Coverage

- [ ] CHK204 - Are requirements defined for platform-specific build behavior (macOS vs Linux build differences)? [Edge Case, Gap]
- [ ] CHK205 - Are requirements defined for offline builds (no network access for mdbook plugins)? [Edge Case, Gap]
- [ ] CHK206 - Are requirements defined for custom markdown extensions not supported by mdBook? [Edge Case, Gap]
- [ ] CHK207 - Are requirements defined for image/asset size limits (large diagrams, screenshots)? [Edge Case, Gap]
- [ ] CHK208 - Are requirements defined for handling build warnings from third-party plugins? [Edge Case, Gap]

## Non-Functional Requirements

**Build Performance**:
- [ ] CHK209 - Are build time requirements defined (<5 min for full build, <30 sec for incremental)? [Gap]
- [ ] CHK210 - Are incremental build requirements defined (what triggers full rebuild)? [Gap]
- [ ] CHK211 - Are caching requirements defined (plugin caching, asset caching)? [Gap]

**Deployment**:
- [ ] CHK212 - Are rollback requirements defined if deployment causes issues? [Gap]
- [ ] CHK213 - Are blue-green deployment requirements defined (avoid downtime)? [Gap]
- [ ] CHK214 - Are cache invalidation requirements defined (CDN, browser cache)? [Gap]

**Maintenance**:
- [ ] CHK215 - Are requirements defined for mdBook version updates? [Gap]
- [ ] CHK216 - Are requirements defined for plugin deprecation/migration? [Gap]
- [ ] CHK217 - Are requirements defined for dependency security updates (cargo audit for mdBook)? [Gap]

**Monitoring**:
- [ ] CHK218 - Are requirements defined for broken link monitoring post-deployment? [Gap]
- [ ] CHK219 - Are analytics requirements defined (page views, search queries, user behavior)? [Gap]
- [ ] CHK220 - Are feedback collection requirements defined (GitHub issues integration)? [Spec §Feedback Mechanisms]

## Dependencies & Assumptions

- [ ] CHK221 - Is the dependency on GitHub Pages infrastructure explicitly documented? [Dependency, Gap]
- [ ] CHK222 - Are dependencies defined for external link availability (Rust Book, docs.rs uptime)? [Dependency, Spec §External References]
- [ ] CHK223 - Is the assumption "mdBook build succeeds with current configuration" validated? [Assumption, Spec §SC-006]
- [ ] CHK224 - Are tooling dependencies defined (Rust toolchain for mdBook installation)? [Dependency, Research §Quickstart]

## Source Control & Workflow

- [ ] CHK225 - Are requirements defined for docs/book/ directory (commit pre-built HTML or generate on deploy?)? [Spec §Assumptions, Gap]
- [ ] CHK226 - Are requirements defined for commit scope (docs-only commits, code+docs commits)? [Gap]
- [ ] CHK227 - Are requirements defined for PR review of documentation (preview build verification)? [Gap]
- [ ] CHK228 - Are requirements defined for documentation branching strategy (one branch per feature/section)? [Gap]

## Resolved Issues ✅

- [x] mdBook version defined: 0.4.36+ with plugins ✓ [Spec §Technology]
- [x] Build command defined: mdbook build (zero warnings) ✓ [Spec §SC-006]
- [x] Deployment target: GitHub Pages ✓ [Spec §Technology]
- [x] Pre-built docs handling: gitignored, source.md is authoritative ✓ [Spec §Assumptions]

## Ambiguities Requiring Clarification ⚠️

- [ ] docs/book/ status: Spec says "gitignored for generated content" but repository shows pre-built HTML is committed. Needs clarification. [Ambiguity, Spec §Assumptions]
- [ ] mdBook plugin versions: "0.4.36+" for mdBook, but what about plugin versions? [Ambiguity, Spec §Technology]
- [ ] CI/CD integration: Are build checks required in CI, or only local validation? [Ambiguity, Gap]

## Conflicts Identified ⚠️

- [ ] Spec §Assumptions states "Pre-built documentation in docs/book/ is gitignored" but this conflicts with observation that docs/book/ contains committed HTML. Requires reconciliation. [Conflict, Spec §Assumptions]

---

**Coverage Summary**:
- Total items: 51
- Completeness: 8 items
- Clarity: 5 items  
- Consistency: 4 items
- Acceptance Criteria: 4 items
- Scenario Coverage: 5 items
- Edge Cases: 5 items
- Non-Functional: 13 items
- Dependencies: 4 items
- Source Control: 4 items

**Gaps Identified**: 23 requirements missing or need definition
**Ambiguities**: 3 items need clarification
**Conflicts**: 1 item needs reconciliation
**Resolved**: 4 items already addressed in spec

---

## Overall Checklist Portfolio Summary

| Checklist | Items | Gaps | Ambiguities | Conflicts | Resolved |
|-----------|-------|------|-------------|-----------|----------|
| Learning    | 43    | 17   | 3           | 0         | 3        |
| Bilingual   | 41    | 19   | 3           | 0         | 3        |
| Technical   | 46    | 18   | 3           | 1         | 3        |
| Voice       | 47    | 17   | 3           | 0         | 5        |
| Build       | 51    | 23   | 3           | 1         | 4        |
| **TOTAL**   | **228** | **94** | **15**      | **2**     | **18**   |

**Next Actions**:
1. Address 94 gaps by adding missing requirements to spec or plan
2. Clarify 15 ambiguities through stakeholder discussion
3. Reconcile 2 conflicts (technical assumption vs reality, docs/book/ status)
4. Re-review checklists after requirements refinement

**File Handling**: Created 5 new checklists in `docs/specs/001-rust-tutorial-docs/checklists/`
