# gram-ontology

A planned extension to the [`gram` CLI](https://github.com/gram-data/tree-sitter-gram) for working with `*.ontology.gram` files: strict syntactic and semantic checks over valid gram, and export of the ontology as JSON.

`gram-ontology check <file>...` reports everything `gram lint` reports, plus the checks RFC-001 §5.6 requires of a conforming `*.ontology.gram` file. It runs on its own: it needs neither `gram` nor a prior lint run. Diagnostics use the same JSON schema as `gram lint --json` (`--json`), or one line each: `path:line:column: Severity [check-name] message`. The command exits 1 if any diagnostic is an error, and 0 if there are only warnings.

Severities are fixed per check; there is no configuration yet. The full list, and what each check means, is in [`specs/001-ontology-check-rfc-001/contracts/check-command.md`](specs/001-ontology-check-rfc-001/contracts/check-command.md). Warnings: `opt-in-mismatch`, `orphaned-pattern`, `anonymous-doc`, `misplaced-label-separator`, `undefined-implied-label`, `ambiguous-implied-label`, `property-value-in-definition`, `duplicate-doc`. Errors: `duplicate-identifier`, `documented-property-missing`, `doc-comment-no-target`, `cardinality-length-mismatch`, `self-loop-ends-differ`, `implied-label-cycle`, `optional-identifying-label`, `relationship-double-colon`, `reserved-property-name`, `gramdoc-parse-error`. Findings inherited from `gram lint` (`syntax-error`, `missing-node`, `duplicate-element`, `duplicate-key`) keep lint's names and severities.

```bash
cargo run -p gram-ontology -- check examples/pattern-rs-spike/movies.ontology.gram
```

## Layout

| Path | Contents |
|---|---|
| `crates/gram-ontology/` | The `gram-ontology` binary |
| `design/` | Design documents: [Explorations and RFCs](design/README.md), including [EXP-001](design/exploration/EXP-001-structural-ontologies.md) and [EXP-002](design/exploration/EXP-002-ontology-tooling-over-the-cst.md) |
| `examples/` | Seed fixtures, see [`examples/README.md`](examples/README.md) |
| `.claude/skills/` | `/exploration-create`, `/exploration-review`, `/rfc-create`, `/rfc-review` |
| `.github/workflows/` | CI, and the release workflow that builds the registry-style artifacts |

## Releases

Pushing a `v*` tag builds `gram-ontology-<target-triple>.<tar.gz|zip>` for the five targets `gram extension install` supports and attaches them to a GitHub release. After the first release, add an entry to the registry in [gram-data/gram-extensions](https://github.com/gram-data/gram-extensions).
