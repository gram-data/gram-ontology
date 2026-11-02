# Implementation Plan: `gram-ontology check` covers every RFC-001 §5.6 check

**Branch**: `001-ontology-check-rfc-001` | **Date**: 2026-11-02 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-ontology-check-rfc-001/spec.md`

## Summary

Extend the existing `gram-ontology check` command so that one run reports (a) everything `gram lint` reports, by calling the same lint implementation, and (b) the 17 profile checks of RFC-001 §5.6 that lint does not make. Profile checks work on the tree-sitter-gram concrete syntax tree through one shared model of the file (definitions, graphs, `Doc`s, annotations), built once and read by every check, so that identifier, label and property resolution is done in one place. Diagnostics use the shared `gram-diagnostics` schema the command already emits.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021 (existing workspace)

**Primary Dependencies**: `tree-sitter` 0.26, `tree-sitter-gram` 0.6 (0.6.1 is released and compatible), `clap` 4.5 (existing); `tree-sitter-gramdoc` bumped 0.2 → 0.3 (released; the repo is still on 0.2); `gram-diagnostics` bumped 0.4 → 0.6; `gram-data` 0.6 added as a library dependency for `lint::lint_source` (see research R1)

**Storage**: N/A (reads files, writes only stdout)

**Testing**: `cargo test`; `assert_cmd` + `tempfile` CLI tests (existing `tests/check.rs`), plus unit tests per check on small inline sources

**Target Platform**: macOS and Linux developer machines and CI; installed through the gram extension registry

**Project Type**: single CLI crate in a Cargo workspace (`crates/gram-ontology`)

**Performance Goals**: 10,000-line file in under 2 s (SC-005)

**Constraints**: standalone (spec A4); no network or other tool required at run time; never modifies files; text and JSON output carry identical diagnostics

**Scale/Scope**: single files of up to tens of thousands of lines; several files per invocation

## Constitution Check

`.specify/memory/constitution.md` is still the unfilled template (all placeholders), so it defines no gates. There is nothing to pass or fail; recorded as a gap, not a violation. Working principles taken from the spec and RFC instead: standalone and a superset of lint (A4), the RFC is the authority, tests first for every check (each check has a failing and a passing scenario, SC-001).

Re-check after design: unchanged.

## Project Structure

### Documentation (this feature)

```text
specs/001-ontology-check-rfc-001/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── check-command.md
└── tasks.md             # created by /speckit-tasks, not by this command
```

### Source Code (repository root)

```text
crates/gram-ontology/
├── Cargo.toml                # gram-diagnostics 0.6, gram-data 0.6, tree-sitter-gramdoc 0.3
├── src/
│   ├── main.rs               # CLI, output, exit status (existing)
│   ├── model.rs              # file model built once from the tree
│   ├── checks/               # one module per check family, each a function over the model
│   │   ├── mod.rs            # registry: check name, severity, function
│   │   ├── identity.rs       # duplicate-identifier, orphaned-pattern, anonymous-doc
│   │   ├── labels.rs         # undefined/ambiguous implied label, cycle, optional, separators
│   │   ├── records.rs        # property-value-in-definition, reserved-property-name
│   │   ├── docs.rs           # documented-property-missing, doc-comment-no-target, duplicate-doc
│   │   └── structure.rs      # cardinality-length-mismatch, self-loop-ends-differ
│   └── lint.rs               # runs gram lint, merges and de-duplicates with profile findings
└── tests/
    ├── check.rs              # existing CLI tests
    └── fixtures/             # one small file per scenario, named after the check
```

**Structure Decision**: stay in the single existing crate; split the 268-line `main.rs` only as far as the model and checks require. Checks are pure functions from the model to diagnostics, with severity held in one table (spec A1: severities easy to change).

## Complexity Tracking

No constitution violations. One notable trade-off is recorded in research R1 (taking the whole `gram-data` crate for its lint function).
