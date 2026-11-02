//! The profile checks of RFC-001 §5.6. Each check is a function over the file model. Severity
//! is held in one table, so a check never picks its own (spec A1, research R7).

mod docs;
mod identity;
mod labels;
mod records;
mod structure;

use gram_diagnostics::{Diagnostic, Range, Severity};

use crate::model::Model;

/// Check name and severity. Severities the RFC states are as stated; the rest were decided in
/// the 2026-11-02 clarification session of `specs/001-ontology-check-rfc-001/spec.md`.
/// The checks inherited from `gram lint` keep lint's own severity and are not listed here.
pub const SEVERITIES: &[(&str, Severity)] = &[
    ("opt-in-mismatch", Severity::Warning),
    ("gramdoc-parse-error", Severity::Error),
    ("io-error", Severity::Error),
    ("orphaned-pattern", Severity::Warning),
    ("anonymous-doc", Severity::Warning),
    ("misplaced-label-separator", Severity::Warning),
    ("undefined-implied-label", Severity::Warning),
    ("ambiguous-implied-label", Severity::Warning),
    ("property-value-in-definition", Severity::Warning),
    ("duplicate-doc", Severity::Warning),
    ("duplicate-identifier", Severity::Error),
    ("documented-property-missing", Severity::Error),
    ("doc-comment-no-target", Severity::Error),
    ("cardinality-length-mismatch", Severity::Error),
    ("self-loop-ends-differ", Severity::Error),
    ("implied-label-cycle", Severity::Error),
    ("optional-identifying-label", Severity::Error),
    ("relationship-double-colon", Severity::Error),
    ("reserved-property-name", Severity::Error),
];

pub fn severity(rule: &str) -> Severity {
    SEVERITIES
        .iter()
        .find(|(name, _)| *name == rule)
        .map(|(_, s)| s.clone())
        .unwrap_or_else(|| panic!("check `{rule}` has no severity in the table"))
}

pub fn diagnostic(rule: &str, message: String, range: Range, help: Option<String>) -> Diagnostic {
    Diagnostic {
        severity: severity(rule),
        rule: rule.into(),
        message,
        range,
        code: None,
        help,
    }
}

/// 1-based line of a range's start, for messages that point at another location.
pub fn line_of(range: &Range) -> u32 {
    range.start.line + 1
}

pub fn run(model: &Model) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    identity::check(model, &mut out);
    labels::check(model, &mut out);
    records::check(model, &mut out);
    docs::check(model, &mut out);
    structure::check(model, &mut out);
    out
}
