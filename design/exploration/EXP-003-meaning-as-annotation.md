---
id: EXP-003
title: Meaning and Structure as Annotation
status: exploring
created: 2026-10-31
---

# EXP-003: Meaning and Structure as Annotation

## 1. Starting Point

An ontology says what things are called and what they mean: a description, alternative names, a reference to a definition, the word for a thing in another language. gram notation can already carry that in two quite different ways, and they look unrelated: a documentation comment placed next to an element, and ordinary gram patterns that describe an element from outside it. The hunch is that these are one idea written two ways, and that treating the comment as a special syntax for an annotation could settle where meaning should live. The concrete question: where does the meaning of an element live, and in what form, so that it is comfortable to write, survives as data, supports several languages and audiences, and can be checked?

Relationships raise a second question on the same ground. Some things said about a relationship are not meaning but structure, such as how many of them a node takes part in. Those are intrinsic to what the relationship is, so they should not be written the way a description is. Where do they go?

## 2. Landscape

**The notation, briefly.** A gram file is a sequence of patterns. In generic gram a node's identity is simply its identity, and a label is a separate matter. An ontology is a meta graph: it describes kinds of things, so each element needs an identifying label, the one label that names the kind, and an ontology file uses that label as the element's identity. That is a convention of the ontology profile, not of gram. A node type is `(Person {name: "STRING!"})`; a relationship type is `(Person)=[ACTED_IN {role: "STRING"}]=>(Movie)`, identified by its type, `ACTED_IN`. Identities are unique within a file, so a pattern's elements name other patterns by reference, and a reference is always a bare identifier, whether it names a node or a relationship: `[Movies:Graph | Person, Movie, ACTED_IN]`. A reference appears only as an element of a pattern, never as a top-level element. A relationship `(a)-[r]->(b)` is the pattern `[r | a, b]`: a relationship is a pattern with two elements, and a pattern can have any number.

A record value may be a scalar, an array of scalars, or a shallow map whose entries are scalars or arrays of scalars; a map cannot hold a map. A range is a scalar: `1..3`, `1...` (lower bound only), `...3` (upper bound only). A string can carry a tag that names its type or format, ``iso`fr` `` for a language code or ``md`text` `` for markdown. Keys may be bare symbols or backtick-quoted names, so `` `@language` `` is a legal key. A repeated key in a record is a duplicate-key error, but that is a semantic check, not a grammar rule: the grammar parses a repeated key, and `gram lint` reports it (`duplicate-key`). Ontology tooling has to run that check itself.

**Annotations.** `@key(value)` before a pattern annotates it, and the value can be a scalar, an array, or a map. An annotation is a pattern whose single element is the annotated pattern, with the keys as its record: `@cardinality([0..., 1...]) P` and `[{ cardinality: [0..., 1...] } | P]` hold the same data. `@@id:Label` before the `@key(value)` entries gives the annotation an identity and labels. Annotation keys must be plain symbols; a backtick-quoted key is a syntax error there, and a repeated annotation key is accepted silently, which the record spelling does not do.

**Comments.** gram has a documentation comment, `/** ... */`, parsed by a separate grammar (gramdoc) that knows tags: `@property name text`, `@alias`, `@reference`, `@elements`, `@source`, `@target`, `@self`, and it keeps tags it does not know. A doc comment documents the next element in the same container. gram's only multi-line comment is this doc block: `//` is a plain line comment, and a plain `/* */` block is a syntax error. Javadoc, TSDoc, and Kotlin's KDoc likewise read documentation only from `/** */`; Rust, C#, Swift, and Dart add a line form, `///`, which gram does not have. The generic pattern parsers discard comments, so anything written only in a comment is absent from the pattern they build.

**What meaning needs.** From the Neo4j ontology-graph-spec and from two spike files in this repository (`examples/pattern-rs-spike/`): a `description`, `aliases`, and a `reference`, on a node, a relationship, and a property. Properties have no identity of their own, so documenting one means naming it by key. The spec also has a property `name`, the property's name in the database when it differs from its id, which is a different thing from a localized word. One spike file (`movies.ontology.gram`) keeps meaning out of the elements, as separate `Meaning` patterns attached with `@on(...)` and grouped in a `Glossary`; the other (`person.ontology.gram`) documents a node in a comment and adds a second pattern for the French name. The spike also gathers its patterns into a containing pattern that names its members by identity, so a pattern that is to belong to a containing graph needs an identity to be named by. An ontology file holds many such graphs, each an explicit pattern, because the alternative for a multi-graph ontology is references from one file to another. An element can belong to several graphs. These are definitions and descriptions, so sharing is natural, in the way a node can be contained by many relationships; it is what lets graphs overlap along a shared interface, as EXP-001 describes.

