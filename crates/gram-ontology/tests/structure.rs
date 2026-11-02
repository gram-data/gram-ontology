mod common;
use common::*;

#[test]
fn cardinality_length_mismatch() {
    assert_only(
        "cardinality-length-mismatch.bad",
        "cardinality-length-mismatch",
        "error",
        "@cardinality",
    );
    // Arity two and arity three, the latter under a second annotation.
    assert_clean("cardinality-length-mismatch.good");
    // Two ranges for three pattern elements.
    assert_only(
        "cardinality-length-mismatch.arity3.bad",
        "cardinality-length-mismatch",
        "error",
        "@cardinality",
    );
}

#[test]
fn self_loop_ends_differ() {
    assert_only(
        "self-loop-ends-differ.bad",
        "self-loop-ends-differ",
        "error",
        "@self(true)",
    );
    assert_clean("self-loop-ends-differ.good");
}
