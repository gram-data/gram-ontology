use std::collections::BTreeMap;

use gram_diagnostics::Diagnostic;

use super::{diagnostic, line_of};
use crate::model::Model;

pub fn check(model: &Model, out: &mut Vec<Diagnostic>) {
    let mut first: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, def) in model.defs.iter().enumerate().filter(|(_, d)| d.top_level) {
        match first.get(def.identifier.as_str()) {
            Some(&j) => out.push(diagnostic(
                "duplicate-identifier",
                format!(
                    "`{}` is already the identifier of another definition; give one of them an explicit identifier, as in `ownsCar:OWNS`",
                    def.identifier
                ),
                def.range.clone(),
                Some(format!(
                    "first defined at line {}",
                    line_of(&model.defs[j].range)
                )),
            )),
            None => {
                first.insert(&def.identifier, i);
            }
        }
        if !model.referenced.contains(&def.identifier) {
            out.push(diagnostic(
                "orphaned-pattern",
                format!("`{}` is not listed by any Graph pattern", def.identifier),
                def.range.clone(),
                Some("add it to a Graph pattern's elements".into()),
            ));
        }
    }
    for doc in model.docs.iter().filter(|d| d.top_level && !d.has_identity) {
        out.push(diagnostic(
            "anonymous-doc",
            "this top-level Doc has no identifier, so no graph can refer to it".into(),
            doc.range.clone(),
            Some("give it an identifier, or write it inside a Graph pattern".into()),
        ));
    }
}
