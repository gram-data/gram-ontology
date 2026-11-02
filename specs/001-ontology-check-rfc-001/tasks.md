---

description: "Task list for gram-ontology check covering every RFC-001 section 5.6 check"
---

# Tasks: `gram-ontology check` covers every RFC-001 §5.6 check

**Input**: Design documents from `/specs/001-ontology-check-rfc-001/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/check-command.md](contracts/check-command.md)

**Tests**: Included. The spec requires every check to have a failing and a passing scenario exercised by an automated test (SC-001), so tests are written first and must fail before the implementation task that satisfies them.

**Organization**: by user story. US6 (diagnostic shape and lint superset) comes first because every other story reports through it; it is P1 like US1-US3.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: can run in parallel (different files, no dependency on an incomplete task)
- **[Story]**: US1-US6 from the spec
- All paths are relative to the repository root. Crate root: `crates/gram-ontology/`.
- Fixture naming: `crates/gram-ontology/tests/fixtures/<check-name>.<good|bad>.ontology.gram`. A `.bad.` fixture holds one instance of the mistake; a `.good.` fixture holds the nearest valid form and must produce no diagnostics.

---

## Phase 1: Setup

- [X] T001 Bump dependencies in `crates/gram-ontology/Cargo.toml`: `gram-diagnostics` to 0.6.1, `tree-sitter-gram` to 0.6.1, `tree-sitter-gramdoc` to 0.3, add `gram-data` 0.6.1; run `cargo build` and fix compile errors from the version bumps in `crates/gram-ontology/src/main.rs` (research R1, R5)
- [X] T002 Re-check `gram-data` 0.6.1's lint rule names against research R1 (`syntax-error`, `missing-node`, `duplicate-element`, `duplicate-key` in 0.6.0) and update `specs/001-ontology-check-rfc-001/research.md` and the catalogue in `specs/001-ontology-check-rfc-001/contracts/check-command.md` if they differ (FR-029)
- [X] T003 Measure the build and binary-size cost of the `gram-data` dependency and record it in `specs/001-ontology-check-rfc-001/research.md` R1; if unacceptable, stop and raise it with the author before continuing (fallback in R1)
- [X] T004 [P] Create `crates/gram-ontology/tests/common/mod.rs` with a helper that writes a source to a temp `*.ontology.gram` file, runs `gram-ontology check [--json]` and returns exit status and parsed diagnostics (check name, severity, 1-based line), and a helper that loads a named fixture
- [X] T005 [P] Create the empty `crates/gram-ontology/tests/fixtures/` directory with a short `README.md` stating the fixture naming rule above

---

## Phase 2: Foundational (blocks all user stories)

**Purpose**: the file model, the severity table, the check registry and the diagnostic builder that every check uses.

- [X] T006 Create `crates/gram-ontology/src/model.rs` with the entities in `data-model.md`: `Definition`, `Graph`, `Doc`, `Annotation` and the derived indexes `by_identifier`, `by_identifying_label`, `label_edges`, `referenced`; building from the tree-sitter CST, looking through `annotated_pattern`, reading `:` vs `::` from the anonymous separator token and expanding the `(Actor::Person)` shorthand (research R3, R4); skip regions with syntax errors (FR-027, research R6)
- [X] T007 Unit-test the model builder in `crates/gram-ontology/src/model.rs` on inline sources: `(Person)`, `(Actor::Person)`, `(actor:Actor:Person)`, `(actor::Actor:Person)`, `[ownsCar:OWNS]`, annotation-wrapped definitions, top-level `(a)` and `()-[r]->()` treated as definitions (spec A6)
- [X] T008 Create `crates/gram-ontology/src/checks/mod.rs` with the check registry (name, severity, function over the model) and one severity table holding the spec's severity table (research R7, FR-021, spec A1); expose a `diagnostic_for(check_name, message, range, help)` builder so no check picks its own severity
- [X] T009 Route the existing opt-in-mismatch and doc-comment parse checks through the severity table in `crates/gram-ontology/src/checks/mod.rs`; the checks themselves stay in `crates/gram-ontology/src/main.rs`, with unchanged names and severities (spec A7), keeping `tests/check.rs` passing
- [X] T010 Extend the doc-comment parse in `crates/gram-ontology/src/model.rs` to read each comment's tags into the `Doc` model with positions mapped to the host file: `@language`, `@property` (term, description, `(@key value)` attributes), `@alias`, `@reference`, `@label`, unknown tags as `@`-prefixed fields; parse terms and attribute values (strip quotes, split lists) here, per research R5

**Checkpoint**: `cargo test` passes; the model builds for both spike examples.

---

## Phase 3: User Story 6 - Diagnostics I can act on, and the lint superset (Priority: P1) MVP

**Goal**: one run reports everything `gram lint` reports plus profile findings, in text and JSON, with correct exit status. Profile checks then only have to add to a working pipeline.

**Independent Test**: run the command on a file with a syntax error, a repeated key and an opt-in mismatch; see all three, in file order, identical in text and JSON; exit status follows severity.

### Tests for User Story 6

- [X] T011 [P] [US6] Superset test in `crates/gram-ontology/tests/superset.rs`: for every fixture file and both spike examples, call `gram_data::lint::lint_source` and assert each lint diagnostic (check name, severity, position) appears in the command's JSON output (SC-007, FR-028)
- [X] T012 [P] [US6] Fixtures and CLI test in `crates/gram-ontology/tests/output.rs`: a file with several mistakes prints lines `<path>:<line>:<column>: <Severity> [<check-name>] <message>` in file order; `--json` lists the same diagnostics in the same order and the existing schema (FR-020, FR-022, SC-004, contract)
- [X] T013 [P] [US6] Tests in `crates/gram-ontology/tests/output.rs`: warnings-only file exits 0; any error-severity diagnostic exits 1; several files are reported independently in command-line order (FR-023, FR-024)
- [X] T014 [P] [US6] Test in `crates/gram-ontology/tests/output.rs`: a file with a syntax error reports lint's `syntax-error`/`missing-node` and no profile check produces diagnostics derived from the broken region (FR-027, US6 scenario 6); update the existing `syntax_error_fails` test in `crates/gram-ontology/tests/check.rs` for the renamed check (`parse-error` → `syntax-error`/`missing-node`, FR-021)
- [X] T015 [P] [US6] Test that both spike examples behave as spec Story 6 scenario 5 requires, in `crates/gram-ontology/tests/check.rs`; if `person.ontology.gram` cannot satisfy the profile, decide with the author whether the file or the RFC changes and note the decision in the test Decision (analysis finding C1): `person.ontology.gram` is a discussion file that sets alternatives side by side. It is left unchanged and is expected to be reported; only `movies.ontology.gram` must be clean.

### Implementation for User Story 6

- [X] T016 [US6] Create `crates/gram-ontology/src/lint.rs`: run `gram_data::lint::lint_source`, merge with profile diagnostics, de-duplicate on (check name, range) with lint's finding winning, and when both a lint and a profile finding hit the same range keep lint's (research R2)
- [X] T017 [US6] Rewrite `check_source` in `crates/gram-ontology/src/main.rs` to: run lint first, build the model, run every registered check, sort diagnostics by position, and return them; remove the old `collect_parse_errors` and the `parse-error` name (FR-021)
- [X] T018 [US6] Update text and JSON output in `crates/gram-ontology/src/main.rs` so both come from the same sorted list; put a second location (earlier definition, key or `Doc`) in the diagnostic's `help` field and print it as a `help:` line (contract, data-model Diagnostic)
- [X] T019 [US6] Make T011-T015 pass; run `cargo test`

**Checkpoint**: lint superset and output are complete; no profile check exists beyond the retained ones.

---

## Phase 4: User Story 1 - Identity and references (Priority: P1)

**Goal**: duplicate identifiers, orphans and anonymous `Doc`s are reported.

**Independent Test**: each fixture triggers exactly its check; good fixtures are clean.

### Tests for User Story 1

- [X] T020 [P] [US1] Fixtures `duplicate-identifier.bad|good.ontology.gram`: two `OWNS` definitions (bad); the same with explicit identifiers `ownsCar` and `ownsFleet` (good); test in `crates/gram-ontology/tests/identity.rs` for scenarios 1-2, expecting `duplicate-identifier` (error) at the later definition with the earlier line in `help`
- [X] T021 [P] [US1] Fixtures `orphaned-pattern.bad|good.ontology.gram`: a top-level node, relationship and annotation no `Graph` lists (bad); the same listed in one graph, and one definition shared by two graphs (good); test in `crates/gram-ontology/tests/identity.rs` for scenarios 3 and 5, expecting warnings
- [X] T022 [P] [US1] Fixtures `anonymous-doc.bad|good.ontology.gram` and a test in `crates/gram-ontology/tests/identity.rs` for scenario 4, expecting a warning at a top-level `Doc` with no identity

### Implementation for User Story 1

- [X] T023 [US1] Implement `duplicate-identifier` in `crates/gram-ontology/src/checks/identity.rs` using `by_identifier`: identifier is the explicit identifier, else identifying label (node) or type (relationship); compare exactly, case-sensitive, backticks removed (spec A9, FR-005, FR-006)
- [X] T024 [US1] Implement `orphaned-pattern` and `anonymous-doc` in `crates/gram-ontology/src/checks/identity.rs` using `referenced` (FR-003, FR-004, FR-006); register both in `crates/gram-ontology/src/checks/mod.rs`
- [X] T025 [US1] Make T020-T022 pass; confirm `movies.ontology.gram` produces no new diagnostics

**Checkpoint**: US1 independently testable.

---

## Phase 5: User Story 2 - Labels (Priority: P1)

**Goal**: implied-label problems and `::` misuse are reported.

**Independent Test**: each fixture triggers exactly its check; the two scenario groups for resolution by identifying label are clean.

### Tests for User Story 2

- [X] T026 [P] [US2] Fixtures and tests in `crates/gram-ontology/tests/labels.rs` for `undefined-implied-label` (warning) and resolution by identifying label regardless of identifier: scenarios 1-3
- [X] T027 [P] [US2] Fixtures and tests in `crates/gram-ontology/tests/labels.rs` for `ambiguous-implied-label` (warning): scenario 4
- [X] T028 [P] [US2] Fixtures and tests in `crates/gram-ontology/tests/labels.rs` for `implied-label-cycle` (error): a three-label cycle reported once, a self-cycle `(A::A)`, and a cycle through an optional label (scenario 5 and edge cases)
- [X] T029 [P] [US2] Fixtures and tests in `crates/gram-ontology/tests/labels.rs` for `optional-identifying-label` (error): `` (`Person?`) `` and `` (actor:`Actor?`) `` bad, `` (Actor::`Director?`) `` good (scenario 6)
- [X] T030 [P] [US2] Fixtures and tests in `crates/gram-ontology/tests/labels.rs` for `misplaced-label-separator` (warning): `(actor:Actor::Person)` bad; `(Actor::Person)`, `(actor::Actor:Person)` and a record `name:: cypher`STRING`` good (scenarios 7-9)
- [X] T031 [P] [US2] Fixtures and tests in `crates/gram-ontology/tests/labels.rs` for `relationship-double-colon` (error): `[ownsCar::OWNS]` bad, `[ownsCar:OWNS]` good (scenario 10)

### Implementation for User Story 2

- [X] T032 [US2] Implement `undefined-implied-label` and `ambiguous-implied-label` in `crates/gram-ontology/src/checks/labels.rs` using `by_identifying_label` (FR-013, FR-014)
- [X] T033 [US2] Implement `implied-label-cycle` in `crates/gram-ontology/src/checks/labels.rs` using `label_edges`, one diagnostic per cycle naming the labels in it (FR-015)
- [X] T034 [US2] Implement `optional-identifying-label` in `crates/gram-ontology/src/checks/labels.rs`; a trailing `?` on an implied label is valid (FR-016)
- [X] T035 [US2] Implement `misplaced-label-separator` and `relationship-double-colon` in `crates/gram-ontology/src/checks/labels.rs` from the separator tokens recorded in the model; the `::` between a record key and its type is not a label separator (FR-017, FR-018); register all US2 checks in `crates/gram-ontology/src/checks/mod.rs`
- [X] T036 [US2] Make T026-T031 pass; confirm `movies.ontology.gram` produces no new diagnostics

**Checkpoint**: US1 and US2 both work.

---

## Phase 6: User Story 3 - Property declarations and records (Priority: P1)

**Goal**: value-vs-declaration entries, reserved names, documented properties that do not exist and targetless doc comments are reported.

**Independent Test**: each fixture triggers exactly its check.

### Tests for User Story 3

- [X] T037 [P] [US3] Fixtures and tests in `crates/gram-ontology/tests/records.rs` for `property-value-in-definition` (warning): single colon in a definition's record bad; the same entry in a `Graph` or `Doc` record good (scenarios 2-3); a repeated-key fixture asserts the inherited `duplicate-key` appears with lint's severity (scenario 1)
- [X] T038 [P] [US3] Fixtures and tests in `crates/gram-ontology/tests/records.rs` for `reserved-property-name` (error): `` `@id` `` as a declared property bad; a `Doc`'s own `` `@description` `` good (scenario 4)
- [X] T039 [P] [US3] Fixtures and tests in `crates/gram-ontology/tests/docs.rs` for `documented-property-missing` (error), written `Doc` and doc comment forms, and an existing property on a definition shared between graphs as good (scenarios 5-7)
- [X] T040 [P] [US3] Fixtures and tests in `crates/gram-ontology/tests/docs.rs` for `doc-comment-no-target` (error); a comment on an annotation-wrapped definition is read through the wrappers and is good (scenarios 8-9)

### Implementation for User Story 3

- [X] T041 [US3] Implement `property-value-in-definition` and `reserved-property-name` in `crates/gram-ontology/src/checks/records.rs` over definition records only (FR-019, FR-010)
- [X] T042 [US3] Implement `documented-property-missing` and `doc-comment-no-target` in `crates/gram-ontology/src/checks/docs.rs` using the `Doc` model from `crates/gram-ontology/src/model.rs` (T010), with diagnostics for comments positioned in the host file (FR-007, FR-008, FR-020); register all US3 checks in `crates/gram-ontology/src/checks/mod.rs`
- [X] T043 [US3] Make T037-T040 pass; confirm `movies.ontology.gram` produces no new diagnostics (resolve any `person.ontology.gram` findings per T015)

**Checkpoint**: P1 stories complete.

---

## Phase 7: User Story 4 - Structure annotations (Priority: P2)

**Goal**: cardinality length and self-loop ends are checked.

**Independent Test**: relationships of arity 2 and 3 with correct and incorrect arrays.

### Tests for User Story 4

- [X] T044 [P] [US4] Fixtures and tests in `crates/gram-ontology/tests/structure.rs` for `cardinality-length-mismatch` (error): correct arity-2, one range for two elements, arity-3 correct, annotation wrapped in other annotations (scenarios 1-3, 6)
- [X] T045 [P] [US4] Fixtures and tests in `crates/gram-ontology/tests/structure.rs` for `self-loop-ends-differ` (error): same-node ends good, different ends bad (scenarios 4-5); first confirm the self-loop spelling against RFC §5.3, EXP-003 and `examples/` (research R8) and note it in the fixture

### Implementation for User Story 4

- [X] T046 [US4] Implement `cardinality-length-mismatch` and `self-loop-ends-differ` in `crates/gram-ontology/src/checks/structure.rs` looking through nested annotations to the wrapped definition (FR-011, FR-012); register both in `crates/gram-ontology/src/checks/mod.rs`
- [X] T047 [US4] Make T044-T045 pass

---

## Phase 8: User Story 5 - Indistinguishable `Doc`s (Priority: P2)

**Goal**: `Doc`s that cannot be told apart are reported as warnings, legitimate variants are not.

**Independent Test**: sets of `Doc`s differing by language, audience, graph or nothing.

### Tests for User Story 5

- [X] T048 [P] [US5] Fixtures and tests in `crates/gram-ontology/tests/docs.rs` for `duplicate-doc` (warning): same language and same other `@` fields bad; different `@language` good; same language but different `@audience` good; one comment's `Doc` copied into two graphs good; differing only in description bad (scenarios 1-5)

### Implementation for User Story 5

- [X] T049 [US5] Implement `duplicate-doc` in `crates/gram-ontology/src/checks/docs.rs`: per element per graph, equal `@language` and equal values for every other `@`-prefixed field, ignoring description, labels and property entries; copies sharing an `origin` are exempt (FR-009); register it in `crates/gram-ontology/src/checks/mod.rs`
- [X] T050 [US5] Make T048 pass

---

## Phase 9: Polish and cross-cutting

- [X] T051 [P] Seeded-file test in `crates/gram-ontology/tests/seeded.rs`: one file containing one instance of every mistake lists all of them in a single run at the correct lines (SC-002)
- [X] T052 [P] Check that every §5.6 bullet maps to a check name and every check has a good and a bad fixture, with a test in `crates/gram-ontology/tests/coverage.rs` that fails when a registered check lacks either fixture (SC-001)
- [X] T053 [P] Performance test or script in `crates/gram-ontology/tests/perf.rs`: generate a 10,000-line ontology file and assert the command finishes under 2 s (SC-005); if lint plus the second parse threatens the budget, record the measurement in research R1
- [X] T054 [P] Review every diagnostic message for clarity without the RFC (SC-006); fix wording in `crates/gram-ontology/src/checks/` Done during implementation; no separate record.
- [X] T055 [P] Update `README.md` (or crate docs) with the check catalogue and exit-status rule, and the fixed-severity note (spec A2); no new configuration
- [X] T056 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test`, then walk `specs/001-ontology-check-rfc-001/quickstart.md` end to end

