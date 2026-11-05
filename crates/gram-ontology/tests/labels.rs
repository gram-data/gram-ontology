mod common;
use common::*;

#[test]
fn undefined_implied_label() {
    assert_only(
        "undefined-implied-label.bad",
        "undefined-implied-label",
        "warning",
        "(Actor::Person)",
    );
    // `Person` resolves by identifying label whatever the identifier: `actor:Performer:Person`.
    assert_clean("undefined-implied-label.good");
}

#[test]
fn ambiguous_implied_label() {
    assert_only(
        "ambiguous-implied-label.bad",
        "ambiguous-implied-label",
        "warning",
        "(Actor::Person)",
    );
    assert_clean("ambiguous-implied-label.good");
}

#[test]
fn implied_label_cycle() {
    assert_only(
        "implied-label-cycle.bad",
        "implied-label-cycle",
        "error",
        "(A::B)",
    );
    assert_eq!(count("implied-label-cycle.bad", "implied-label-cycle"), 1);
    assert_only(
        "implied-label-cycle.self.bad",
        "implied-label-cycle",
        "error",
        "(A::A)",
    );
    // `?` marks optionality, not a different label, so the cycle is still reported.
    assert_only(
        "implied-label-cycle.optional.bad",
        "implied-label-cycle",
        "error",
        "(A::`B?`)",
    );
    assert_clean("implied-label-cycle.good");
}

#[test]
fn optional_identifying_label() {
    assert_only(
        "optional-identifying-label.bad",
        "optional-identifying-label",
        "error",
        "(`Person?`)",
    );
    assert_eq!(
        count(
            "optional-identifying-label.bad",
            "optional-identifying-label"
        ),
        2
    );
    assert_clean("optional-identifying-label.good");
}

#[test]
fn misplaced_label_separator() {
    assert_only(
        "misplaced-label-separator.bad",
        "misplaced-label-separator",
        "warning",
        "(actor:Actor::Person)",
    );
    // `::` directly after the identifier, and the `::` of a property declaration, are fine.
    assert_clean("misplaced-label-separator.good");
}

#[test]
fn relationship_double_colon() {
    assert_only(
        "relationship-double-colon.bad",
        "relationship-double-colon",
        "error",
        "[ownsCar::OWNS]",
    );
    assert_clean("relationship-double-colon.good");
}

#[test]
fn misplaced_label_separator_ignores_the_record_key_double_colon() {
    // `name:: cypher`STRING`` is a record entry, not a label separator.
    assert_clean("misplaced-label-separator.record-key.good");
    let (_, diags) = run_fixture("misplaced-label-separator.record-key.bad");
    assert!(
        diags.iter().all(|d| d.rule == "misplaced-label-separator"),
        "{diags:?}"
    );
    let line = line_of("misplaced-label-separator.record-key.bad", "(actor:Actor::");
    assert_eq!(diags.iter().map(|d| d.line).collect::<Vec<_>>(), [line]);
}
