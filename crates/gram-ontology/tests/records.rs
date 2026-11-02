mod common;
use common::*;

#[test]
fn property_value_in_definition() {
    assert_only(
        "property-value-in-definition.bad",
        "property-value-in-definition",
        "warning",
        "name: \"STRING\"",
    );
    // Single-colon entries are fine in a Graph's and a Doc's records.
    assert_clean("property-value-in-definition.good");
    // The same mistake in a relationship's record.
    assert_only(
        "property-value-in-definition.relationship.bad",
        "property-value-in-definition",
        "warning",
        "role: \"S\"",
    );
}

#[test]
fn repeated_key_is_reported_by_the_inherited_lint_check() {
    let (code, diags) = run_fixture("duplicate-key.bad");
    let line = line_of("duplicate-key.bad", "name::");
    assert!(
        diags
            .iter()
            .any(|d| d.rule == "duplicate-key" && d.line == line),
        "{diags:?}"
    );
    assert_eq!(code, 1);
    assert_clean("duplicate-key.good");
}

#[test]
fn reserved_property_name() {
    assert_only(
        "reserved-property-name.bad",
        "reserved-property-name",
        "error",
        "`@id`::",
    );
    // A Doc's own `@description` is metadata, not a property name.
    assert_clean("reserved-property-name.good");
}
