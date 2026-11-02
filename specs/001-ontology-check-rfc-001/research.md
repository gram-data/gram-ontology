# Research: `gram-ontology check` and RFC-001 §5.6

## R1. Reusing `gram lint`

- **Decision**: depend on `gram-data` 0.6 as a library and call `gram_data::lint::lint_source(source, &LintOptions)`, merging its diagnostics with the profile checks. Bump `gram-diagnostics` to 0.6.
- **Rationale**: the lint implementation is public in `gram-data` and returns `gram_diagnostics::Diagnostic`, the type this command already emits, so the two cannot disagree (spec A4, FR-028). `gram-diagnostics` 0.4.1 and 0.6.0 have identical sources but are different crates to the compiler, so the dependency must move to 0.6 for the types to unify. gram-data's rules today: `syntax-error`, `missing-node`, `duplicate-element`, `duplicate-key`.
- **Cost**: `gram-data` is one crate with the `gram` CLI, so it also brings `reqwest`, `zip`, `tar`, `flate2`, `toml`, `ariadne` and others used by extension install. This only costs build time and binary size.
- **Alternatives**: (a) reimplement `duplicate-key` and `duplicate-element` here: duplicates code, can drift from lint (violates FR-029); (b) shell out to `gram lint`: contradicts standalone (A4); (c) ask for a slimmer `gram-lint` crate upstream: better long-term, out of scope now. If the dependency's weight proves unacceptable during implementation, fall back to (c) and record it.
- **Open risk**: lint parses the source itself, so the file is parsed twice. Acceptable for SC-005; measure.

## R2. De-duplicating lint and profile findings

- **Decision**: key findings on (check name, range). Lint findings win. `duplicate-identifier` (profile) and `duplicate-element` (lint) overlap but are kept as separate rules (author decision); if both land on the same range, only lint's is shown (FR-028).
- **Alternative**: always show both. Rejected: noisy for the same cause.

## R3. Reading `::` from the tree

- **Finding**: tree-sitter-gram accepts `:` or `::` as a label separator and as a record key separator (`grammar.js`: `choice(":", "::")`), and `--tree` output does not distinguish them: `(Actor::Person)` shows identifier `Actor`, labels `(Person)`. The separator is an anonymous token node, readable by walking all children.
- **Decision**: the model records each separator token's text and position. Shorthand expansion (`(Actor::Person)` means identifier `Actor` and identifying label `Actor`, implied `Person`) is done by the tool in the model, not by the grammar.
- **Consequence**: `misplaced-label-separator`, `relationship-double-colon` and `property-value-in-definition` need no grammar change.

## R4. Tree node kinds available

Named kinds relevant here: `node_pattern`, `subject_pattern` (`[...|...]`, with `identifier`, `labels`, `record`, `elements`), `relationship_pattern` (`left`, `right`, `kind`), `annotated_pattern` (`annotations`, `elements`), `identified_annotation`, `property_annotation`, `pattern_reference`, `record`, `record_property` (`key`, `value`), `map`, `range`, `array`, `doc_comment`, `labels`, `symbol`, `quoted_name`. Enough for every check; no tree-sitter-gram change needed.

## R5. Doc comments

- **Decision**: reuse the existing gramdoc parse in `check_doc_comments` and extend it to read each comment's tags (`@property`, `@language`, `@alias`, `@reference`, `@label`) into the same `Doc` shape as a written-out `Doc` pattern, with positions mapped back to the host file (FR-020).
- **Open**: the exact tag grammar comes from `tree-sitter-gramdoc` 0.2 and the gramdoc 0.3.0 hand-over note in the shared notes folder. Check it at task time. Not a blocker.

## R6. Check order and failure policy

- **Decision**: run lint first (it carries syntax errors). If the tree has errors, profile checks skip regions that failed to parse (FR-027) but still run on well-formed top-level patterns.
- **Rationale**: matches FR-027 and avoids misleading diagnostics.

## R7. Where severities live

- **Decision**: one table mapping check name → severity (spec severity table), consulted when building a diagnostic. No configuration (spec A2).

## R8. Self-loop spelling

- **Open**: RFC §7.2 (`@self`) is unresolved. The plan targets whatever form the RFC examples and `tests` use; if none exists, the first task is to confirm the spelling against `examples/` and EXP-003. The detection is isolated in `checks/structure.rs` so a change touches one place.

## R9. Test strategy

- One fixture per scenario in `tests/fixtures/`, named `<check-name>.<good|bad>.ontology.gram`; a CLI test asserts check name, severity, line and exit status. The two spike examples must stay clean (FR-026, SC-003). A superset test runs `lint_source` on every fixture and asserts each lint diagnostic appears in the command's output (SC-007).
