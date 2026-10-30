# gram-ontology

A planned extension to the [`gram` CLI](https://github.com/gram-data/tree-sitter-gram) for working with `*.ontology.gram` files: strict syntactic and semantic checks over valid gram, and export of the ontology as JSON.

Early stage. Today `gram-ontology check <file>...` is a skeleton: it parses a file with tree-sitter-gram, parses each `/** ... */` doc comment with [tree-sitter-gramdoc](https://github.com/gram-data/tree-sitter-gramdoc), and warns when the `*.ontology.gram` filename and `kind: ontology` in the top-level record disagree. Diagnostics use the same JSON schema as `gram check --json`. There are no ontology rules yet.

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
