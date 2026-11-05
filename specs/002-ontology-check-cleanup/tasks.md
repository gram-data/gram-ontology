# Tasks: `gram-ontology check` cleanup against RFC-001 §5.6

**Input**: `specs/002-ontology-check-cleanup/` (spec.md, plan.md, research.md, data-model.md, contracts/). Tests are written first: each fixture must fail before its fix. Paths are relative to the repo root; `T` = `crates/gram-ontology`.

## Phase 1: Setup

- [x] T001 Run `cargo test -p gram-ontology` on the branch to record the passing baseline.

## Phase 2: User Story 1 - undefined-reference (P1)

**Independent test**: `undefined-reference.bad` reports one error at `OWNS`; `.good` is clean.

- [x] T002 [P] [US1] Add `T/tests/fixtures/undefined-reference.bad.ontology.gram`: `(Person)=[ownsCar:OWNS]=>(Car)` with `[:Doc {...} | OWNS]` inside a `Graph`; also a mistyped Graph member and an annotated undefined reference.
- [x] T003 [P] [US1] Add `T/tests/fixtures/undefined-reference.good.ontology.gram`: references by explicit identifier (`ownsCar`), a node's own identifier, a `Graph` identifier and a `Doc` identifier, and a doc comment on a definition; must be clean.
- [x] T004 [US1] Add an `undefined_reference` test in `T/tests/identity.rs` using `assert_only`/`assert_clean` and `count` (expect the failing state first).
- [x] T005 [US1] In `T/src/model.rs`, record each reference (Graph member, Doc target, annotation-wrapped) with its range, and the identifiers of `Graph` and `Doc` patterns; add a unit test.
- [x] T006 [US1] In `T/src/checks/identity.rs`, report `undefined-reference` (message names the identifier; help mentions referring by explicit identifier); never for doc-comment implicit references or unparsed regions.
- [x] T007 [US1] Add `("undefined-reference", Severity::Error)` to the table in `T/src/checks/mod.rs`.
- [x] T008 [P] [US1] List `undefined-reference` in `README.md` (Errors) and in `specs/001-ontology-check-rfc-001/contracts/check-command.md` (catalogue).

## Phase 3: User Story 2 - syntax error once (P2)

- [x] T009 [P] [US2] Add `T/tests/fixtures/syntax-error.bad.ontology.gram` (`[g:Graph {name:"g"} | Actor?, Person]` after `{ kind: "ontology" }`) and a test in `T/tests/check.rs` asserting exactly one `syntax-error` at the `?`, plus a file with two different syntax errors keeps both.
- [x] T010 [US2] In `T/src/lint.rs`, drop lint findings identical in rule, range and message to an earlier one; add a unit test.

## Phase 4: User Story 3 - fixtures (P3)

- [x] T011 [P] [US3] Add `T/tests/fixtures/duplicate-identifier.shared-name.bad.ontology.gram` (`(Follows)` and `[Follows]`) and `.shared-name.good.ontology.gram` (explicit identifiers); wire into `T/tests/identity.rs`.
- [x] T012 [P] [US3] Add `T/tests/fixtures/misplaced-label-separator.record-key.good.ontology.gram` (`name:: cypher`STRING`` in a node record, clean) and a `.record-key.bad.` variant adding a misplaced `::`; wire into `T/tests/labels.rs`.

## Phase 5: User Story 4 - help text (P3)

- [x] T013 [US4] Add tests (in `T/tests/identity.rs`) that node clash help contains `actor:Actor`, relationship clash contains `ownsCar:OWNS`; then change `T/src/checks/identity.rs` to pick the example by the reported definition's kind.

## Phase 6: Polish

- [x] T014 Update `T/tests/fixtures/README.md` if naming needs a note; mark A6 superseded (done in spec).
- [x] T015 Run `cargo fmt --check`, `cargo clippy --all-targets`, `cargo test`; confirm `examples/pattern-rs-spike/movies.ontology.gram` is still clean.

## Dependencies

T002-T004 before T005-T007; T008 independent. US2, US3 and US4 are independent of US1 and of each other (US3 and US4 both touch `tests/identity.rs`; do them in order). T015 last.

## MVP

US1 alone.
