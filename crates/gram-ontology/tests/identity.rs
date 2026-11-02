mod common;
use common::*;

#[test]
fn duplicate_identifier() {
    assert_only(
        "duplicate-identifier.bad",
        "duplicate-identifier",
        "error",
        "(Company)=[OWNS]",
    );
    assert_clean("duplicate-identifier.good");
}

#[test]
fn duplicate_identifier_shared_name() {
    // A node and a relationship that share a name clash; the later one is reported.
    assert_only(
        "duplicate-identifier.shared-name.bad",
        "duplicate-identifier",
        "error",
        "(Person)=[Follows]",
    );
    assert_clean("duplicate-identifier.shared-name.good");
}

#[test]
fn duplicate_identifier_help_fits_the_definition() {
    let help = |src: &str| {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.ontology.gram");
        std::fs::write(&path, src).unwrap();
        let out = assert_cmd::Command::cargo_bin("gram-ontology")
            .unwrap()
            .args(["check", "--json"])
            .arg(&path)
            .output()
            .unwrap();
        let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        report["files"][0]["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["rule"] == "duplicate-identifier")
            .map(|d| d["message"].as_str().unwrap().to_string())
            .expect("a duplicate-identifier diagnostic")
    };
    let head = "{ kind: \"ontology\" }\n";
    let nodes = help(&format!("{head}(Actor)\n(Actor)\n"));
    assert!(nodes.contains("actor:Actor"), "{nodes}");
    assert!(!nodes.contains("ownsCar"), "{nodes}");
    let rels = help(&format!("{head}(A)=[OWNS]=>(B)\n(C)=[OWNS]=>(B)\n"));
    assert!(rels.contains("ownsCar:OWNS"), "{rels}");
    let mixed = help(&format!("{head}(Follows)\n(A)=[Follows]=>(B)\n"));
    assert!(mixed.contains("ownsCar:OWNS"), "{mixed}");
}

#[test]
fn undefined_reference() {
    let name = "undefined-reference.bad";
    let (code, diags) = run_fixture(name);
    assert!(
        diags.iter().all(|d| d.rule == "undefined-reference"),
        "{diags:?}"
    );
    assert!(diags.iter().all(|d| d.severity == "error"));
    // A mistyped graph member, the RFC's `OWNS` once `ownsCar` is the identifier, and an
    // annotated Doc whose target is mistyped.
    let lines: Vec<u32> = diags.iter().map(|d| d.line).collect();
    assert_eq!(
        lines,
        [
            line_of(name, "Prson"),
            line_of(name, "| OWNS]"),
            line_of(name, "| Persn]"),
        ]
    );
    assert_eq!(code, 1);
    // References by explicit identifier, own identifier, Graph and Doc identifiers, and a
    // doc comment's implicit reference all resolve.
    assert_clean("undefined-reference.good");
}

#[test]
fn orphaned_pattern() {
    assert_only(
        "orphaned-pattern.bad",
        "orphaned-pattern",
        "warning",
        "(Movie)",
    );
    assert_clean("orphaned-pattern.good");
    // A relationship and an annotation-wrapped definition are orphaned too.
    let (_, diags) = run_fixture("orphaned-pattern.relationship.bad");
    assert!(
        diags.iter().all(|d| d.rule == "orphaned-pattern"),
        "{diags:?}"
    );
    let lines: Vec<u32> = diags.iter().map(|d| d.line).collect();
    assert_eq!(
        lines,
        [
            line_of("orphaned-pattern.relationship.bad", "(Person)=[ACTED_IN]"),
            line_of("orphaned-pattern.relationship.bad", "(Show"),
        ]
    );
}

#[test]
fn anonymous_doc() {
    assert_only("anonymous-doc.bad", "anonymous-doc", "warning", "[:Doc");
    assert_clean("anonymous-doc.good");
}
