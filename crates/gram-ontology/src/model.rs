//! The file model the profile checks read: definitions, graphs and `Doc`s, built once from the
//! tree-sitter-gram CST (and the gramdoc tree of each doc comment).
//!
//! Regions of the file that failed to parse are skipped, so no check derives a finding from
//! them (spec FR-027).

use std::collections::{BTreeMap, BTreeSet};

use gram_diagnostics::{Position, Range};
use tree_sitter::{Node, Parser as TsParser, Tree};

pub fn text<'a>(node: Node, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

pub fn range_of(node: Node) -> Range {
    let (s, e) = (node.start_position(), node.end_position());
    Range {
        start: Position {
            line: s.row as u32,
            character: s.column as u32,
        },
        end: Position {
            line: e.row as u32,
            character: e.column as u32,
        },
    }
}

/// The first named node in a field. A field can also cover anonymous tokens, such as the `|`
/// before a subject pattern's elements, which `child_by_field_name` would return first.
fn named_field<'t>(node: Node<'t>, field: &str) -> Option<Node<'t>> {
    let mut cursor = node.walk();
    let found = node
        .children_by_field_name(field, &mut cursor)
        .find(|n| n.is_named());
    found
}

/// A name as written: a symbol, or a quoted name without its backticks or quotes.
fn name_of(node: Node, source: &str) -> String {
    text(node, source)
        .trim_matches(|c| c == '`' || c == '"')
        .to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sep {
    Single,
    Double,
}

#[derive(Debug, Clone)]
pub struct Label {
    /// The name without a trailing `?`.
    pub name: String,
    pub optional: bool,
    pub sep: Sep,
    pub range: Range,
    pub sep_range: Range,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub key: String,
    pub sep: Sep,
    pub range: Range,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub name: String,
    pub value: String,
    /// The number of items when the value is an array.
    pub array_len: Option<usize>,
    pub range: Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Node,
    Relationship,
}

/// A comment read through gramdoc into the pieces the checks need.
#[derive(Debug, Clone, Default)]
pub struct CommentInfo {
    pub fields: BTreeMap<String, String>,
    pub properties: Vec<(String, Range)>,
}

#[derive(Debug, Clone)]
pub struct Def {
    pub kind: Kind,
    pub identifier: String,
    pub identifier_range: Range,
    /// A node's identifying label, without a trailing `?`.
    pub identifying: Option<String>,
    pub identifying_optional: bool,
    pub implied: Vec<Label>,
    /// `::` separators that do not directly follow a node's identifier.
    pub misplaced_separators: Vec<Range>,
    /// `::` separators before a relationship's type.
    pub relationship_separators: Vec<Range>,
    pub record: Vec<Entry>,
    pub annotations: Vec<Annotation>,
    /// A relationship's ends, by identity (`None` when an end has none).
    pub ends: Vec<Option<String>>,
    /// The number of elements in a relationship's pattern.
    pub element_count: usize,
    pub range: Range,
    pub top_level: bool,
    pub graphs: Vec<usize>,
    pub comments: Vec<CommentInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub members: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub target: Option<String>,
    pub has_identity: bool,
    pub top_level: bool,
    pub graph: Option<usize>,
    pub fields: BTreeMap<String, String>,
    pub properties: Vec<(String, Range)>,
    pub range: Range,
}

#[derive(Debug, Default)]
pub struct Model {
    pub defs: Vec<Def>,
    pub graphs: Vec<Graph>,
    pub docs: Vec<Doc>,
    pub dangling_comments: Vec<Range>,
    pub referenced: BTreeSet<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ctx {
    Top,
    Graph(usize),
}

pub fn build(tree: &Tree, source: &str) -> Model {
    let mut model = Model::default();
    let root = tree.root_node();
    let mut b = Builder {
        source,
        model: &mut model,
    };
    let mut cursor = root.walk();
    let children: Vec<Node> = root.named_children(&mut cursor).collect();
    b.container(&children, Ctx::Top);

    // Membership: a top-level definition belongs to every graph that lists its identifier.
    let graphs = model.graphs.clone();
    for def in &mut model.defs {
        for (i, g) in graphs.iter().enumerate() {
            if def.top_level && g.members.contains(&def.identifier) && !def.graphs.contains(&i) {
                def.graphs.push(i);
            }
        }
    }
    for g in &graphs {
        model.referenced.extend(g.members.iter().cloned());
    }
    // A comment on a definition reads into a Doc inline in each graph that holds it.
    let mut comment_docs = Vec::new();
    for def in &model.defs {
        for c in &def.comments {
            let graph_ids: Vec<Option<usize>> = if def.graphs.is_empty() {
                vec![None]
            } else {
                def.graphs.iter().map(|g| Some(*g)).collect()
            };
            for g in graph_ids {
                comment_docs.push(Doc {
                    target: Some(def.identifier.clone()),
                    has_identity: true,
                    top_level: false,
                    graph: g,
                    fields: c.fields.clone(),
                    properties: c.properties.clone(),
                    range: def.range.clone(),
                });
            }
        }
    }
    model.docs.extend(comment_docs);
    model
}

struct Builder<'a, 'm> {
    source: &'a str,
    model: &'m mut Model,
}

impl Builder<'_, '_> {
    /// Patterns in one container; a doc comment documents the next pattern in the same container.
    fn container(&mut self, children: &[Node], ctx: Ctx) {
        let mut pending: Option<Node> = None;
        for &child in children {
            match child.kind() {
                "doc_comment" => {
                    if let Some(prev) = pending.replace(child) {
                        self.model.dangling_comments.push(range_of(prev));
                    }
                }
                "record" | "ERROR" => {
                    if let Some(prev) = pending.take() {
                        self.model.dangling_comments.push(range_of(prev));
                    }
                }
                _ => {
                    let comment = pending.take();
                    if child.has_error() {
                        continue;
                    }
                    self.pattern(child, comment, ctx);
                }
            }
        }
        if let Some(prev) = pending {
            self.model.dangling_comments.push(range_of(prev));
        }
    }

    fn pattern(&mut self, node: Node, comment: Option<Node>, ctx: Ctx) {
        let mut annotations = Vec::new();
        let mut inner = node;
        while inner.kind() == "annotated_pattern" {
            if let Some(a) = inner.child_by_field_name("annotations") {
                let mut c = a.walk();
                for ann in a.named_children(&mut c) {
                    if ann.kind() == "property_annotation" {
                        annotations.push(self.annotation(ann));
                    }
                }
            }
            match named_field(inner, "elements") {
                Some(e) => inner = e,
                None => return,
            }
        }
        let info = comment.and_then(|c| desugar_comment(c, self.source));
        match inner.kind() {
            "node_pattern" => {
                if let Some(def) = self.node_def(inner, annotations, ctx) {
                    self.push_def(def, info);
                } else if let Some(c) = comment {
                    self.model.dangling_comments.push(range_of(c));
                }
            }
            "relationship_pattern" => {
                if let Some(def) = self.relationship_def(inner, annotations, ctx) {
                    self.push_def(def, info);
                } else if let Some(c) = comment {
                    self.model.dangling_comments.push(range_of(c));
                }
            }
            "subject_pattern" => self.subject(inner, annotations, comment, info, ctx),
            "pattern_reference" => {
                let name = name_of(inner, self.source);
                if let Ctx::Graph(g) = ctx {
                    self.model.graphs[g].members.insert(name.clone());
                    if let Some(info) = info {
                        self.model.docs.push(Doc {
                            target: Some(name),
                            has_identity: true,
                            top_level: false,
                            graph: Some(g),
                            fields: info.fields,
                            properties: info.properties,
                            range: range_of(inner),
                        });
                    }
                } else if let Some(c) = comment {
                    self.model.dangling_comments.push(range_of(c));
                }
            }
            _ => {
                if let Some(c) = comment {
                    self.model.dangling_comments.push(range_of(c));
                }
            }
        }
    }

    fn push_def(&mut self, mut def: Def, info: Option<CommentInfo>) {
        if let Some(info) = info {
            def.comments.push(info);
        }
        self.model.defs.push(def);
    }

    fn annotation(&self, node: Node) -> Annotation {
        let key = node
            .child_by_field_name("key")
            .map(|k| text(k, self.source).to_string())
            .unwrap_or_default();
        let value = node.child_by_field_name("value");
        Annotation {
            name: key,
            value: value
                .map(|v| text(v, self.source).trim().to_string())
                .unwrap_or_default(),
            array_len: value
                .filter(|v| v.kind() == "array")
                .map(|v| v.named_child_count()),
            range: range_of(node),
        }
    }

    fn labels(&self, node: Node) -> Vec<Label> {
        let Some(labels) = node.child_by_field_name("labels") else {
            return Vec::new();
        };
        let mut out = Vec::new();
        let mut sep = Sep::Single;
        let mut sep_range = range_of(labels);
        let mut cursor = labels.walk();
        for child in labels.children(&mut cursor) {
            match child.kind() {
                ":" => {
                    sep = Sep::Single;
                    sep_range = range_of(child);
                }
                "::" => {
                    sep = Sep::Double;
                    sep_range = range_of(child);
                }
                "symbol" | "quoted_name" => {
                    let raw = name_of(child, self.source);
                    let optional = raw.ends_with('?');
                    out.push(Label {
                        name: raw.trim_end_matches('?').to_string(),
                        optional,
                        sep,
                        range: range_of(child),
                        sep_range: sep_range.clone(),
                    });
                }
                _ => {}
            }
        }
        out
    }

    fn record(&self, node: Node) -> Vec<Entry> {
        let Some(record) = node.child_by_field_name("record") else {
            return Vec::new();
        };
        let mut out = Vec::new();
        let mut cursor = record.walk();
        for prop in record.named_children(&mut cursor) {
            if prop.kind() != "record_property" {
                continue;
            }
            let Some(key) = prop.child_by_field_name("key") else {
                continue;
            };
            let mut sep = Sep::Single;
            let mut c = prop.walk();
            for t in prop.children(&mut c) {
                if t.kind() == "::" {
                    sep = Sep::Double;
                }
            }
            out.push(Entry {
                key: name_of(key, self.source),
                sep,
                range: range_of(prop),
            });
        }
        out
    }

    fn identity(&self, node: Node) -> Option<(String, Range)> {
        node.child_by_field_name("identifier")
            .map(|n| (name_of(n, self.source), range_of(n)))
    }

    fn node_def(&mut self, node: Node, annotations: Vec<Annotation>, ctx: Ctx) -> Option<Def> {
        let subject = node;
        let ident = self.identity(subject);
        let labels = self.labels(subject);
        let record = self.record(subject);
        let (identifier, identifier_range) = match (&ident, labels.first()) {
            (Some((n, r)), _) => (n.clone(), r.clone()),
            (None, Some(l)) => (l.name.clone(), l.range.clone()),
            (None, None) => return None,
        };
        let shorthand = ident.is_some() && labels.first().is_some_and(|l| l.sep == Sep::Double);
        let (identifying, optional, implied): (String, bool, Vec<Label>) =
            match (&ident, labels.first()) {
                (Some((n, _)), _) if labels.is_empty() || shorthand => (
                    n.trim_end_matches('?').to_string(),
                    n.ends_with('?'),
                    labels.clone(),
                ),
                (_, Some(first)) => (first.name.clone(), first.optional, labels[1..].to_vec()),
                _ => return None,
            };
        let misplaced = labels
            .iter()
            .enumerate()
            .filter(|(i, l)| l.sep == Sep::Double && !(*i == 0 && ident.is_some()))
            .map(|(_, l)| l.sep_range.clone())
            .collect();
        Some(Def {
            kind: Kind::Node,
            identifier,
            identifier_range,
            identifying: Some(identifying),
            identifying_optional: optional,
            implied,
            misplaced_separators: misplaced,
            relationship_separators: Vec::new(),
            record,
            annotations,
            ends: Vec::new(),
            element_count: 0,
            range: range_of(node),
            top_level: ctx == Ctx::Top,
            graphs: match ctx {
                Ctx::Graph(g) => vec![g],
                Ctx::Top => Vec::new(),
            },
            comments: Vec::new(),
        })
    }

    fn end_identity(&self, node: Node) -> Option<String> {
        let ident = self.identity(node).map(|(n, _)| n);
        ident.or_else(|| self.labels(node).first().map(|l| l.name.clone()))
    }

    fn relationship_def(
        &mut self,
        node: Node,
        annotations: Vec<Annotation>,
        ctx: Ctx,
    ) -> Option<Def> {
        let kind = node.child_by_field_name("kind")?;
        let ident = self.identity(kind);
        let labels = self.labels(kind);
        let (identifier, identifier_range) = match (&ident, labels.first()) {
            (Some((n, r)), _) => (n.clone(), r.clone()),
            (None, Some(l)) => (l.name.clone(), l.range.clone()),
            (None, None) => return None,
        };
        let mut ends = Vec::new();
        let mut cur = Some(node);
        while let Some(rel) = cur {
            if rel.kind() == "relationship_pattern" {
                if ends.is_empty() {
                    ends.push(
                        rel.child_by_field_name("left")
                            .and_then(|n| self.end_identity(n)),
                    );
                }
                cur = rel.child_by_field_name("right");
                if let Some(r) = cur {
                    if r.kind() == "node_pattern" {
                        ends.push(self.end_identity(r));
                        cur = None;
                    }
                }
            } else {
                cur = None;
            }
        }
        Some(Def {
            kind: Kind::Relationship,
            identifier,
            identifier_range,
            identifying: None,
            identifying_optional: false,
            implied: Vec::new(),
            misplaced_separators: Vec::new(),
            relationship_separators: labels
                .iter()
                .filter(|l| l.sep == Sep::Double)
                .map(|l| l.sep_range.clone())
                .collect(),
            record: self.record(kind),
            annotations,
            element_count: ends.len(),
            ends,
            range: range_of(node),
            top_level: ctx == Ctx::Top,
            graphs: match ctx {
                Ctx::Graph(g) => vec![g],
                Ctx::Top => Vec::new(),
            },
            comments: Vec::new(),
        })
    }

    fn subject(
        &mut self,
        node: Node,
        annotations: Vec<Annotation>,
        comment: Option<Node>,
        info: Option<CommentInfo>,
        ctx: Ctx,
    ) {
        let ident = self.identity(node);
        let labels = self.labels(node);
        let first = labels.first().map(|l| l.name.as_str());
        let elements = named_field(node, "elements");
        // A doc comment before the first element is a child of the subject pattern itself.
        let mut element_nodes: Vec<Node> = elements
            .map(|e| {
                let mut c = e.walk();
                e.named_children(&mut c).collect()
            })
            .unwrap_or_default();
        let mut c = node.walk();
        element_nodes.extend(
            node.named_children(&mut c)
                .filter(|n| n.kind() == "doc_comment"),
        );
        element_nodes.sort_by_key(|n| n.start_byte());

        match first {
            Some("Graph") => {
                let idx = self.model.graphs.len();
                self.model.graphs.push(Graph::default());
                self.container(&element_nodes, Ctx::Graph(idx));
            }
            Some("Doc") => {
                let mut fields = BTreeMap::new();
                let mut properties = Vec::new();
                if let Some(record) = node.child_by_field_name("record") {
                    let mut c = record.walk();
                    for prop in record.named_children(&mut c) {
                        let Some(key) = prop.child_by_field_name("key") else {
                            continue;
                        };
                        let name = name_of(key, self.source);
                        if name.starts_with('@') {
                            let value = prop
                                .child_by_field_name("value")
                                .map(|v| text(v, self.source).trim().to_string())
                                .unwrap_or_default();
                            fields.insert(name, value);
                        } else {
                            properties.push((name, range_of(key)));
                        }
                    }
                }
                if let Some(info) = info {
                    for (k, v) in info.fields {
                        fields.entry(k).or_insert(v);
                    }
                    properties.extend(info.properties);
                }
                let target = element_nodes
                    .iter()
                    .find(|n| n.kind() == "pattern_reference")
                    .map(|n| name_of(*n, self.source));
                self.model.docs.push(Doc {
                    target,
                    has_identity: ident.is_some(),
                    top_level: ctx == Ctx::Top,
                    graph: match ctx {
                        Ctx::Graph(g) => Some(g),
                        Ctx::Top => None,
                    },
                    fields,
                    properties,
                    range: range_of(node),
                });
            }
            _ => {
                // A relationship of any arity written as `[:TYPE | (a), (b), (c)]`.
                let ends: Vec<Option<String>> = element_nodes
                    .iter()
                    .filter(|n| n.kind() != "doc_comment")
                    .map(|n| match n.kind() {
                        "node_pattern" => self.end_identity(*n),
                        "pattern_reference" => Some(name_of(*n, self.source)),
                        _ => None,
                    })
                    .collect();
                if ends.is_empty() {
                    if let Some(c) = comment {
                        self.model.dangling_comments.push(range_of(c));
                    }
                    return;
                }
                let (identifier, identifier_range) = match (&ident, labels.first()) {
                    (Some((n, r)), _) => (n.clone(), r.clone()),
                    (None, Some(l)) => (l.name.clone(), l.range.clone()),
                    (None, None) => return,
                };
                let def = Def {
                    kind: Kind::Relationship,
                    identifier,
                    identifier_range,
                    identifying: None,
                    identifying_optional: false,
                    implied: Vec::new(),
                    misplaced_separators: Vec::new(),
                    relationship_separators: labels
                        .iter()
                        .filter(|l| l.sep == Sep::Double)
                        .map(|l| l.sep_range.clone())
                        .collect(),
                    record: self.record(node),
                    annotations,
                    element_count: ends.len(),
                    ends,
                    range: range_of(node),
                    top_level: ctx == Ctx::Top,
                    graphs: match ctx {
                        Ctx::Graph(g) => vec![g],
                        Ctx::Top => Vec::new(),
                    },
                    comments: Vec::new(),
                };
                self.push_def(def, info);
            }
        }
    }
}

/// Read a doc comment through gramdoc. `None` when the comment is not valid gramdoc (that is
/// reported separately as `gramdoc-parse-error`).
fn desugar_comment(comment: Node, source: &str) -> Option<CommentInfo> {
    let body = text(comment, source);
    let mut parser = TsParser::new();
    parser
        .set_language(&tree_sitter_gramdoc::LANGUAGE.into())
        .expect("load gramdoc grammar");
    let tree = parser.parse(body, None)?;
    if tree.root_node().has_error() {
        return None;
    }
    let start = comment.start_position();
    let host = |n: Node| -> Range {
        let map = |p: tree_sitter::Point| Position {
            line: (start.row + p.row) as u32,
            character: if p.row == 0 {
                (start.column + p.column) as u32
            } else {
                p.column as u32
            },
        };
        Range {
            start: map(n.start_position()),
            end: map(n.end_position()),
        }
    };
    let mut info = CommentInfo::default();
    let mut aliases: Vec<String> = Vec::new();
    let mut cursor = tree.root_node().walk();
    let mut stack: Vec<Node> = tree.root_node().named_children(&mut cursor).collect();
    stack.reverse();
    while let Some(node) = stack.pop() {
        let value = |field: &str| {
            node.child_by_field_name(field)
                .map(|n| body[n.byte_range()].trim().trim_matches('"').to_string())
        };
        match node.kind() {
            "property_tag" => {
                if let Some(name) = node.child_by_field_name("name") {
                    info.properties
                        .push((body[name.byte_range()].to_string(), host(name)));
                }
            }
            "language_tag" => {
                if let Some(v) = value("value") {
                    info.fields.insert("@language".into(), v);
                }
            }
            "alias_tag" => aliases.extend(value("value")),
            "reference_tag" => {
                if let Some(v) = value("target") {
                    info.fields.insert("@reference".into(), v);
                }
            }
            "unknown_tag" => {
                if let Some(n) = value("name") {
                    info.fields
                        .insert(n, value("description").unwrap_or_default());
                }
            }
            "document" => {
                let mut c = node.walk();
                let mut kids: Vec<Node> = node.named_children(&mut c).collect();
                kids.reverse();
                stack.extend(kids);
            }
            _ => {}
        }
    }
    if !aliases.is_empty() {
        info.fields
            .insert("@aliases".into(), aliases.join("\u{1f}"));
    }
    Some(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(src: &str) -> Model {
        let mut p = TsParser::new();
        p.set_language(&tree_sitter_gram::LANGUAGE.into()).unwrap();
        build(&p.parse(src, None).unwrap(), src)
    }

    fn def<'m>(m: &'m Model, id: &str) -> &'m Def {
        m.defs.iter().find(|d| d.identifier == id).unwrap()
    }

    fn implied(d: &Def) -> Vec<&str> {
        d.implied.iter().map(|l| l.name.as_str()).collect()
    }

    #[test]
    fn identifier_is_the_identifying_label_by_default() {
        let m = model("(Person)");
        let d = def(&m, "Person");
        assert_eq!(d.identifying.as_deref(), Some("Person"));
        assert!(d.implied.is_empty());
    }

    #[test]
    fn double_colon_after_the_identifier_is_the_shorthand() {
        let m = model("(Actor::Person)");
        let d = def(&m, "Actor");
        assert_eq!(d.identifying.as_deref(), Some("Actor"));
        assert_eq!(implied(d), ["Person"]);
        assert!(d.misplaced_separators.is_empty());
    }

    #[test]
    fn explicit_identifier_keeps_plain_gram_reading() {
        let m = model("(actor:Actor:Person)");
        let d = def(&m, "actor");
        assert_eq!(d.identifying.as_deref(), Some("Actor"));
        assert_eq!(implied(d), ["Person"]);
    }

    #[test]
    fn shorthand_applies_with_an_explicit_identifier() {
        let m = model("(actor::Actor:Person)");
        let d = def(&m, "actor");
        assert_eq!(d.identifying.as_deref(), Some("actor"));
        assert_eq!(implied(d), ["Actor", "Person"]);
        assert!(d.misplaced_separators.is_empty());
    }

    #[test]
    fn double_colon_elsewhere_is_misplaced() {
        let m = model("(actor:Actor::Person)");
        assert_eq!(def(&m, "actor").misplaced_separators.len(), 1);
    }

    #[test]
    fn relationship_identifier_is_its_type_unless_explicit() {
        let m = model("(A)=[OWNS]=>(B)\n(A)=[ownsCar:OWNS]=>(B)");
        assert!(m
            .defs
            .iter()
            .any(|d| d.identifier == "OWNS" && d.ends.len() == 2));
        assert!(m.defs.iter().any(|d| d.identifier == "ownsCar"));
    }

    #[test]
    fn annotations_are_looked_through_to_the_definition() {
        let m = model("@cardinality([0..., 1...])\n@unique(x)\n(A)=[R]=>(B)");
        let d = def(&m, "R");
        assert_eq!(d.annotations.len(), 2);
        assert_eq!(d.annotations[0].array_len, Some(2));
    }

    #[test]
    fn top_level_node_and_path_patterns_are_definitions() {
        let m = model("(a)\n()-[r]->()");
        assert!(m.defs.iter().any(|d| d.identifier == "a"));
        assert!(m.defs.iter().any(|d| d.identifier == "r"));
    }

    #[test]
    fn optional_mark_is_stripped_from_the_label_name() {
        let m = model("(Actor::`Director?`)");
        let d = def(&m, "Actor");
        assert_eq!(implied(d), ["Director"]);
        assert!(d.implied[0].optional);
    }
}
