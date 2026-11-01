# Examples

## `pattern-rs-spike/`

Copies of the files from the ontology spike on the `spike/ontology-graph-spec-gram` branch of [relateby/pattern-rs](https://github.com/relateby/pattern-rs) (`data/schema/ontology/`), kept as seed fixtures:

- `person.ontology.gram`: a node documented with gramdoc, plus a French context variant.
- `movies.ontology.gram`: a translation of the ontology-graph-spec's `movies.yaml` example, with `kind: "ontology"` in the top-level record.
- `movies.yaml`: the spec's example, for comparison.
- `README.md`: the spike's own notes.

`person.ontology.gram` is the spike's file with label and `Doc` examples added. `movies.ontology.gram` was rewritten to follow EXP-003 and EXP-004: the identifier is the identifying label, relationships are identified by their type, documentation is gramdoc comments, the members are listed by a `Graph` pattern, and there is no `schema` field (`kind` is what will associate the file with a gram schema).

They are exploratory and may not match the model this project settles on. Both `.ontology.gram` files parse with tree-sitter-gram with no errors, and the integration tests in `crates/gram-ontology/tests/` run `gram-ontology check` against them.
