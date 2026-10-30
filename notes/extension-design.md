# gram-ontology: extension design notes

**Status:** Early notes, not a spec. Captures decisions made on 2026-10-30; layout and details are still to be worked out.

## What this is

`gram-ontology` is planned as an extension to the `gram` CLI (`gram ontology ...`, binary `gram-ontology`). It works on the tree-sitter-gram concrete syntax tree (CST), before any `Pattern` is built. The native (nom) parser path in pattern-rs is not involved.

The repository name follows the registry's `gram-<name>` convention for extensions (the registry contract's own example is `gram-data/gram-fmt` with binary `gram-fmt`).

## The base tree stays the tree-sitter-gram CST

Everything expressible is expressible in tree-sitter-gram. Extensions add syntax or strictness without introducing a different tree:

- **gramdoc** adds structured documentation comments, by injection into the host grammar's `doc_comment` (`/** ... */`). See [`proposals/tree-sitter-gramdoc.md`](https://github.com/gram-data/tree-sitter-gram/blob/main/proposals/tree-sitter-gramdoc.md) and [tree-sitter-gramdoc](https://github.com/gram-data/tree-sitter-gramdoc).
- **ontology** adds strict syntactic and semantic requirements on top of valid gram. A conforming file still parses to a normal, valid CST, so highlighting, the LSP, `gram check` and editor extensions keep working on ontology files. `gram ontology check` is a stricter layer, not a different language.

The unifiedjs ecosystem (a common base tree with packaged extensions) is a reference for this shape, not for runtime or tooling.

## The model comes first

The shape of a valid `*.ontology.gram` file is the source of truth. Gram does not yet have a schema definition language, so defining that shape is part of this work.

JSON, including any JSON Schema, is a serialization of that model. It is not the model's contract, and the design should not work backward from JSON. The Neo4j ontology-graph-spec feels awkward for the opposite reason: its shape suits an object-oriented model more naturally than a graph model. The spec is a lower bound for coverage, and it is missing features a multi-graph space needs, such as a version of the frame/span approach explored in [structural-ontology-notes.md](structural-ontology-notes.md) and in pattern-hs.

## Opt-in

A file opts in with both:

- the filename convention `*.ontology.gram`
- `kind: ontology` in the top-level record (the host grammar allows one optional record before the patterns)

Open: what happens when they disagree (extension without `kind`, or `kind` in a plain `.gram` file). Leaning toward a warning on any mismatch, applying the full profile when both are present.

## Rules

Rule delivery is a mix:

- **Syntactic rules** as tree-sitter queries, in the style of tree-sitter-gram's `proposals/gram-lint-rules.md` (query-driven lint under `queries/lint/`).
- **Semantic rules** in Rust, where a rule must read the host tree and the injected gramdoc tree together. Examples: `@property` names exist on the documented element; `@source` and `@target` appear only on arity-2 elements; cardinalities are consistent (`n <= m`); `@self` only where both ends are the same node.

Diagnostics should use the same JSON format as `gram check` (the published `gram-diagnostics` crate).

Doc comments attach to elements by adjacency; the proposed rules are in the gramdoc proposal ("Attachment"). Cardinality on `@source`/`@target` is relationships of that kind per node at that end (the ontology-spec's meaning, the reverse of UML).

## Export

- **Primary:** JSON of the complete gram ontology. A JSON Schema for it is welcome, derived from the model above.
- **Secondary:** a lossy, limited export to the Neo4j ontology-graph-spec, with a documented list of what it drops.

## Open items

- Define the ontology model (the shape of a valid `*.ontology.gram`), informed by the structural notes (frames, spans, layering informal semantics before tightening boundaries).
- Opt-in mismatch behavior.
- The `gram` extension registry (`registry.toml`) does not exist yet. `gram extension install` fetches it from tree-sitter-gram's `main`, which currently returns 404. Where it should live, avoiding circular dependencies, is the next task.
- Project layout, crate structure, release packaging (artifact naming per the registry contract).
