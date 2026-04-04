# Checklist: Voice & Tone Requirements Quality

**Purpose**: Validate requirements for plain language (平实的语言), easy reading (阅读轻松), encouraging tone, and cognitive load management
**Created**: 2026-04-03
**Spec**: [spec.md](../spec.md)

## Requirement Completeness

- [x] CHK131 - Are tone requirements defined for all chapter sections (encouraging, supportive, non-condescending)? [Spec §Tone and Voice]
- [ ] CHK132 - Are "we" language requirements defined (using "让我们" vs "你应该")? [Spec §Tone and Voice]
- [x] CHK133 - Are forbidden word/phrase requirements defined (避免"显然"、"简单"、"只需")? [Spec §Tone and Voice]
- [x] CHK134 - Are difficulty acknowledgment requirements defined (认可难度，正常化困难)? [Spec §Tone and Voice]
- [ ] CHK135 - Are celebration requirements defined for learner achievements (庆祝小进步)? [Spec §Tone and Voice]
- [x] CHK136 - Are paragraph length requirements defined (2-4 sentences, max 20 characters per sentence)? [Spec §Chinese Writing, §Cognitive Load]
- [ ] CHK137 - Are conceptual density requirements defined (one concept per paragraph)? [Spec §Cognitive Load]
- [ ] CHK138 - Are code example length requirements defined (max 15 lines per example)? [Spec §Cognitive Load]

## Requirement Clarity

- [ ] CHK139 - Is "平实的语言" (plain language) defined with specific linguistic criteria? [Clarity, Spec §Principle 2]
- [ ] CHK140 - Is "阅读轻松" (easy reading) quantified with readability metrics? [Ambiguity]
- [ ] CHK141 - Is "encouraging" tone defined with example phrases vs forbidden phrases? [Clarity, Spec §Tone and Voice]
- [ ] CHK142 - Is "2-4 sentences per paragraph" measured with Chinese punctuation rules? [Clarity, Spec §Chinese Writing]
- [ ] CHK143 - Is "20 characters per sentence" counted including/excluding code/technical terms? [Ambiguity, Spec §Chinese Writing]
- [ ] CHK144 - Is "one concept per paragraph" defined with concept scope (sub-concepts vs main concepts)? [Ambiguity, Spec §Cognitive Load]

## Requirement Consistency

- [ ] CHK145 - Are tone requirements consistent between Principle 2 (Simple Language) and Tone and Voice section? [Consistency]
- [ ] CHK146 - Are paragraph length requirements consistent between Chinese Writing and Cognitive Load sections? [Consistency]
- [ ] CHK147 - Is "we" language requirement consistent across all user stories? [Consistency, Spec §User Story 1-5]
- [ ] CHK148 - Are difficulty acknowledgment requirements aligned with edge case handling? [Consistency, Spec §Edge Cases]

## Acceptance Criteria Quality

- [ ] CHK149 - Can "平实的语言" be objectively measured with readability scores? [Measurability, Spec §Principle 2]
- [ ] CHK150 - Can "encouraging tone" be verified with specific phrase presence/absence? [Measurability, Spec §Tone and Voice]
- [ ] CHK151 - Can "one concept per paragraph" be verified with concept extraction? [Measurability, Spec §Cognitive Load]
- [ ] CHK152 - Can "阅读轻松" be assessed without subjective reader panels? [Measurability, Gap]

## Scenario Coverage

- [ ] CHK153 - Are requirements defined for technical explanations that cannot be simplified further? [Coverage, Gap]
- [ ] CHK154 - Are requirements defined for error messages that must include harsh compiler output? [Coverage, Spec §Principle 5]
- [ ] CHK155 - Are requirements defined for topics that are genuinely difficult (lifetimes, unsafe Rust)? [Coverage, Gap]
- [ ] CHK156 - Are requirements defined for chapters targeting advanced learners (less hand-holding)? [Coverage, Spec §User Story 3]
- [ ] CHK157 - Are requirements defined for cross-chapter tone consistency when multiple authors contribute? [Coverage, Gap]

