---
id: EXP-003
title: Meaning as Annotation
status: exploring
created: 2026-10-31
---

# EXP-003: Meaning as Annotation

## 1. Starting Point

An ontology says what things are called and what they mean: a description, alternative names, a reference to a definition, the word for a thing in another language. gram notation can already carry that in two quite different ways, and they look unrelated: a documentation comment placed next to an element, and ordinary gram patterns that describe an element from outside it. The hunch is that these are one idea written two ways, and that treating the comment as a special syntax for an annotation could settle where meaning should live. The concrete question: where does the meaning of an element live, and in what form, so that it is comfortable to write, survives as data, supports several languages and audiences, and can be checked?

## 2. Landscape

**The notation, briefly.** A gram file is a sequence of patterns. A node is `(id:Label {key: value})`. A bare `(Person)` has only an identity, and because identities are unique within a file it is a reference to the element with that identity defined elsewhere (a label would be written `(:Person)`). A subject pattern `[id:Label {record} | element, element]` has an identity, labels, a record of properties, and elements. A record value may be a scalar, an array of scalars, or a shallow map whose entries are scalars or arrays of scalars; a map cannot hold a map. Keys may be bare symbols or backtick-quoted names, so `` `@language` `` is a legal key. A record rejects a repeated key as a duplicate.

**Annotations.** `@key(value)` before a pattern annotates it, and the value can be a scalar, an array, or a map. `@@id:Label` before the `@key(value)` entries gives the annotation an identity and labels. Annotation keys must be plain symbols; a backtick-quoted key is a syntax error there, and a repeated annotation key is accepted silently. Once parsed into the generic pattern structure, an annotation and the equivalent pattern with a record hold the same data, so the two spellings differ only on the page.

**Comments.** gram has a documentation comment, `/** ... */`, parsed by a separate grammar (gramdoc) that knows tags: `@property name text`, `@alias`, `@reference`, `@elements`, `@source`, `@target`, `@self`, and it keeps tags it does not know. A doc comment documents the next element in the same container. The generic pattern parsers discard comments, so anything written only in a comment is absent from the pattern they build.

**What meaning needs.** From the Neo4j ontology-graph-spec and from two spike files in this repository (`examples/pattern-rs-spike/`): a `description`, `aliases`, and a `reference`, on a node, a relationship, and a property. Properties have no identity of their own, so documenting one means naming it by key. The spec also has a property `name`, the property's name in the database when it differs from its id, which is a different thing from a localized word. One spike file (`movies.ontology.gram`) keeps meaning out of the elements, as separate `Meaning` patterns attached with `@on(...)` and grouped in a `Glossary`; the other (`person.ontology.gram`) documents a node in a comment and adds a second pattern for the French name. A language, or an audience, is a context in which a whole group of meanings applies. That spike also gathers its patterns into a containing pattern that names its members by identity, `[movies::Ontology {...} | person, movie, acted-in]`, so a pattern that is to belong to a containing graph needs an identity to be named by. An ontology file holds many such graphs, each an explicit pattern, because the alternative for a multi-graph ontology is references from one file to another. An element can belong to several graphs. These are definitions and descriptions, so sharing is natural, in the way a node can be contained by many relationships; it is what lets graphs overlap along a shared interface, as EXP-001 describes.

## 3. Divergent Framings

### 3.1 The doc comment is the home

Meaning is written in `/** ... */` next to the element, in the comment's tags.

```gram
/**
 * A human individual.
 *
 * @property name The full name of the person.
 */
(person:Person {name: "STRING!"})
```

A second language is a second comment on a reference to the same element:

```text
/**
 * Un être humain.
 *
 * @language "fr"
 * @property name "nom" Le nom complet de la personne.
 */
(person)
```

Writing it is natural, and it reads as documentation. The costs: the content is free text with tags inside a comment, so a generic pattern parser never sees it; each language repeats the comment; and `@language` and a quoted localized name are not tags gramdoc knows today (it keeps them as unknown tags, or as description text).

### 3.2 An annotation pattern, written as a record

Meaning is data: a labeled pattern, with an identity, whose element is the thing documented.

```gram
[person-fr:Doc {
  `@language`: "fr",
  `@description`: "Un être humain.",
  `@aliases`: ["Personne"],
  name: {
    term: "nom",
    description: "Le nom complet de la personne.",
    aliases: ["anon"]
  }
} | (person)]
```

The `Doc` has an identity, `person-fr`, so that a containing graph can list it as a member. Its own fields use the `@` prefix through backtick-quoted keys, and a plain key is the name of a property of `person`, whose value is a small map. This survives any parser, keeps arrays in the per-property map, and rejects a repeated key. It is noisy to write by hand, and nothing in the grammar enforces the `@` convention.

### 3.3 An annotation pattern, written as annotations

The same data, with the annotation syntax.

```gram
@@person-fr :Doc
@language("fr")
@description("Un être humain.")
@aliases(["Personne"])
@name({ term: "nom", description: "Le nom complet de la personne.", aliases: ["anon"] })
(person)
```

The identified annotation gives the `Doc` an identity and a label, and the values can be arrays. It reads as a prefix on the element. Annotation keys cannot be quoted, so there is no `@` convention available, a property called `description` would clash with the `Doc`'s own `description` with no error, and a repeated key goes unnoticed.

### 3.4 The comment is surface syntax for an annotation pattern

