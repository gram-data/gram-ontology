#![allow(dead_code)]

use assert_cmd::Command;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Diag {
    pub rule: String,
    pub severity: String,
    /// 1-based line.
    pub line: u32,
    /// 0-based start column, and the 0-based end line and column.
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(format!("{name}.ontology.gram"))
}

pub fn run_path(path: &std::path::Path) -> (i32, Vec<Diag>) {
    let out = Command::cargo_bin("gram-ontology")
        .unwrap()
        .args(["check", "--json"])
        .arg(path)
        .output()
        .unwrap();
    let report: Value = serde_json::from_slice(&out.stdout).expect("json report");
    let diags = report["files"][0]["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| Diag {
            rule: d["rule"].as_str().unwrap().into(),
            severity: d["severity"].as_str().unwrap().into(),
            line: d["range"]["start"]["line"].as_u64().unwrap() as u32 + 1,
            col: d["range"]["start"]["character"].as_u64().unwrap() as u32,
            end_line: d["range"]["end"]["line"].as_u64().unwrap() as u32,
            end_col: d["range"]["end"]["character"].as_u64().unwrap() as u32,
        })
        .collect();
    (out.status.code().unwrap(), diags)
}

pub fn run_fixture(name: &str) -> (i32, Vec<Diag>) {
    run_path(&fixture_path(name))
}

/// 1-based line of the first line in the fixture containing `needle`.
pub fn line_of(name: &str, needle: &str) -> u32 {
    let src = std::fs::read_to_string(fixture_path(name)).unwrap();
    src.lines()
        .position(|l| l.contains(needle))
        .unwrap_or_else(|| panic!("`{needle}` not in fixture {name}")) as u32
        + 1
}

/// The fixture produces no diagnostics at all.
pub fn assert_clean(name: &str) {
    let (code, diags) = run_fixture(name);
    assert!(diags.is_empty(), "{name}: expected clean, got {diags:?}");
    assert_eq!(code, 0, "{name}");
}

/// The fixture produces only `rule`, with `severity`, and one at the line holding `needle`.
/// The exit status follows the severity.
pub fn assert_only(name: &str, rule: &str, severity: &str, needle: &str) {
    let (code, diags) = run_fixture(name);
    assert!(
        diags.iter().all(|d| d.rule == rule),
        "{name}: expected only `{rule}`, got {diags:?}"
    );
    let line = line_of(name, needle);
    assert!(
        diags
            .iter()
            .any(|d| d.line == line && d.severity == severity),
        "{name}: expected `{rule}` ({severity}) at line {line}, got {diags:?}"
    );
    assert_eq!(code, if severity == "error" { 1 } else { 0 }, "{name}");
}

/// How many diagnostics of `rule` the fixture produces.
pub fn count(name: &str, rule: &str) -> usize {
    run_fixture(name)
        .1
        .iter()
        .filter(|d| d.rule == rule)
        .count()
}
