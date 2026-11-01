# Examples

## `pattern-rs-spike/`

Verbatim copies of the files from the ontology spike on the `spike/ontology-graph-spec-gram` branch of [relateby/pattern-rs](https://github.com/relateby/pattern-rs) (`data/schema/ontology/`), kept as seed fixtures:

- `person.ontology.gram`: a node documented with gramdoc, plus a French context variant.
- `movies.ontology.gram`: a translation of the ontology-graph-spec's `movies.yaml` example, with `kind: "ontology"` in the top-level record. It documents meaning with separate `Meaning` patterns attached by `@on(...)`, which predates the gramdoc approach.
- `movies.yaml`: the spec's example, for comparison.
- `README.md`: the spike's own notes.

The `movies.ontology.gram` header also carries a `schema` field that points at the ontology-graph-spec's JSON Schema. That is a mistake and not a convention to follow: once gram has a schema definition, `kind` is what associates a file with it, so a file does not name a schema. The file is left verbatim as provenance.

`movies.ontology.gram` also predates the identity convention EXP-004 settles. Its identity is the spec's node id (`(person::Person {...})`) with the identifying label after `::`, while the ontology profile takes the identifier itself as the identifying label (`(Person {...})`), so it has to be converted before it is a model of the profile.

They are exploratory and may not match the model this project settles on. Both `.ontology.gram` files parse with tree-sitter-gram with no errors, and the integration tests in `crates/gram-ontology/tests/` run `gram-ontology check` against them.