**What a relationship needs beyond meaning.** The spec gives a relationship end counts, the cardinality of each end. In the spec's reading, the count at an end is how many relationships of that kind each node at that end takes part in, the reverse of the UML convention. A relationship kind can also be required to be a self-loop, which is a claim about instances (the same node at both ends) and not about types: `(Person)=[KNOWS]=>(Person)` says both ends are persons, not that they are the same person.

## 3. Divergent Framings

### 3.1 The doc comment is the home

Meaning is written in `/** ... */` next to the element, in the comment's tags.

```gram
/**
 * A human individual.
 *
 * @property name The full name of the person.
 */
(Person {name: "STRING!"})
```

A reference cannot be a top-level element, so a second language has to sit before a pattern that names the element. A `Doc` with no fields of its own can carry the comment, and take its content from it:

```text
/**
 * Un être humain.
 *
 * @language "fr"
 * @property name "nom" Le nom complet de la personne.
 */
[:Doc | Person]
```

Writing it is natural, and it reads as documentation. The costs: the content is free text with tags inside a comment, so a generic pattern parser never sees it; each language repeats the comment, and needs a `Doc` to carry it; and `@language` and a quoted localized name are not tags gramdoc knows today (it keeps them as unknown tags, or as description text).

### 3.2 An annotation pattern, written as a record

Meaning is data: a labeled pattern whose element is the thing documented, referenced by its bare identifier.

```gram
[Person_fr:Doc {
  `@language`: iso`fr`,
  `@description`: md`Un être humain.`,
  `@aliases`: ["Personne"],
  name: {
    term: "nom",
    description: md`Le nom complet de la personne.`,
    aliases: ["anon"]
  }
} | Person]
```

The `Doc` has an identity, `Person_fr`, so that a containing graph can list it as a member. Its own fields use the `@` prefix through backtick-quoted keys, and a plain key is the name of a property of `Person`, whose value is a small map. This survives any parser, keeps arrays in the per-property map, and lets tooling reject a repeated key, which the grammar does not do on its own. It is noisy to write by hand, and nothing in the grammar enforces the `@` convention.

### 3.3 An annotation pattern, written as annotations

The same data, with the annotation syntax.

```gram
@@Person_fr :Doc
@language(iso`fr`)
@description(md`Un être humain.`)
@aliases(["Personne"])
@name({ term: "nom", description: md`Le nom complet de la personne.`, aliases: ["anon"] })
(Person)
```

The identified annotation gives the `Doc` an identity and a label, and the values can be arrays. Its keys are the plain `language`, `description` and `aliases`, not the `@`-prefixed keys of 3.2, so the two forms are different conventions, not interchangeable spellings. It reads as a prefix on the element. Annotation keys cannot be quoted, so there is no `@` convention available, a property called `description` would clash with the `Doc`'s own `description` with no error, and a repeated key goes unnoticed, since the duplicate-key check applies to records.

### 3.4 The comment is surface syntax for an annotation pattern

The doc comment is authoring syntax, and the model is a `Doc` pattern as in 3.2. Reading a comment produces the pattern. A comment names no identity for the `Doc` it produces, and none is needed: the reading puts the `Doc` inline among the elements of each graph that contains the documented element, where an inline member needs no identity. Identities are not minted. The cost is that the desugaring has to be defined exactly. Section 5 states it.

### 3.5 Where a relationship's cardinality lives

Five places were considered for the counts at a relationship's ends.

- **In the `Doc`.** The counts sit beside the description. But `Doc`s accumulate, one per language, so several could state a cardinality and disagree, and cardinality is not language-specific.
- **In a sibling pattern that references the relationship**, `[:Constraint {...} | ACTED_IN]`, like a `Doc`. It can sit in any graph and accumulate, which is wrong for something that changes what the relationship is.
- **In the relationship's own record**, under `@`-prefixed keys: `(Person)=[ACTED_IN { `@cardinality`: [0..., 1...], role: "STRING" }]=>(Movie)`. Intrinsic, but the record holds property declarations, and this mixes structure into it.
- **As annotations that wrap the definition.**

  ```gram
  @cardinality([0..., 1...])
  (Person)=[ACTED_IN { role: "STRING" }]=>(Movie)
  ```

  Written out as patterns, the same data is `[{ cardinality: [0..., 1...] } | [ACTED_IN { role: "STRING" } | Person, Movie]]`. The annotation's keys are its own, so they cannot collide with the relationship's properties; written as a record, a repeated key is a duplicate-key error once the semantic check runs; and the nesting is the point: the annotation wraps the definition, where a `Doc` only references its target.
