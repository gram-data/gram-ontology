# Quickstart

```bash
cargo test -p gram-ontology
cargo run -q -p gram-ontology -- check crates/gram-ontology/tests/fixtures/undefined-reference.bad.ontology.gram   # one undefined-reference error
cargo run -q -p gram-ontology -- check crates/gram-ontology/tests/fixtures/undefined-reference.good.ontology.gram  # clean
```

A file with `[g:Graph {name:"g"} | Actor?, Person]` reports one `syntax-error`. `cargo fmt --check` and `cargo clippy` pass.
