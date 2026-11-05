# Implementation Plan: `gram-ontology check` cleanup against RFC-001 §5.6

**Branch**: `002-ontology-check-cleanup` | **Date**: 2026-11-02 | **Spec**: [spec.md](spec.md)

## Summary

Four small changes to `crates/gram-ontology`: a new `undefined-reference` check (error) read from the existing file model, a once-only filter on lint's findings, a help text chosen by definition kind, and four new fixture files wired into existing tests. No dependency or schema change.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021 (existing). **Dependencies**: unchanged. **Testing**: `cargo test` (`assert_cmd` fixture tests in `crates/gram-ontology/tests/`, unit tests in `src/`), plus `cargo fmt --check` and `cargo clippy`. **Project Type**: one CLI crate. **Constraints**: severities live in the one table in `checks/mod.rs`; text and JSON output identical; 10,000-line file under 2 s (feature 001 SC-005) must still hold.

## Constitution Check

`.specify/memory/constitution.md` is still the unfilled template, so there are no gates (recorded as a gap, as in feature 001). Working principles: RFC-001 is the authority, tests first, severities in one table. Unchanged after design.

## Project Structure

```text
specs/002-ontology-check-cleanup/   spec.md plan.md research.md data-model.md quickstart.md contracts/ tasks.md
crates/gram-ontology/src/model.rs          record references (graph members, Doc targets) with ranges
crates/gram-ontology/src/checks/identity.rs   undefined-reference; duplicate-identifier help by kind
crates/gram-ontology/src/checks/mod.rs     severity table entry
crates/gram-ontology/src/lint.rs           drop repeated lint findings
crates/gram-ontology/tests/                fixtures + wiring
README.md, specs/001-.../contracts/check-command.md   list the check
```

**Structure Decision**: extend the existing modules; no new module is needed.

## Design decisions (detail in research.md)

1. **undefined-reference** works from the model: collect every reference (a bare identifier among a `Graph`'s elements, including one wrapped by annotations; a `Doc`'s target) with its range, then compare against the identifier set (definitions' identifiers, `Graph` and `Doc` identifiers). Doc comments' implicit references are never collected.
2. **Once-only syntax errors**: the repeat comes from the lint library, so the filter goes where lint's findings enter (`lint::lint`): drop a finding identical to an earlier one in rule, range and message.
3. **Help text**: pick the example from the reported definition's kind.
4. **Fixtures**: `undefined-reference.{good,bad}`, `duplicate-identifier.shared-name.{good,bad}`, `misplaced-label-separator.record-key.good` (the existing bad file covers the separator) and `syntax-error.bad`, each wired into the existing test files.
