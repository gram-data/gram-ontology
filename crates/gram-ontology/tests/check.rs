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
    cmd()
        .args(["check", &spike("person.ontology.gram")])
        .assert()
        .success()
        .stdout(predicate::str::contains("opt-in-mismatch"));
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
        .stdout(predicate::str::contains("parse-error"));
}

#[test]
fn invalid_gramdoc_fails() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("doc.ontology.gram");
    // `@self` is a bare marker: text after it is a gramdoc parse error.
    fs::write(
        &path,
        "{kind: \"ontology\"}\n/** @self nope */\n[:SAME_AS | (p), (p)]\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .arg(&path)
        .assert()
        .failure()
        .stdout(predicate::str::contains("gramdoc-parse-error"));
}

#[test]
fn json_output_uses_the_gram_diagnostics_schema() {
    let out = cmd()
        .args(["check", "--json", &spike("person.ontology.gram")])
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
