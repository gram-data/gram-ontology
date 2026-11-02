use gram_diagnostics::Diagnostic;

use super::diagnostic;
use crate::model::{Kind, Model};

pub fn check(model: &Model, out: &mut Vec<Diagnostic>) {
    for def in model.defs.iter().filter(|d| d.kind == Kind::Relationship) {
        for ann in &def.annotations {
            match ann.name.as_str() {
                "cardinality" => {
                    if let Some(n) = ann.array_len.filter(|n| *n != def.element_count) {
                        out.push(diagnostic(
                            "cardinality-length-mismatch",
                            format!(
                                "`{}` has {} elements but the cardinality has {n} ranges; give one range per element",
                                def.identifier, def.element_count
                            ),
                            ann.range.clone(),
                            None,
                        ));
                    }
                }
                // A self-loop is written `@self(true)` for now (EXP-003, RFC §7.2).
                "self" if ann.value == "true" => {
                    let same = def.ends.first().is_some_and(|first| {
                        first.is_some() && def.ends.iter().all(|e| e == first)
                    });
                    if !same {
                        out.push(diagnostic(
                            "self-loop-ends-differ",
                            format!(
                                "`{}` is declared a self-loop but its ends are not the same node",
                                def.identifier
                            ),
                            ann.range.clone(),
                            None,
                        ));
                    }
                }
                _ => {}
            }
        }
    }
}
