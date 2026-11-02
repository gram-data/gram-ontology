mod common;
use common::*;

#[test]
fn duplicate_identifier() {
    assert_only(
        "duplicate-identifier.bad",
        "duplicate-identifier",
        "error",
        "(Company)=[OWNS]",
    );
    assert_clean("duplicate-identifier.good");
}

#[test]
fn orphaned_pattern() {
    assert_only(
        "orphaned-pattern.bad",
        "orphaned-pattern",
        "warning",
        "(Movie)",
    );
    assert_clean("orphaned-pattern.good");
    // A relationship and an annotation-wrapped definition are orphaned too.
    let (_, diags) = run_fixture("orphaned-pattern.relationship.bad");
    assert!(
        diags.iter().all(|d| d.rule == "orphaned-pattern"),
        "{diags:?}"
    );
    let lines: Vec<u32> = diags.iter().map(|d| d.line).collect();
    assert_eq!(
        lines,
        [
            line_of("orphaned-pattern.relationship.bad", "(Person)=[ACTED_IN]"),
            line_of("orphaned-pattern.relationship.bad", "(Show"),
        ]
    );
}

#[test]
fn anonymous_doc() {
    assert_only("anonymous-doc.bad", "anonymous-doc", "warning", "[:Doc");
    assert_clean("anonymous-doc.good");
}
