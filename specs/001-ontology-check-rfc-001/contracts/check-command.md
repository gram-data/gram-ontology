# Contract: `gram-ontology check`

```text
gram-ontology check [--json] <FILE>...
```

## Behavior
- Each file is checked independently; results are reported per file in command-line order.
- The command reads files and writes only to stdout. It never modifies files.
- Output includes every finding `gram lint` makes on the file (FR-028) and every profile check below.

## Exit status
- `0`: no error-severity diagnostics (warnings allowed).
- `1`: at least one error-severity diagnostic.

## Text output
One line per diagnostic, in file order:
`<path>:<line>:<column>: <Severity> [<check-name>] <message>`
Lines and columns are 1-based. A second location, where relevant, is appended on the following line as `help: ...`.

## JSON output (`--json`)
Same content and order as the text output, in the existing schema: `{ schema_version: 1, tool: "gram-ontology", files: [{ path, diagnostics: [{ severity, rule, message, range: {start:{line,character}, end:{...}}, code?, help? }] }] }`. Ranges are 0-based, as the schema defines.

## Check catalogue

| Check name | Severity | Source |
|------------|----------|--------|
| `syntax-error`, `missing-node`, `duplicate-element`, `duplicate-key` | as `gram lint` | inherited |
| `io-error` | error | existing |
| `gramdoc-parse-error` | error | existing |
| `opt-in-mismatch` | warning | RFC §5.1 |
| `orphaned-pattern` | warning | RFC §5.1 |
| `anonymous-doc` | warning | RFC §5.1 |
| `misplaced-label-separator` | warning | RFC §5.4 |
| `undefined-implied-label` | warning | RFC §5.4 |
| `ambiguous-implied-label` | warning | RFC §5.4 |
| `property-value-in-definition` | warning | RFC §5.5 |
| `duplicate-doc` | warning | clarified |
| `duplicate-identifier` | error | clarified |
| `undefined-reference` | error | RFC §5.6 (feature 002) |
| `documented-property-missing` | error | clarified |
| `doc-comment-no-target` | error | clarified |
| `cardinality-length-mismatch` | error | clarified |
| `self-loop-ends-differ` | error | clarified |
| `implied-label-cycle` | error | clarified |
| `optional-identifying-label` | error | clarified |
| `relationship-double-colon` | error | clarified |
| `reserved-property-name` | error | clarified |

Check names are stable; the earlier `parse-error` is retired in favor of lint's `syntax-error` and `missing-node` (no other consumers).
