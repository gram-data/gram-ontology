//! `gram ontology`: strict checks over `*.ontology.gram` files.
//!
//! The base tree is the tree-sitter-gram CST. This extension adds requirements on top of
//! valid gram and never produces a different tree. One run reports everything `gram lint`
//! reports plus the profile checks of RFC-001 §5.6.

mod checks;
mod lint;
mod model;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use gram_diagnostics::{Diagnostic, FileResult, LintResult, Position, Range, Severity};
use tree_sitter::{Node, Parser as TsParser, Tree};

use model::{range_of, text};

#[derive(Parser)]
#[command(
    name = "gram-ontology",
    version,
    about = "Checks for *.ontology.gram files"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check ontology files and report diagnostics
    Check {
        /// Files to check
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Output diagnostics as JSON (the same schema as `gram check --json`)
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Check { files, json } => check(&files, json),
    }
}

fn check(files: &[PathBuf], json: bool) -> ExitCode {
    let mut results = Vec::new();
    for path in files {
        let diagnostics = match std::fs::read_to_string(path) {
            Ok(source) => check_source(path, &source),
            Err(e) => vec![diagnostic(
                "io-error",
                format!("cannot read file: {e}"),
                Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                },
            )],
        };
        results.push(FileResult {
            path: path.display().to_string(),
            diagnostics,
        });
    }

    let has_errors = results
        .iter()
        .flat_map(|f| &f.diagnostics)
        .any(|d| d.severity == Severity::Error);

    if json {
        let report = LintResult {
            schema_version: 1,
            tool: "gram-ontology".into(),
            files: results,
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("diagnostics serialize")
        );
    } else {
        for file in &results {
            for d in &file.diagnostics {
                println!(
                    "{}:{}:{}: {:?} [{}] {}",
                    file.path,
                    d.range.start.line + 1,
                    d.range.start.character + 1,
                    d.severity,
                    d.rule,
                    d.message
                );
                if let Some(help) = &d.help {
                    println!("  help: {help}");
                }
            }
        }
    }

    if has_errors {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

/// All checks for one file's source text.
fn check_source(path: &Path, source: &str) -> Vec<Diagnostic> {
    let mut parser = TsParser::new();
    parser
        .set_language(&tree_sitter_gram::LANGUAGE.into())
        .expect("load gram grammar");
    let lint = lint::lint(source);
    let Some(tree) = parser.parse(source, None) else {
        return lint;
    };

    let mut profile = Vec::new();
    check_opt_in(path, source, &tree, &mut profile);
    check_doc_comments(tree.root_node(), source, &mut profile);
    let model = model::build(&tree, source);
    profile.extend(checks::run(&model));
    lint::merge(lint, profile)
}

/// A file opts in with both the `*.ontology.gram` name and `kind: ontology` in the top-level
/// record. What to do when they disagree is an open question in
/// `design/exploration/EXP-002-ontology-tooling-over-the-cst.md`;
/// for now any mismatch is a warning.
fn check_opt_in(path: &Path, source: &str, tree: &Tree, out: &mut Vec<Diagnostic>) {
    let by_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.ends_with(".ontology.gram"));
    let by_kind = has_kind_ontology(tree.root_node(), source);

    let message = match (by_name, by_kind) {
        (true, true) | (false, false) => return,
        (true, false) => {
            "file is named *.ontology.gram but its top-level record has no `kind: ontology`"
        }
        (false, true) => {
            "top-level record has `kind: ontology` but the file is not named *.ontology.gram"
        }
    };
    out.push(diagnostic("opt-in-mismatch", message.into(), whole_file()));
}

fn has_kind_ontology(root: Node, source: &str) -> bool {
    let Some(record) = root.child_by_field_name("root") else {
        return false;
    };
    let mut cursor = record.walk();
    for property in record
        .children(&mut cursor)
        .filter(|n| n.kind() == "record_property")
    {
        let key = property.child_by_field_name("key").map(|n| text(n, source));
        let value = property
            .child_by_field_name("value")
            .map(|n| text(n, source));
        if key == Some("kind") && value.map(|v| v.trim_matches('"')) == Some("ontology") {
            return true;
        }
    }
    false
}

/// Each `doc_comment` is parsed with gramdoc. Gramdoc errors point into the host file.
fn check_doc_comments(node: Node, source: &str, out: &mut Vec<Diagnostic>) {
    if node.kind() == "doc_comment" {
        let mut parser = TsParser::new();
        parser
            .set_language(&tree_sitter_gramdoc::LANGUAGE.into())
            .expect("load gramdoc grammar");
        let comment = text(node, source);
        if let Some(tree) = parser.parse(comment, None) {
            if tree.root_node().has_error() {
                out.push(diagnostic(
                    "gramdoc-parse-error",
                    "doc comment is not valid gramdoc".into(),
                    range_of(node),
                ));
            }
        }
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        check_doc_comments(child, source, out);
    }
}

fn whole_file() -> Range {
    Range {
        start: Position {
            line: 0,
            character: 0,
        },
        end: Position {
            line: 0,
            character: 0,
        },
    }
}

fn diagnostic(rule: &str, message: String, range: Range) -> Diagnostic {
    checks::diagnostic(rule, message, range, None)
}