## Edge Case Coverage

- [ ] CHK158 - Are requirements defined for handling learner frustration in documentation (failed exercises)? [Edge Case, Spec §Tone and Voice]
- [ ] CHK159 - Are requirements defined for humor usage (culturally appropriate, not distracting)? [Edge Case, Gap]
- [ ] CHK160 - Are requirements defined for emotional support when code doesn't work (normalizing struggle)? [Edge Case, Spec §Tone and Voice]
- [ ] CHK161 - Are requirements defined for warnings without inducing fear (unsafe code, performance pitfalls)? [Edge Case, Gap]

## Non-Functional Requirements

**Readability**:
- [ ] CHK162 - Are white space requirements defined (headings every 200-300 words, section breaks)? [Spec §Cognitive Load]
- [ ] CHK163 - Are visual hierarchy requirements defined (heading levels, formatting)? [Spec §Visual Aids]
- [ ] CHK164 - Are bullet point vs prose requirements defined (when to use lists)? [Gap]

**Engagement**:
- [ ] CHK165 - Are story requirements defined (2-4 sentences, culturally relevant, memorable)? [Spec §Principle 3]
- [ ] CHK166 - Are analogy quality requirements defined (relatable, accurate, not misleading)? [Spec §Principle 3]
- [ ] CHK167 - Are exercise engagement requirements defined (progressive difficulty, achievable challenges)? [Spec §FR-007]

**Accessibility**:
- [ ] CHK168 - Are requirements defined for readers with different reading speeds? [Gap]
- [ ] CHK169 - Are requirements defined for skimming vs deep reading usage patterns? [Gap]
- [ ] CHK170 - Are requirements defined for non-native Chinese speakers learning Rust? [Gap]

## Dependencies & Assumptions

- [ ] CHK171 - Is there a dependency on Chinese language style guides or standards? [Dependency, Gap]
- [ ] CHK172 - Is the assumption "Chinese-speaking developers have similar reading preferences" validated? [Assumption, Spec §Language Strategy]
- [ ] CHK173 - Are requirements dependent on cultural context understanding for analogies? [Dependency, Spec §Principle 3]

## Cognitive Load Management

- [ ] CHK174 - Are progressive disclosure requirements defined (simple → complex progression)? [Spec §Principle 4]
- [ ] CHK175 - Are summary requirements defined at multiple levels (section, chapter, book)? [Spec §Cognitive Load]
- [ ] CHK176 - Are cross-reference requirements defined without overwhelming navigation? [Gap]
- [ ] CHK177 - Are optional/deep-dive section requirements defined (clearly marked as optional)? [Spec §Chapter Structure §10]

## Resolved Issues ✅

- [x] Tone defined: supportive, encouraging, never condescending ✓ [Spec §Tone and Voice]
- [x] Forbidden terms listed: "显然", "简单", "只需", etc. ✓ [Spec §Tone and Voice]
- [x] "We" language required: "让我们" not "你应该" ✓ [Spec §Tone and Voice]
- [x] Paragraph length defined: 2-4 sentences ✓ [Spec §Chinese Writing]
- [x] Code examples max 15 lines ✓ [Spec §Cognitive Load]

## Ambiguities Requiring Clarification ⚠️

- [ ] How to measure "阅读轻松" objectively without reader panels? [Ambiguity]
- [ ] What's the escalation path for difficult topics that resist simplification? [Ambiguity, Gap]
- [ ] How to balance encouragement with technical precision? [Ambiguity, Gap]

---

**Coverage Summary**:
- Total items: 47
- Completeness: 8 items
- Clarity: 6 items  
- Consistency: 4 items
- Acceptance Criteria: 4 items
- Scenario Coverage: 5 items
- Edge Cases: 4 items
- Non-Functional: 9 items
- Dependencies: 3 items
- Cognitive Load: 4 items

**Gaps Identified**: 17 requirements missing or need definition
**Ambiguities**: 3 items need clarification
**Resolved**: 5 items already addressed in spec
