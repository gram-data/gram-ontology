//! `gram lint` as a library, merged with the profile checks. This command is a superset of
//! `gram lint`: every lint finding is reported, under lint's own name (spec FR-028).

use gram_data::lint::{lint_source, LintOptions};
use gram_diagnostics::{Diagnostic, Range};

/// Lint's findings, each reported once: a finding repeated with the same check, range and
/// message (a syntax error can be raised twice at one position) is dropped.
pub fn lint(source: &str) -> Vec<Diagnostic> {
    dedupe(lint_source(source, &LintOptions { strict: false }))
}

fn dedupe(findings: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let mut out: Vec<Diagnostic> = Vec::new();
    for d in findings {
        if !out
            .iter()
            .any(|o| o.rule == d.rule && o.message == d.message && same_range(&o.range, &d.range))
        {
            out.push(d);
        }
    }
    out
}

fn same_range(a: &Range, b: &Range) -> bool {
    within(a, b) && within(b, a)
}

/// `inner` lies within `outer` (equal ranges count).
fn within(inner: &Range, outer: &Range) -> bool {
    let key = |p: &gram_diagnostics::Position| (p.line, p.character);
    key(&outer.start) <= key(&inner.start) && key(&inner.end) <= key(&outer.end)
}

/// Lint findings win. A profile finding is dropped only when lint already reports the same
/// check over the same range, or, for `duplicate-identifier`, when lint's `duplicate-element`
/// lies within the definition it points at. The result is in file order.
pub fn merge(lint: Vec<Diagnostic>, profile: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let mut out = lint;
    let lint_len = out.len();
    for d in profile {
        let covered = out[..lint_len].iter().any(|l| {
            (l.rule == d.rule && same_range(&l.range, &d.range))
                || (d.rule == "duplicate-identifier"
                    && l.rule == "duplicate-element"
                    && within(&l.range, &d.range))
        });
        if !covered {
            out.push(d);
        }
    }
    out.sort_by_key(|d| (d.range.start.line, d.range.start.character));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use gram_diagnostics::{Position, Severity};

    fn diag(rule: &str, start: (u32, u32), end: (u32, u32)) -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            rule: rule.into(),
            message: String::new(),
            range: Range {
                start: Position {
                    line: start.0,
                    character: start.1,
                },
                end: Position {
                    line: end.0,
                    character: end.1,
                },
            },
            code: None,
            help: None,
        }
    }

    #[test]
    fn a_finding_repeated_at_one_position_is_kept_once() {
        let a = diag("syntax-error", (4, 27), (4, 28));
        let b = diag("syntax-error", (4, 40), (4, 41));
        assert_eq!(dedupe(vec![a.clone(), a, b]).len(), 2);
    }

    #[test]
    fn a_lint_finding_elsewhere_on_the_line_does_not_hide_a_profile_finding() {
        let lint = vec![diag("duplicate-element", (3, 30), (3, 35))];
        let profile = vec![diag("duplicate-identifier", (3, 0), (3, 20))];
        assert_eq!(merge(lint, profile).len(), 2);
    }

    #[test]
    fn a_lint_finding_within_the_definition_covers_the_profile_finding() {
        let lint = vec![diag("duplicate-element", (3, 1), (3, 8))];
        let profile = vec![diag("duplicate-identifier", (3, 0), (3, 20))];
        let merged = merge(lint, profile);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].rule, "duplicate-element");
    }
}
