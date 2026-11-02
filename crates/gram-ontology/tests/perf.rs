//! A 10,000-line file is checked in under 2 seconds (spec SC-005).
mod common;
use common::*;
use std::fmt::Write;
use std::time::Instant;

#[test]
fn ten_thousand_lines_under_two_seconds() {
    let mut src = String::from("{ kind: \"ontology\" }\n");
    let blocks = 1900;
    for i in 0..blocks {
        write!(
            src,
            "/**\n * Thing {i}.\n *\n * @property p The property.\n */\n(T{i} {{ p:: \"STRING\" }})\n"
        )
        .unwrap();
    }
    src.push_str("[g:Graph |\n");
    for i in 0..blocks {
        writeln!(src, "  T{i}{}", if i + 1 < blocks { "," } else { "" }).unwrap();
    }
    src.push_str("]\n");
    assert!(src.lines().count() >= 10_000, "{}", src.lines().count());
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("big.ontology.gram");
    std::fs::write(&path, &src).unwrap();
    let start = Instant::now();
    let (code, diags) = run_path(&path);
    let elapsed = start.elapsed();
    assert_eq!((code, diags.len()), (0, 0), "{diags:?}");
    assert!(elapsed.as_secs_f64() < 2.0, "took {elapsed:?}");
}
