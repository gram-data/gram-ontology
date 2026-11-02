use gram_diagnostics::Diagnostic;

use super::diagnostic;
use crate::model::{Model, Sep};

pub fn check(model: &Model, out: &mut Vec<Diagnostic>) {
    for def in &model.defs {
        for entry in &def.record {
            if entry.sep == Sep::Single {
                out.push(diagnostic(
                    "property-value-in-definition",
                    format!(
                        "`{}: value` looks like a value where a property declaration belongs",
                        entry.key
                    ),
                    entry.range.clone(),
                    Some(format!("declare the property with `{}:: type`", entry.key)),
                ));
            }
            if entry.key.starts_with('@') {
                out.push(diagnostic(
                    "reserved-property-name",
                    format!(
                        "property names cannot begin with `@`; `{}` is reserved for metadata",
                        entry.key
                    ),
                    entry.range.clone(),
                    None,
                ));
            }
        }
    }
}
