use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

fn spike(name: &str) -> String {
    format!(
        "{}/../../examples/pattern-rs-spike/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn cmd() -> Command {
    Command::cargo_bin("gram-ontology").unwrap()
}

#[test]
fn spike_movies_file_is_clean() {
    cmd()
        .args(["check", &spike("movies.ontology.gram")])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn file_named_ontology_without_kind_warns_but_succeeds() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("plain.ontology.gram");
    fs::write(&path, "(a)\n[g:Graph | a]\n").unwrap();
    cmd()
        .arg("check")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("opt-in-mismatch"));
}

/// `person.ontology.gram` sets alternative spellings side by side ("Don't do this...", "Do this
/// instead"), so it deliberately breaks the profile. It is a discussion file, not a conforming
/// ontology, and the check rightly reports it. Spec Story 6 scenario 5: the file stays as it is.
#[test]
fn person_spike_is_a_discussion_file_and_is_reported() {
    cmd()
        .args(["check", &spike("person.ontology.gram")])
        .assert()
        .failure()
        .stdout(predicate::str::contains("opt-in-mismatch"))
        .stdout(predicate::str::contains("duplicate-identifier"))
        .stdout(predicate::str::contains("property-value-in-definition"));
}

#[test]
fn kind_ontology_in_plain_gram_file_warns() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("plain.gram");
    fs::write(&path, "{kind: \"ontology\"}\n(a)\n").unwrap();
    cmd()
        .arg("check")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("opt-in-mismatch"));
}

#[test]
fn syntax_error_fails() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("broken.ontology.gram");
    fs::write(&path, "{kind: \"ontology\"}\n(a\n").unwrap();
    cmd()
        .arg("check")
        .arg(&path)
        .assert()
        .failure()
        .code(1)
        .stdout(
            predicate::str::contains("missing-node").or(predicate::str::contains("syntax-error")),
        );
}

#[test]
fn invalid_gramdoc_fails() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("doc.ontology.gram");
    // `@language` needs a code: a bare `@language` is a gramdoc parse error.
    fs::write(&path, "{kind: \"ontology\"}\n/** @language */\n(p)\n").unwrap();
    cmd()
        .arg("check")
        .arg(&path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("gramdoc-parse-error"));
}

#[test]
fn json_output_uses_the_gram_diagnostics_schema() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("plain.ontology.gram");
    fs::write(&path, "(a)\n[g:Graph | a]\n").unwrap();
    let out = cmd()
        .args(["check", "--json", &path.display().to_string()])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let report: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(report["tool"], "gram-ontology");
    assert_eq!(report["schema_version"], 1);
    assert_eq!(
        report["files"][0]["diagnostics"][0]["rule"],
        "opt-in-mismatch"
    );
}
