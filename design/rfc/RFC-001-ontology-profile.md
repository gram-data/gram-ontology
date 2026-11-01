---
id: RFC-001
title: An Ontology Profile for gram
status: draft
created: 2026-11-01
---

# RFC-001: An Ontology Profile for gram

## 1. Summary

An ontology describes the kinds of things in a domain: the types of entity, how they relate, what properties they carry, and what each of them means. A property graph keeps its own model only implicitly, in the data, so an ontology written next to it is how people and tools share the model. gram is a notation for graphs written as nested patterns, where a file is a sequence of patterns made of nodes, relationships, records and annotations.

Ontologies can be written in plain gram as a profile: a file named `*.ontology.gram` whose top-level record declares `kind: ontology`. A profile adds conventions and checks on top of valid gram and changes none of its syntax, so every gram tool, such as the parser, highlighter and language server, keeps working on an ontology file. The profile settles six things: how an element is identified, how elements are grouped into graphs, how meaning (descriptions, aliases, references, localized names) is attached, how structure (cardinality, self-loops) is attached, how a node's labels are written, and how properties and constraints are declared. It also states the checks a tool must make on a conforming file.

## 2. Problem / Motivation

The Neo4j ontology-graph-spec describes a graph model as YAML or JSON: nodes with labels and properties, relationships with two ends, constraints, and per-element `description`, `aliases` and `reference`. That shape nests meaning inside each element, and it gives a model no place for a second language or a second audience of the same description. Its relationship ends carry an id and a label that repeat what the surrounding structure already says.

The spec holds one `description` string per element, so a second language or a second audience has nowhere to go. Written in gram, the same need has several plausible shapes and nothing says which is right. A French description of a `Person` can be a node that renames the property it describes:

```gram
[:French { name: "nom" } | Person]
```

or a pattern that carries the description and the localized term as data:

```gram
[:Doc {
  `@language`: iso`fr`,
  `@description`: md`Un être humain.`,
  name: { term: "nom", description: md`Le nom complet de la personne.` }
} | Person]
```

A node's identity can be a spec-style id (`person`) or its label (`Person`), and meaning can sit in separate patterns grouped into a glossary or in the element's own record. Both shapes above are valid gram and neither is reported. A typo in a documented property name, a repeated record key, or a pattern that no graph refers to is accepted the same way.

A doc comment, the natural place to write a description, is discarded by the generic pattern parsers. Anything written only there is absent from the model they build.

## 3. Goals

- A file written to the profile is valid gram, and every gram tool still reads it.
- Everything the spec says about nodes, relationships, labels, properties, constraints and meaning can be written, and what the spec does not say (several languages, several audiences) can be added without changing a definition.
- The model is the shape of a valid ontology file. Other formats are serializations of it.
- A conforming file can be checked by a tool: syntax, conventions, and the mistakes that gram alone does not catch.
- The profile is hand-built first, so that its conventions can later be distilled into a gram schema associated with `kind`.

## 4. Non-Goals

- Any change to gram's syntax, other than as an open question below.
- Connecting several ontologies, and the bridges between graphs.
- Other domain-specific annotations, such as indexes, table definitions and data mappings, and the spec's extension data (display layout, data sources, import mappings).
- Enforcing an ontology against data in a database.
- Defining the gram schema language.

## 5. Proposed Approach

### 5.1 The profile and its identities

A file opts in by both its name and its `kind: ontology` record, and a mismatch between them is a warning. An ontology is a graph about kinds of things, so in the profile an element's identity is the name it is known by: a node's identifier is its identifying label, and a relationship's identifier is its type. `(Person)` is the identifier and the label at once, and the label is not written twice. An identifier is unique within a file, so a node and a relationship cannot share a name, and a relationship type has one definition. A type used between different pairs of nodes, such as `OWNS` from both `Person` and `Company` to `Car`, is therefore not yet expressible by default (see 7.6). In plain gram an identity is only an identity, and this convention belongs to the profile.

A reference to an element is its bare identifier, written among another pattern's elements, and a reference is never a top-level element. A file holds any number of `Graph` patterns, each a pattern whose elements list the graph's members by identity. An element may belong to several graphs, since a definition is shared by nature. A top-level node, relationship or annotation that no graph refers to is orphaned, which is a warning, as is a top-level `Doc` with no identity.

### 5.2 Meaning as an accumulating `Doc`

Meaning is carried by a `Doc` pattern that references the element it describes. Its own fields use `@`-prefixed keys, written in backticks (`@description`, `@language`, `@aliases`, `@reference`, `@labels`), so they cannot collide with the names of the properties it documents. A property name that begins with `@` is not allowed in an ontology file, which reserves the prefix for metadata. Each documented property is an entry keyed by its name, holding a localized term, a description, and any further attributes. Descriptions are markdown, and a language is an ISO-tagged string.

Several `Doc`s may describe one element, one per language, audience or version, and nothing overrides anything: a reader chooses which to read, and the graph an element appears in is the context. Authors who need to tell `Doc`s apart beyond language add their own `@`-prefixed fields.

Authoring is by doc comment, a `/** ... */` block whose tags are parsed by gramdoc, which the profile reads into a `Doc`. The text becomes the description, and `@language`, `@alias`, `@reference`, `@property` and `@label` become the corresponding fields and entries. A comment on a definition produces its `Doc` inline in each graph that contains the definition, so no identity is minted for it. A comment on a reference, such as a second-language description, fills a field-less `[:Doc | X]`. The comment is read through any annotation wrappers to the definition they wrap. Unknown tags are kept as `@`-prefixed fields. The comment below reads into the French `Doc` shown in the motivation:

