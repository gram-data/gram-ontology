//! One file with an instance of every mistake lists all of them in one run (spec SC-002).
mod common;
use common::*;

#[test]
fn every_check_reports_in_a_single_run() {
    let (code, diags) = run_fixture("seeded");
    let rules: std::collections::BTreeSet<&str> = diags.iter().map(|d| d.rule.as_str()).collect();
    for rule in [
        "duplicate-key",
        "duplicate-identifier",
        "orphaned-pattern",
        "anonymous-doc",
        "undefined-implied-label",
        "ambiguous-implied-label",
        "implied-label-cycle",
        "optional-identifying-label",
        "misplaced-label-separator",
        "relationship-double-colon",
        "property-value-in-definition",
        "reserved-property-name",
        "documented-property-missing",
        "doc-comment-no-target",
        "cardinality-length-mismatch",
        "self-loop-ends-differ",
        "duplicate-doc",
    ] {
        assert!(rules.contains(rule), "`{rule}` missing from {diags:?}");
    }
    assert_eq!(code, 1);
    assert!(
        diags.windows(2).all(|w| w[0].line <= w[1].line),
        "not in file order"
    );
    assert_eq!(
        diags
            .iter()
            .find(|d| d.rule == "duplicate-identifier")
            .unwrap()
            .line,
        line_of("seeded", "(Twin)") + 1
    );
}
