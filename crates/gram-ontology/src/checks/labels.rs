use std::collections::BTreeMap;

use gram_diagnostics::Diagnostic;

use super::diagnostic;
use crate::model::{Kind, Model};

pub fn check(model: &Model, out: &mut Vec<Diagnostic>) {
    let mut by_label: BTreeMap<&str, usize> = BTreeMap::new();
    for def in model.defs.iter().filter(|d| d.kind == Kind::Node) {
        if let Some(l) = &def.identifying {
            *by_label.entry(l).or_default() += 1;
        }
    }

    for def in &model.defs {
        if def.kind == Kind::Node && def.identifying_optional {
            out.push(diagnostic(
                "optional-identifying-label",
                format!(
                    "the identifying label `{}` cannot be optional; remove the trailing `?`",
                    def.identifying.as_deref().unwrap_or_default()
                ),
                def.identifier_range.clone(),
                None,
            ));
        }
        for range in &def.misplaced_separators {
            out.push(diagnostic(
                "misplaced-label-separator",
                "`::` is only read directly after a node's identifier; use `:` here".into(),
                range.clone(),
                None,
            ));
        }
        for range in &def.relationship_separators {
            out.push(diagnostic(
                "relationship-double-colon",
                "a relationship has one type and no implied labels, so `::` is not used; write `:`"
                    .into(),
                range.clone(),
                None,
            ));
        }
        for label in &def.implied {
            match by_label.get(label.name.as_str()).copied().unwrap_or(0) {
                0 => out.push(diagnostic(
                    "undefined-implied-label",
                    format!("no node definition has the identifying label `{}`", label.name),
                    label.range.clone(),
                    None,
                )),
                1 => {}
                n => out.push(diagnostic(
                    "ambiguous-implied-label",
                    format!(
                        "`{}` is the identifying label of {n} definitions, so this reference is ambiguous",
                        label.name
                    ),
                    label.range.clone(),
                    None,
                )),
            }
        }
    }
    cycles(model, out);
}

/// Implied labels form edges from a node's identifying label to each implied label.
/// One diagnostic per cycle: a strongly connected component of more than one label, or a
/// label that implies itself.
fn cycles(model: &Model, out: &mut Vec<Diagnostic>) {
    let mut names: Vec<&str> = Vec::new();
    let mut index: BTreeMap<&str, usize> = BTreeMap::new();
    for def in model.defs.iter().filter(|d| d.kind == Kind::Node) {
        for n in def
            .identifying
            .as_deref()
            .into_iter()
            .chain(def.implied.iter().map(|l| l.name.as_str()))
        {
            index.entry(n).or_insert_with(|| {
                names.push(n);
                names.len() - 1
            });
        }
    }
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); names.len()];
    let mut first_def: Vec<Option<usize>> = vec![None; names.len()];
    for (di, def) in model
        .defs
        .iter()
        .enumerate()
        .filter(|(_, d)| d.kind == Kind::Node)
    {
        let Some(from) = def.identifying.as_deref().map(|n| index[n]) else {
            continue;
        };
        first_def[from].get_or_insert(di);
        for l in &def.implied {
            edges[from].push(index[l.name.as_str()]);
        }
    }

    // Tarjan's strongly connected components.
    struct State {
        counter: usize,
        idx: Vec<Option<usize>>,
        low: Vec<usize>,
        on: Vec<bool>,
        stack: Vec<usize>,
        comps: Vec<Vec<usize>>,
    }
    fn visit(v: usize, edges: &[Vec<usize>], s: &mut State) {
        s.idx[v] = Some(s.counter);
        s.low[v] = s.counter;
        s.counter += 1;
        s.stack.push(v);
        s.on[v] = true;
        for &w in &edges[v] {
            match s.idx[w] {
                None => {
                    visit(w, edges, s);
                    s.low[v] = s.low[v].min(s.low[w]);
                }
                Some(i) if s.on[w] => s.low[v] = s.low[v].min(i),
                _ => {}
            }
        }
        if Some(s.low[v]) == s.idx[v] {
            let mut comp = Vec::new();
            while let Some(w) = s.stack.pop() {
                s.on[w] = false;
                comp.push(w);
                if w == v {
                    break;
                }
            }
            s.comps.push(comp);
        }
    }
    let mut s = State {
        counter: 0,
        idx: vec![None; names.len()],
        low: vec![0; names.len()],
        on: vec![false; names.len()],
        stack: Vec::new(),
        comps: Vec::new(),
    };
    for v in 0..names.len() {
        if s.idx[v].is_none() {
            visit(v, &edges, &mut s);
        }
    }
    for mut comp in s.comps {
        let cyclic = comp.len() > 1 || edges[comp[0]].contains(&comp[0]);
        if !cyclic {
            continue;
        }
        comp.sort_by_key(|&v| first_def[v].unwrap_or(usize::MAX));
        let at = comp
            .iter()
            .find_map(|&v| first_def[v])
            .map(|d| model.defs[d].range.clone());
        let Some(range) = at else { continue };
        let mut labels: Vec<&str> = comp.iter().map(|&v| names[v]).collect();
        labels.sort();
        out.push(diagnostic(
            "implied-label-cycle",
            format!("implied labels form a cycle: {}", labels.join(", ")),
            range,
            None,
        ));
    }
}
