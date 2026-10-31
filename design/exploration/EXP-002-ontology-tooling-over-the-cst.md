---
id: EXP-002
title: Ontology Tooling over the gram Syntax Tree
status: exploring
created: 2026-10-30
---

# EXP-002: Ontology Tooling over the gram Syntax Tree

## 1. Starting Point

If ontologies are written as gram files (`*.ontology.gram`), what should check them, interpret them, and export them, and at which layer should that work happen? A file can be handled as text, as a concrete syntax tree, as a generic pattern, or as a domain object model. Each choice puts the strictness, the meaning, and the tooling in a different place.

## 2. Landscape

gram notation has a tree-sitter grammar, so every gram file has a concrete syntax tree (CST) that keeps positions and comments. That grammar can be extended without changing the host language. Structured documentation comments (`/** ... */`) are one example: the comment is a single node in the host tree, and a separate grammar, gramdoc, is injected into it. gramdoc defines tags for documenting an element: `@property` and `@label` for parts of the subject, `@elements`/`@element` for the pattern sequence, `@source`/`@target` with a cardinality for the two ends of a relationship, `@self` for self-loops, and `@alias` and `@reference`. A doc comment documents its next sibling in the same container.

The `gram` command-line tool dispatches unknown sub-commands to external `gram-<name>` binaries, installed from GitHub release artifacts listed in a registry. An ontology tool could be such a binary. Separately, a gram file can also be parsed natively (without tree-sitter) into a generic pattern data structure, a path that discards comments.

The Neo4j ontology-graph-spec covers `description`, `aliases`, `reference`, and relationship end counts in a JSON/YAML shape that nests meaning inside elements. It is a lower bound for what an ontology needs to express here, and it lacks features a multi-graph space needs, such as spans between graphs.

## 3. Divergent Framings

### 3.1 A profile over the CST

A conforming file is ordinary, valid gram with extra requirements. A `gram ontology` extension adds syntactic requirements (for example a canonical form for relationships) and semantic requirements (every relationship documents `@source` and `@target`, `@property` names exist on the documented element, cardinalities are consistent, `@self` only where both ends are the same node), and still produces the normal CST, so highlighting, the language server, and generic checking keep working on ontology files. A file opts in with both the `*.ontology.gram` name and `kind: ontology` in the top-level record. The `kind` is also the hook for a future gram schema (see section 5). For now, rules are delivered as a mix: tree-sitter queries for syntactic rules and Rust for rules that must read the host tree and the injected gramdoc tree together. Export is JSON of the complete ontology, plus a lossy export to the ontology-graph-spec.

### 3.2 A library over the generic pattern

Ontology semantics are interpreted after parsing, from the generic pattern structure. Graph notation and pattern notation are normalized by the pattern layer, so the rules deal with one shape. The cost is that comments are discarded on the native path and not carried into the pattern, so documentation would have to be threaded through the pattern structure first.

### 3.3 A dedicated tree-sitter grammar for ontologies

Ontology files get their own grammar, one member of a family of specialized gram grammars, each parsing into a generic pattern but with domain objects on top. Syntax can be tailored exactly. The cost is a second grammar to maintain, and ontology files stop being plain gram unless the new grammar is a strict subset.

## 4. Tensions

- **Where strictness lives.** 3.1 keeps the language open and puts strictness in a checker. 3.3 puts strictness in the grammar. They disagree about whether an invalid ontology should be a syntax error or a diagnostic.
- **Which tree carries documentation.** The CST keeps comments, and the pattern layer does not today. 3.1 and 3.2 disagree about whether meaning is read before or after normalization.
- **Two syntactic forms.** gram expresses a relationship as `(a)-[:R]->(b)` or as `[:R | a, b]`. Working on the CST means handling both or requiring a canonical form. Working on patterns means handling neither.
- **Coverage.** The ontology-graph-spec is a floor, and multi-graph framings such as spans between graphs and layered semantics are outside it, so a model shaped to the spec alone may not hold them.

## 5. Directions Worth Pursuing

3.1 is the working direction. It keeps one base tree, adds no new syntax, and fits the existing extension mechanism. Within it:

- The shape of a valid `*.ontology.gram` file is the source of truth. JSON, including any JSON Schema, is a serialization of that shape, never the contract.
- Gram will eventually have its own schema definition, associated with a `kind`, that captures the conventions and semantics this project has to enforce in code and queries today. A file therefore does not name a schema in its top-level record: `kind` is the association. A few kinds of gram will be worked out by hand first, and ontology is the first; a gram schema is distilled from those.
- Until then, syntactic rules as queries and semantic rules as Rust, written so that they can later be distilled into a schema. They are the hand-built version of that schema, not a competing mechanism.
- Diagnostics in the same JSON format `gram check` already emits.

3.3 is kept in reserve for a domain whose syntax genuinely cannot be valid gram.

## 6. Disposition

Candidate RFC, not yet written: the ontology profile over the gram CST, including how a file opts in and what the model is. Open items that an RFC would need to settle:

- Define the model, using EXP-001 as a starting point.
- What happens when the filename and `kind: ontology` disagree. A warning on any mismatch is the current leaning, and the skeleton already does that.
- Which attachment rules for doc comments to adopt (adjacency, as proposed for gramdoc).
- Which conventions belong to `kind: ontology` and which are general gram, since the former is what a gram schema will eventually carry.

## 7. Appendix: References

- The gramdoc design proposal in gram-data/tree-sitter-gram (`proposals/tree-sitter-gramdoc.md`): tags, cardinality semantics, and doc-comment attachment.
- The query-driven lint proposal in gram-data/tree-sitter-gram (`proposals/gram-lint-rules.md`): `tags.scm`, `locals.scm`, and lint queries as a rule-delivery mechanism.
- The `gram` extension registry contract in gram-data/tree-sitter-gram (`specs/007-unified-gram-cli/contracts/extension-registry.md`): artifact naming and installation.
- The unified ecosystem (unifiedjs.com), as a model of a common base syntax tree with separately packaged extensions.
- Neo4j ontology-graph-spec, as the lower bound for coverage.
