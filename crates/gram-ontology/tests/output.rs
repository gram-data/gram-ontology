mod common;
use assert_cmd::Command;
use common::*;
use std::fs;

fn write(dir: &tempfile::TempDir, name: &str, src: &str) -> std::path::PathBuf {
    let p = dir.path().join(name);
    fs::write(&p, src).unwrap();
    p
}

const SEVERAL: &str = "{ kind: \"ontology\" }\n\
(Person { name:: \"A\", name:: \"B\" })\n\
(Actor::Missing)\n\
(Movie)\n\
[g:Graph | Person, Actor]\n";

fn text_lines(path: &std::path::Path) -> Vec<String> {
    let out = Command::cargo_bin("gram-ontology")
        .unwrap()
        .arg("check")
        .arg(path)
        .output()
        .unwrap();
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

/// The diagnostic lines, without the indented `help:` lines that follow some of them.
fn diagnostic_lines(lines: &[String]) -> Vec<&String> {
    lines.iter().filter(|l| !l.starts_with("  help:")).collect()
}

#[test]
fn text_lines_have_file_line_column_severity_name_and_message_in_file_order() {
    let dir = tempfile::tempdir().unwrap();
    let path = write(&dir, "several.ontology.gram", SEVERAL);
    let all = text_lines(&path);
    let lines = diagnostic_lines(&all);
    assert_eq!(lines.len(), 3, "{all:?}");
    // `orphaned-pattern` carries a help line, printed right after it.
    let orphan = all
        .iter()
        .position(|l| l.contains("[orphaned-pattern]"))
        .unwrap();
    assert!(all[orphan + 1].starts_with("  help: "), "{all:?}");
    let prefix = path.display().to_string();
    assert!(
        lines[0].starts_with(&format!("{prefix}:2:")) && lines[0].contains("Error [duplicate-key]")
    );
    assert!(
        lines[1].starts_with(&format!("{prefix}:3:"))
            && lines[1].contains("Warning [undefined-implied-label]")
    );
    assert!(
        lines[2].starts_with(&format!("{prefix}:4:"))
            && lines[2].contains("Warning [orphaned-pattern]")
    );
}

#[test]
fn json_and_text_agree() {
    let dir = tempfile::tempdir().unwrap();
    let path = write(&dir, "several.ontology.gram", SEVERAL);
    let (_, diags) = run_path(&path);
    let all = text_lines(&path);
    let lines = diagnostic_lines(&all);
    assert_eq!(diags.len(), lines.len());
    for (d, l) in diags.iter().zip(&lines) {
        assert!(l.contains(&format!("[{}]", d.rule)), "{l}");
        assert!(l.contains(&format!(":{}:", d.line)), "{l}");
    }
}

#[test]
fn exit_status_follows_severity() {
    let dir = tempfile::tempdir().unwrap();
    let warnings = write(&dir, "w.ontology.gram", "{ kind: \"ontology\" }\n(Movie)\n");
    let errors = write(
        &dir,
        "e.ontology.gram",
        "{ kind: \"ontology\" }\n(Actor::Person)\n(a)\n[g:Graph | Actor, a]\n(Actor)\n",
    );
    assert_eq!(run_path(&warnings).0, 0);
    assert!(!run_path(&warnings).1.is_empty());
    let (code, diags) = run_path(&errors);
    assert!(diags.iter().any(|d| d.severity == "error"), "{diags:?}");
    assert_eq!(code, 1);
}

#[test]
fn several_files_are_reported_independently_in_order() {
    let dir = tempfile::tempdir().unwrap();
    let a = write(&dir, "a.ontology.gram", "{ kind: \"ontology\" }\n(Movie)\n");
    let b = write(
        &dir,
        "b.ontology.gram",
        "{ kind: \"ontology\" }\n(Person { n:: \"x\", n:: \"y\" })\n[g:Graph | Person]\n",
    );
    let out = Command::cargo_bin("gram-ontology")
        .unwrap()
        .args(["check", "--json"])
        .arg(&a)
        .arg(&b)
        .output()
        .unwrap();
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let files = report["files"].as_array().unwrap();
    assert_eq!(files.len(), 2);
    assert_eq!(files[0]["path"], a.display().to_string());
    assert_eq!(files[0]["diagnostics"][0]["rule"], "orphaned-pattern");
    assert_eq!(files[1]["diagnostics"][0]["rule"], "duplicate-key");
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn no_profile_finding_comes_from_a_broken_region() {
    let dir = tempfile::tempdir().unwrap();
    // The broken pattern would be an orphan and has an undefined implied label if it parsed.
    let path = write(
        &dir,
        "broken.ontology.gram",
        "{ kind: \"ontology\" }\n(Good)\n(Actor::Nope\n[g:Graph | Good]\n",
    );
    let (code, diags) = run_path(&path);
    assert_eq!(code, 1);
    assert!(
        diags
            .iter()
            .any(|d| d.rule == "missing-node" || d.rule == "syntax-error"),
        "{diags:?}"
    );
    assert!(
        diags
            .iter()
            .all(|d| d.rule == "missing-node" || d.rule == "syntax-error"),
        "{diags:?}"
    );
}
