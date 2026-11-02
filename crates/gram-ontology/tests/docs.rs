mod common;
use common::*;

#[test]
fn documented_property_missing() {
    assert_only(
        "documented-property-missing.bad",
        "documented-property-missing",
        "error",
        "nmae",
    );
    // The same mistake in a doc comment is located in the comment.
    assert_only(
        "documented-property-missing.comment.bad",
        "documented-property-missing",
        "error",
        "@property nmae",
    );
    // An existing property, on a definition shared between two graphs.
    assert_clean("documented-property-missing.good");
}

#[test]
fn doc_comment_no_target() {
    assert_only(
        "doc-comment-no-target.bad",
        "doc-comment-no-target",
        "error",
        "/** dangling */",
    );
    // A comment on an annotation-wrapped definition is read through the annotation.
    assert_clean("doc-comment-no-target.good");
}

#[test]
fn duplicate_doc() {
    assert_only("duplicate-doc.bad", "duplicate-doc", "warning", "md`b`");
    // A doc comment's Doc and a written Doc, neither with a `@language`, cannot be told apart.
    assert_only(
        "duplicate-doc.comment.bad",
        "duplicate-doc",
        "warning",
        "Another",
    );
    // Different language, different audience, one comment copied into two graphs.
    assert_clean("duplicate-doc.good");
}