- **Dedicated syntax at the arrow**, which would read as part of the relationship and needs a grammar change.

Within the wrapping form, cardinality can be keyed by end, `{ source: 0..., target: 1... }`, or positional, `[0..., 1...]`, one range per element of the pattern in the pattern's order. Keyed only makes sense for a directed binary relationship. Positional works for any arity (`@cardinality([0..., 1..., 1]) [PRESCRIBED | Doctor, Drug, Patient]`), needs no words for direction, and makes `?`, `+`, and `*` unnecessary, since a range or an exact count says the same.

## 4. Tensions

- **Comfort versus survival as data.** 3.1 is the easiest to write and the only one that vanishes in a plain pattern parse. 3.2 and 3.3 survive and are awkward. 3.4 tries to have both, at the price of a defined desugaring.
- **One namespace for two kinds of key.** In a `Doc` record, the `Doc`'s own fields and the names of the documented element's properties share the key space. The `@` prefix separates them in a record (3.2) and cannot be used in annotation syntax (3.3). Without a convention, a property named `description` or `language` cannot be documented.
- **How much structure fits per property.** A per-property map can hold scalars and arrays of scalars, so a description, a localized term, a reference, and a list of aliases fit. Anything deeper needs another pattern.
- **Where the language lives.** On each comment or `Doc`, as a qualifying field, so a reader can pick the `Doc`s in the language they want. Beyond `@language` the convention is deliberately left open: a doc comment keeps tags it does not know, and a `Doc` record can carry further `@`-prefixed fields, so an author who needs to tell `Doc`s apart by audience, version, or anything else can evolve their own convention.
- **Where a second-language comment sits.** A reference is never a top-level element, so a comment cannot document a reference directly. The candidate is a field-less `Doc`, `[:Doc | Person]`, that takes its content from the comment. Whether that is the right carrier, or the comment should be written as a full `Doc`, is open.
- **Intrinsic versus extra.** A `Doc` references its target and is extra; cardinality wraps the definition and is intrinsic. The cost of wrapping is that a relationship's outermost pattern may be an annotation, and tools look through it to the relationship, whose identity is unchanged.
- **Positional counts are less self-describing.** `[0..., 1...]` makes the reader pair each range with an element. The array must be as long as the pattern has elements, which is a check, and the order matters for a relationship whose ends are interchangeable.
- **Self-loops are not cardinality.** The positional ranges count participation; they cannot say that two elements are the same node. A self-loop requirement needs its own attribute.
- **Where the rules are checked.** The content of a comment is checked by a comment grammar, and the content of a record by whatever describes records of this kind. 3.4 puts both in play for one piece of meaning.
- **Two spellings, one set of data, two key conventions.** 3.2 and 3.3 carry the same prose but not the same keys: the `Doc` record uses `@`-prefixed metadata keys (`` `@language` ``), while the annotation `@language(...)` produces the plain key `language`, which in a `Doc` record would name a documented property. A tool that accepts both must normalize the metadata keys and decide what happens when a property is itself called `language` or `description`.

## 5. Directions Worth Pursuing

3.4 over 3.2 looks like the strongest combination for meaning: the model is an annotation pattern in record form, which survives every parser, allows a duplicate-key check (a semantic check the ontology tooling must run, since the grammar parses repeated keys), and has an `@` convention, and the doc comment is the comfortable way to write it. 3.3 is the form with the most open problems. For structure, the wrapping annotation with positional cardinality (3.5). Around that:

