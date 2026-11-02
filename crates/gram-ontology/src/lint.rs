//! `gram lint` as a library, merged with the profile checks. This command is a superset of
//! `gram lint`: every lint finding is reported, under lint's own name (spec FR-028).

use gram_data::lint::{lint_source, LintOptions};
use gram_diagnostics::Diagnostic;

pub fn lint(source: &str) -> Vec<Diagnostic> {
    lint_source(source, &LintOptions { strict: false })
}

/// Lint findings win. A profile finding is dropped when lint already reports the same check at
/// the same place, or, for `duplicate-identifier`, when lint's `duplicate-element` reports the
/// same line. The result is in file order.
pub fn merge(lint: Vec<Diagnostic>, profile: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let mut out = lint;
    let lint_len = out.len();
    for d in profile {
        let covered = out[..lint_len].iter().any(|l| {
            l.range.start.line == d.range.start.line
                && (l.rule == d.rule
                    || (d.rule == "duplicate-identifier" && l.rule == "duplicate-element"))
        });
        if !covered {
            out.push(d);
        }
    }
    out.sort_by_key(|d| (d.range.start.line, d.range.start.character));
    out
}
