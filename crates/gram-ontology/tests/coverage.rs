//! Every registered check has a passing and a failing fixture (spec SC-001).
use std::collections::BTreeSet;

#[test]
fn every_check_has_a_good_and_a_bad_fixture() {
    let src =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/checks/mod.rs")).unwrap();
    let rules: BTreeSet<String> = src
        .lines()
        .filter_map(|l| l.trim().strip_prefix("(\""))
        .filter_map(|l| l.split('"').next())
        .map(String::from)
        .collect();
    // Covered by tests in check.rs: they need a file name, a missing file, or invalid gramdoc.
    let exempt = ["opt-in-mismatch", "gramdoc-parse-error", "io-error"];
    assert!(rules.len() >= 16, "found {rules:?}");
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
    for rule in rules.iter().filter(|r| !exempt.contains(&r.as_str())) {
        for kind in ["good", "bad"] {
            let p = format!("{dir}/{rule}.{kind}.ontology.gram");
            assert!(std::path::Path::new(&p).exists(), "missing fixture {p}");
        }
    }
}