```gram
/**
 * Un être humain.
 *
 * @language "fr"
 * @property name "nom": Le nom complet de la personne.
 */
[:Doc | Person]
```

### 5.3 Structure as a wrapping annotation

What a relationship is, as opposed to what it means, is intrinsic and is written as an anonymous annotation that wraps the definition. Cardinality is one range per element of the pattern, in the pattern's order, using gram's range notation, so it works for a relationship of any arity. A self-loop is a separate attribute, since counts say how many and not whether two elements are the same node. The array length must equal the number of pattern elements. Each range counts how many relationships of that kind a node at that element takes part in, so in the example below each `Person` acts in any number of movies and each `Movie` has at least one actor. This is the reading of SHACL's `sh:minCount` and `sh:maxCount`, which count values per node of the shaped class. Entity-relationship diagrams in crow's-foot notation, which tools such as Mermaid's `erDiagram` use, put the number at the other end, counting the entities at that end per one entity at the opposite end, so an exporter to a diagram must swap the ends.

```gram
@cardinality([0..., 1...])
(Person)=[ACTED_IN { role: "STRING" }]=>(Movie)
```

Documentation is an extra reference to a definition, and an annotation is part of the definition. A tool looking at a relationship looks through its annotations.

### 5.4 Labels

The identifier is the identifying label. Any other label on a node is implied, meaning every node of the kind has it, and the double colon is the preferred separator: `(Actor::Person)` says every actor is a person. A label that only some nodes have is optional and ends in `?`, which for now is written inside backticks, `` `Director?` ``. A label with a trailing `?` is always optional, and the identifying label cannot be. A label that matches a defined node is a reference to that definition, and an implied label that matches none is a warning. A relationship end is the reference and needs no separate label. Any label may be documented where it is used, in the `Doc`'s `@labels` map, keyed by the label name without its `?`.

### 5.5 Properties, constraints and model metadata

A node or relationship declares its properties in its record, one entry per property, whose value names the property's type. A requirement that a property exist is written into the type as a trailing `!`. Other constraints on a property, such as uniqueness and keys, are anonymous annotations that wrap the definition, in the same tier as cardinality. The model's own name, version and description sit in the record of its `Graph` pattern.

```gram
@unique(title)
(Movie { title: cypher`STRING` })
```

### 5.6 Checks

A tool conforming to the profile reports, beyond syntax:

- a mismatch between the file name and `kind`;
- a repeated key in a record, which gram parses but does not accept as a record;
- orphaned top-level patterns and top-level `Doc`s without an identity;
- a documented property that does not exist on the element, and a doc comment that has no target;
- a cardinality whose length differs from the number of pattern elements, and a self-loop declared where the relationship's ends are not the same node;
- an implied label with no definition, a cycle among implied labels, and a trailing `?` on an identifying label;
- a property name beginning with `@`, and two definitions with the same identifier;
- two `Doc`s that cannot be told apart: for one element in one graph, the same `@language` and the same values for every other `@`-prefixed field except the description, labels and property entries. Copies of one comment's `Doc` in different graphs, and `Doc`s that differ by audience or version, are not duplicates.

## 6. Alternatives Considered

- **Author the spec's YAML or JSON.** Rejected: it nests meaning inside each element and cannot hold several descriptions of one, and a model in gram is a file that the rest of the gram tools read with no translation.
- **Conventions without a profile.** Rejected: nothing would report a mistyped property name, a repeated key or an orphan.
- **Meaning nested inside each element.** Rejected: a second language or audience would edit the definition, and nothing could hold several descriptions of one element.
- **Meaning only in doc comments.** Rejected: generic parsers drop comments, so the model would lose its meaning.
- **Cardinality in the `Doc` or in the relationship's record.** Rejected: `Doc`s accumulate and could disagree about a count, and a record holds property declarations, not structure.
- **Cardinality as a pattern that references the relationship.** Rejected: it can sit in any graph and accumulate, which is wrong for something that changes what the relationship is.
- **Optional and implied labels as annotations, or as relationships between label definitions.** Rejected: labels stay on the node, where tools that show labels expect them; an annotation, or a relationship, would need a reserved name that a domain could not use.
- **A grouping construct for glossaries or contexts.** Rejected: a graph that lists the `Doc`s for an element already is that context.
- **Cypher's label expression operators for the optional mark.** Rejected: they describe which nodes to match, so operators like `|` and `!` have no meaning in a definition.

## 7. Open Questions

7.1. Whether a terminating `?` on a label should become part of gram, with plain gram treating it as decorative, if review finds the backtick spelling unacceptable. Other tools read `` `Director?` `` as a label whose name includes the `?`.

7.2. Whether gram should allow an empty annotation value that means true, so a self-loop can be written `@self`.

7.3. Whether gram needs a line-style doc comment, as Rust and C# have, beyond the block form.

7.4. Whether a required property stays folded into its type (`STRING!`) or becomes a separate constraint, as the spec keeps them.

7.5. Whether `Doc`s that cannot be told apart are a warning or an error, and whether the checks in 5.6 are configurable in severity.

7.6. How to define a relationship type used between several pairs of nodes, and a node and relationship that share a name. Candidates: an explicit identifier distinct from the label or type for the definitions that need one, as plain gram already allows, or a qualified identifier.