- **Identity follows containment.** An ontology file holds many `Graph` patterns, which list their members by identity. A top-level `Doc` the author writes therefore carries an identity, and an anonymous top-level `Doc` is a warning. A `Doc` produced from a comment carries none: it is produced inline in each graph that contains the documented element, so nothing is minted. In an ontology, elements are identified by their identifying label (nodes) and type (relationships), a convention of the ontology profile and not of gram, and a reference is a bare identifier among a pattern's elements.
- **Elements are shared, and `Doc`s accumulate.** An element can belong to several graphs and carries its meaning into each. A graph can hold any number of `Doc`s for one element (languages, audiences, versions), and nothing overrides anything. Choosing which to read is the reader's job, and an author limits the choice by including only the `Doc`s that apply. A comment and an authored `Doc` for the same element simply both exist.
- **The graph is the context.** A graph that holds the French and English `Doc`s of an element already groups them, and a separate audience can have a separate graph over the same elements, so no extra grouping construct is needed.
- **Orphans.** A top-level node, relationship, or annotation not referenced from within any `Graph` is orphaned, and an orphan is a warning.
- **Desugaring a doc comment.** The target is the next sibling in its container, looking through any intrinsic annotation wrappers: for a comment placed before `@cardinality(...)` and a relationship definition, the target is the relationship, whose identity and graph membership are checked, not the anonymous wrapper. A comment on a definition produces one `[:Doc {...} | X]`, where `X` is the definition's identity, in each graph that contains it; a comment on an element with no identity is a warning. A comment on a `Doc` that has no fields, `[:Doc | X]`, fills that `Doc`. The free text becomes `` `@description` `` as a markdown string, md`...`; `@language "x"` becomes `` `@language` `` for the whole comment, as an ISO-tagged string, iso`x`; `@alias` and `@reference` become `` `@aliases` `` (an array) and `` `@reference` ``; `@property key "term" text` becomes an entry `key: { term, description }`, with the description a markdown string and the term a plain string; an unknown tag becomes an `@`-prefixed field of the same name.
- **Properties are addressed by map entry.** One `Doc` holds an entry per property, keyed by its name, with its description, localized term, aliases, and reference. A property that needs more than scalars and arrays is promoted to its own pattern, as gram's own depth budget says to do.
- **Documentation says nothing about the ends of a relationship.** A description of a source or target adds little. What matters about an end is its cardinality, which is structure.
- **Cardinality is a wrapping annotation with positional ranges.** `@cardinality([0..., 1...])` before the definition, written with gram's range literals. For element *i* the range is how many patterns of this kind each such element takes part in. The array's length must equal the pattern's number of elements. A self-loop requirement stays a separate attribute.
- **Two tiers of annotation.** A `Doc` is lightweight, semi-formal extra information, labeled, accumulating, and optional to read. An `@` annotation is extra definition: anonymous, intrinsic, written once, and changing what the thing is. This echoes object-oriented languages, which mark a declaration with an annotation, attribute, or decorator to say more about what it is, and keep prose documentation separate. Comments carry the first tier and the definition carries the second.
- **Intrinsic annotations are anonymous.** In entity-component-system terms, a named annotation identifies a specific system, and an anonymous one is an intrinsic, extensional attribute. The aim is to accumulate more intrinsic attributes in that space before specializing. A kind of annotation gets a label when it becomes a system of its own, and table definitions are the likely first.
- **A self-loop is `@self(true)`.** `@cardinality([0..., 0...]) @self(true) (Person)=[KNOWS]=>(Person)` parses today. An implicit-true form would need gram to change, and the options differ: an empty `@self()` is a syntax error now but would be unambiguous; a bare `@self` cannot work, because `@self (n)` is read as `@self(n)` with the value `n`. What an empty value means in general (true, null, empty) is a question for gram as a whole, so it is deferred.
- **gramdoc drops `@source`, `@target`, and `@self`.** They are redundant under this split, and the removal is decided. It is a small breaking change to a published grammar.

Underspecified on purpose: beyond `@language`, how `Doc`s are told apart is left to authors, since unknown tags and fields are kept.

## 6. Disposition

Candidate RFCs, not yet written: the shape of a documentation pattern (`Doc`): its fields, the `@` convention, per-property entries, and how a doc comment reads into it; and the shape of the structural annotation on a relationship definition, starting with cardinality. A narrower candidate follows in the gramdoc grammar: `@language`, a quoted localized term after `@property`'s key, the rule that a comment on a reference documents the referenced element, and the removal of `@source`, `@target`, and `@self`, which is decided. Items to settle first:

- The carrier for a second-language comment: a field-less `[:Doc | X]`, or something else.
- Whether gram gets a line-style doc comment, `///`, as Rust, C#, Swift, and Dart have. Not needed for a first RFC: the block form is enough, and a `///` form could be added later.
- The orphan rule and the anonymous top-level `Doc` warning: rules for the ontology profile to check (they join the semantic requirements listed in EXP-002), applying to every kind of top-level pattern.
- The length check on positional cardinality.

Out of scope here, and the next ground to cover: multiple graphs and the bridges between them, and how other domain-specific annotations attach, such as indexes and table definitions. Cardinality is the first non-documentary annotation, and the wrapping shape is what those would reuse.

## 7. References

- The Neo4j ontology-graph-spec: `description`, `aliases`, and `reference` on nodes, relationships, and properties, relationship end counts, and a property `name` distinct from its id.
- Annotations, attributes, and decorators in object-oriented languages (Java annotations, C# attributes, Python and TypeScript decorators): prior art for stating extra definition on a declaration with a marker written before it.
- SKOS and RDFS labels: `rdfs:label`, `rdfs:comment`, `skos:prefLabel`, and `skos:altLabel`, with language tags on the literal. Prior art for a localized term and its alternatives, attached from outside the thing named.
- OWL annotation properties: statements about an entity that carry no logical meaning, attached to the entity from outside it.
