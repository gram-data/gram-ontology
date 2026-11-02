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
}

#[test]
fn anonymous_doc() {
    assert_only("anonymous-doc.bad", "anonymous-doc", "warning", "[:Doc");
    assert_clean("anonymous-doc.good");
}
