use std::collections::BTreeMap;

use gram_diagnostics::Diagnostic;

use super::{diagnostic, line_of};
use crate::model::Model;

/// A `Doc`'s target, graph and distinguishing fields.
type DocKey = (String, Option<usize>, Vec<(String, String)>);

/// Fields that do not tell two `Doc`s apart.
const NOT_DISTINGUISHING: [&str; 2] = ["@description", "@labels"];

pub fn check(model: &Model, out: &mut Vec<Diagnostic>) {
    for range in &model.dangling_comments {
        out.push(diagnostic(
            "doc-comment-no-target",
            "this doc comment is not followed by a pattern to describe".into(),
            range.clone(),
            None,
        ));
    }

    for doc in &model.docs {
        let Some(target) = &doc.target else { continue };
        let defs: Vec<_> = model
            .defs
            .iter()
            .filter(|d| &d.identifier == target)
            .collect();
        if defs.is_empty() {
            continue;
        }
        for (name, range) in &doc.properties {
            if !defs.iter().any(|d| d.record.iter().any(|e| &e.key == name)) {
                out.push(diagnostic(
                    "documented-property-missing",
                    format!("`{target}` declares no property `{name}`"),
                    range.clone(),
                    None,
                ));
            }
        }
    }

    let mut seen: BTreeMap<DocKey, usize> = BTreeMap::new();
    // Compare in file order, so the later of two is the one reported.
    let mut order: Vec<usize> = (0..model.docs.len()).collect();
    order.sort_by_key(|&i| {
        let r = &model.docs[i].range;
        (r.start.line, r.start.character)
    });
    for i in order {
        let doc = &model.docs[i];
        let Some(target) = &doc.target else { continue };
        let fields: Vec<(String, String)> = doc
            .fields
            .iter()
            .filter(|(k, _)| !NOT_DISTINGUISHING.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        match seen.get(&(target.clone(), doc.graph, fields.clone())) {
            Some(&j) => out.push(diagnostic(
                "duplicate-doc",
                format!(
                    "this Doc for `{target}` cannot be told apart from another: same @language and same other @-prefixed fields"
                ),
                doc.range.clone(),
                Some(format!(
                    "the other is at line {}; differ them with `@language` or another `@` field",
                    line_of(&model.docs[j].range)
                )),
            )),
            None => {
                seen.insert((target.clone(), doc.graph, fields), i);
            }
        }
    }
}