---

## Dependencies and execution order

- **Phase 1 → Phase 2 → everything else.** T001-T003 are sequential (T002 and T003 need the dependency added in T001). T004-T005 are parallel.
- **Phase 2**: T006 before T007; T008 before T009; T010 needs T006. T007, T008 and T010 can overlap once T006 exists.
- **US6 (Phase 3)** needs Phase 2 and blocks nothing technically, but is done first so later stories report through the finished pipeline.
- **US1-US5** depend only on Phases 1-2 and on each other not at all, except that US3's `documented-property-missing` needs T010. They can proceed in parallel once US6 lands; within a story, tests (marked [P]) come first and must fail before implementation.
- **Same-file conflicts**: tasks touching `crates/gram-ontology/src/checks/mod.rs` (T008, T024, T035, T042, T046, T049) and `crates/gram-ontology/src/checks/docs.rs` (T042, T049) must not run in parallel.
- **Polish (Phase 9)** needs every story phase.

## Parallel example: User Story 2

Launch together, since each writes its own fixtures and its own tests in one file's separate functions (coordinate on `labels.rs` merges):

```text
T026 undefined / ambiguous tests   T027 ambiguous tests   T028 cycle tests
T029 optional-identifying tests    T030 separator tests   T031 relationship tests
```

Then T032-T035 sequentially in `crates/gram-ontology/src/checks/labels.rs`.

## Implementation strategy

- **MVP**: Phase 1, Phase 2 and US6 (T001-T019). The result is a command that is a strict superset of `gram lint` with the retained checks and the final output contract. It already delivers value and every later story adds checks to it.
- **First increment**: add US1, US2 and US3 (all P1) one story at a time, validating each against its Independent Test and the spike examples.
- **Then**: US4 and US5 (P2), then Polish.
- Commit after each task or story checkpoint. Stop and consult the author on: the `gram-data` dependency weight (T003), a `person.ontology.gram` that cannot satisfy the profile (T015), and an unresolved self-loop spelling (T045).
