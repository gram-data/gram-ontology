# Data Model: file model for profile checks

Built once per file from the tree-sitter-gram CST (research R3, R4); read by every check. Positions are byte ranges mapped to line/column.

## Definition
A top-level node or relationship pattern (looking through `annotated_pattern`).
- `kind`: node | relationship
- `identifier`: explicit identifier if present, else identifying label (node) or type (relationship)
- `identifying_label` (node): first label after `::` shorthand expansion
- `implied_labels`: later labels, each with `optional` (trailing `?`) and the separator token used
- `type` (relationship)
- `record`: entries (`key`, `separator` `:` | `::`, `value`, position)
- `annotations`: wrapping annotations, outermost first
- `ends` (relationship): the node identities at each position
- `range`

## Graph
A top-level pattern whose elements list members by identity (`pattern_reference`). Holds `members` (identifiers) and a record.

## Doc
A `:Doc` pattern or a doc comment read into one.
- `target` identifier (or none, for `anonymous-doc`)
- `graph` it lives in (copies from one comment in several graphs share `origin`)
- `origin`: written pattern | doc comment (comment position)
- `fields`: `@`-prefixed fields (`@language`, others)
- `property_entries`: documented property names
- `label_entries`

## Annotation
`name` (`cardinality`, `unique`, self-loop, …), `arguments`, `range`.

## Derived indexes
- `by_identifier`: identifier → definitions (more than one is `duplicate-identifier`)
- `by_identifying_label`: label → node definitions (none: `undefined-implied-label`; more than one: `ambiguous-implied-label`)
- `label_edges`: identifying label → implied labels (cycles reported once each)
- `referenced`: identifiers named by any graph (the complement among definitions is `orphaned-pattern`)

## Diagnostic
As `gram_diagnostics::Diagnostic`: `severity`, `rule` (check name), `message`, `range`, optional `help`. The second location of duplicate-class checks goes in `help` (spec A8 resolved: the type has a `help` field).
