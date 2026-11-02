# Quickstart: validating the feature

Prerequisites: Rust toolchain, this branch checked out.

1. Run the full suite: `cargo test`. Expect all existing tests, the per-check fixture tests and the superset test to pass.
2. Spike examples (SC-003): `cargo run -q -p gram-ontology -- check examples/pattern-rs-spike/movies.ontology.gram` prints nothing and exits 0. `person.ontology.gram` is a discussion file that sets alternatives side by side, so it is reported (`opt-in-mismatch`, `duplicate-identifier`, `property-value-in-definition`, and more) and exits 1; it is left unchanged on purpose.
3. One mistake per check: run the command on each file in `crates/gram-ontology/tests/fixtures/*.bad.ontology.gram`. Each prints exactly its check name at the expected line; `*.good.ontology.gram` files print nothing.
4. Seeded file (SC-002): `tests/fixtures/seeded.ontology.gram` contains an instance of every mistake lists all of them in a single run, in line order.
5. Text and JSON agree (SC-004): `... check --json file | jq '.files[].diagnostics | length'` equals the number of text lines.
6. Exit status: a warnings-only file exits 0; a file with any error-severity diagnostic exits 1.
7. Performance (SC-005): `time` a generated 10,000-line ontology file; under 2 s.