The doc comment is authoring syntax, and the model is a `Doc` pattern as in 3.2. Reading a comment produces the pattern: `@language "fr"` applies to the whole comment; the free text is the description; each `@property` tag contributes a per-property entry, with an optional quoted localized term after the key. Writing 3.1's French comment and 3.2's pattern then mean the same thing, and the comment form is the comfortable way to type it. A comment names no identity for the `Doc` it produces, and none is needed: the reading puts the `Doc` inline among the elements of each graph that contains the documented element, where an inline member needs no identity. Identities are not minted. The cost is that the desugaring has to be defined exactly, and that an author can edit the comment and the pattern out of step if both exist in one file.

## 4. Tensions

- **Comfort versus survival as data.** 3.1 is the easiest to write and the only one that vanishes in a plain pattern parse. 3.2 and 3.3 survive and are awkward. 3.4 tries to have both, at the price of a defined desugaring.
- **One namespace for two kinds of key.** In a `Doc` record, the `Doc`'s own fields and the names of the documented element's properties share the key space. The `@` prefix separates them in a record (3.2) and cannot be used in annotation syntax (3.3). Without a convention, a property named `description` or `language` cannot be documented.
- **Addressing a property.** Properties have no identity. A comment names one with a tag (3.1), a record entry keyed by its name addresses it (3.2), and an annotation can do the same only by key (3.3). A separate `Doc` pattern per property would address it uniformly but costs verbosity.
- **How much structure fits per property.** A per-property map can hold scalars and arrays of scalars, so a description, a localized term, a reference, and a list of aliases fit. Anything deeper needs another pattern.
- **Where the language lives.** On each comment or `Doc`, as a qualifying field, so a reader can pick the `Doc`s in the language they want. Beyond `@language` the convention is deliberately left open: a doc comment keeps tags it does not know, and a `Doc` record can carry further `@`-prefixed fields, so an author who needs to tell `Doc`s apart by audience, version, or anything else can evolve their own convention.
- **Where an identity comes from.** A `Doc` the author writes at top level carries one, chosen by the author. A `Doc` produced from a comment carries none and sits inline in each graph that contains the documented element. The open edge: a comment on an element that no graph contains produces a `Doc` in no graph, which is the orphan case.
- **Inline `Doc`s.** An identity is needed only to be named from elsewhere. A `Doc` written inline among a containing graph's elements needs none, so it can stay anonymous there. At the top level, an anonymous `Doc` is a warning, since nothing can name it.
- **Where the rules are checked.** The content of a comment is checked by a comment grammar, and the content of a record by whatever describes records of this kind. 3.4 puts both in play for one piece of meaning.
- **Two spellings, one set of data.** 3.2 and 3.3 hold the same data, and a tool that reads one must decide whether to accept the other.

## 5. Directions Worth Pursuing

3.4 over 3.2 looks like the strongest combination: the model is an annotation pattern in record form, which survives every parser, keeps a duplicate-key check, and has an `@` convention, and the doc comment is the comfortable way to write it. 3.3 is the form with the most open problems. Around that:

- **Identity follows containment.** An ontology file holds many `Graph` patterns, which list their members by identity. A top-level `Doc` the author writes therefore carries an identity, and an anonymous top-level `Doc` is a warning. A `Doc` produced from a comment carries none: it is produced inline in each graph that contains the documented element, so nothing is minted.
- **Elements are shared, and `Doc`s accumulate.** An element can belong to several graphs and carries its meaning into each. A graph can hold any number of `Doc`s for one element (languages, audiences, versions), and nothing overrides anything. Choosing which to read is the reader's job, and an author limits the choice by including only the `Doc`s that apply.
- **The graph is the context.** A graph that holds the French and English `Doc`s of an element already groups them, and a separate audience can have a separate graph over the same elements, so no extra grouping construct is needed.
- **Orphans.** A top-level node, relationship, or annotation not referenced from within any `Graph` is orphaned, and an orphan is a warning.

Underspecified on purpose: beyond `@language`, how `Doc`s are told apart is left to authors, since unknown tags and fields are kept. Not settled: the exact desugaring and how a property is addressed (map entry or separate pattern); a revision of any of them could change which framing leads.

## 6. Disposition

Candidate RFC, not yet written: the shape of a documentation pattern (`Doc`): its fields, the `@` convention, per-property entries, and how a doc comment reads into it. A narrower candidate follows from it, in the gramdoc grammar: `@language`, a quoted localized term after `@property`'s key, and the rule that a comment on a reference documents the referenced element. Items to settle first:

- The exact desugaring from a doc comment to a `Doc` pattern, including what happens when a file holds both.
- Where a comment's `Doc` is produced: inline in each graph that contains the documented element, with nothing minted.
- The orphan rule: a top-level node, relationship, or annotation not referenced from within a `Graph` is a warning, and an anonymous top-level `Doc` is a warning. These are rules for the ontology profile to check (they join the semantic requirements listed in EXP-002), and they apply to every kind of top-level pattern, not only to documentation.
- Property addressing: map entries, or one `Doc` per property.
- `@language` as the one conventional qualifying field. Other qualifiers are left to authors, who can add unknown tags or `@`-prefixed fields.

## 7. References

- The Neo4j ontology-graph-spec: `description`, `aliases`, and `reference` on nodes, relationships, and properties, and a property `name` distinct from its id.
- SKOS and RDFS labels: `rdfs:label`, `rdfs:comment`, `skos:prefLabel`, and `skos:altLabel`, with language tags on the literal. Prior art for a localized term and its alternatives, attached from outside the thing named.
- OWL annotation properties: statements about an entity that carry no logical meaning, attached to the entity from outside it.
