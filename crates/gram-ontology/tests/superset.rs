//! The command is a superset of `gram lint` (spec FR-028, SC-007).
mod common;
use common::*;

use gram_data::lint::{lint_source, LintOptions};
use std::path::PathBuf;

fn all_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> =
        std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "gram"))
            .collect();
    for name in ["movies", "person"] {
        files.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../examples/pattern-rs-spike/{name}.ontology.gram"
        )));
    }
    files
}

#[test]
fn every_lint_diagnostic_is_reported() {
    let mut seen_lint = 0;
    for path in all_files() {
        let source = std::fs::read_to_string(&path).unwrap();
        let ours = run_path(&path).1;
        for l in lint_source(&source, &LintOptions { strict: false }) {
            seen_lint += 1;
            let sev = format!("{:?}", l.severity).to_lowercase();
            let line = l.range.start.line + 1;
            assert!(
                ours.iter().any(|d| d.rule == l.rule
                    && d.severity == sev
                    && d.line == line
                    && d.col == l.range.start.character
                    && d.end_line == l.range.end.line
                    && d.end_col == l.range.end.character),
                "{}: lint's {} at line {line} (range preserved) is missing from {ours:?}",
                path.display(),
                l.rule
            );
        }
    }
    assert!(
        seen_lint > 0,
        "the fixtures include at least one lint finding"
    );
}
