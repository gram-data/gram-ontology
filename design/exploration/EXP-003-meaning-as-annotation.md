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

**What meaning needs.** From the Neo4j ontology-graph-spec and from two spike files in this repository (`examples/pattern-rs-spike/`): a `description`, `aliases`, and a `reference`, on a node, a relationship, and a property. Properties have no identity of their own, so documenting one means naming it by key. The spec also has a property `name`, the property's name in the database when it differs from its id, which is a different thing from a localized word. One spike file (`movies.ontology.gram`) keeps meaning out of the elements, as separate `Meaning` patterns attached with `@on(...)` and grouped in a `Glossary`; the other (`person.ontology.gram`) documents a node in a comment and adds a second pattern for the French name. A language, or an audience, is a context in which a whole group of meanings applies. That spike also gathers its patterns into a containing pattern that names its members by identity, `[movies::Ontology {...} | person, movie, acted-in]`, so a pattern that is to belong to a containing graph needs an identity to be named by. An ontology file holds many such graphs, each an explicit pattern, because the alternative for a multi-graph ontology is references from one file to another.

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

The doc comment is authoring syntax, and the model is a `Doc` pattern as in 3.2. Reading a comment produces the pattern: `@language "fr"` applies to the whole comment; the free text is the description; each `@property` tag contributes a per-property entry, with an optional quoted localized term after the key. Writing 3.1's French comment and 3.2's pattern then mean the same thing, and the comment form is the comfortable way to type it. A comment names no identity for the `Doc` it produces, so the reading has to mint one. The cost is that the desugaring has to be defined exactly, and that an author can edit the comment and the pattern out of step if both exist in one file.

### 3.5 Meaning as a layer: glossaries

Meaning is kept in a layer apart from the elements, and groups of meanings form contexts. An element knows nothing of its documentation, and a glossary names the `Doc` patterns that apply together.

```gram
[person-fr:Doc {
  `@language`: "fr",
  `@description`: "Un être humain."
} | (person)]

[french:Glossary {`@language`: "fr"} | person-fr]
```

A language or audience is then one glossary, and swapping or adding one edits nothing else. The `Doc` needs an identity so that the glossary can name it, and the language is stated on the glossary, on each `Doc`, or both.

## 4. Tensions

- **Comfort versus survival as data.** 3.1 is the easiest to write and the only one that vanishes in a plain pattern parse. 3.2 and 3.3 survive and are awkward. 3.4 tries to have both, at the price of a defined desugaring.
- **One namespace for two kinds of key.** In a `Doc` record, the `Doc`'s own fields and the names of the documented element's properties share the key space. The `@` prefix separates them in a record (3.2) and cannot be used in annotation syntax (3.3). Without a convention, a property named `description` or `language` cannot be documented.
- **Addressing a property.** Properties have no identity. A comment names one with a tag (3.1), a record entry keyed by its name addresses it (3.2), and an annotation can do the same only by key (3.3). A separate `Doc` pattern per property would address it uniformly but costs verbosity.
- **How much structure fits per property.** A per-property map can hold scalars and arrays of scalars, so a description, a localized term, a reference, and a list of aliases fit. Anything deeper needs another pattern.
- **Where the language lives.** On each comment or `Doc` (3.1, 3.2, 3.3), or on a context that groups them (3.5). The first is local and repeats; the second adds a layer and an identity.
- **Where an identity comes from.** A top-level `Doc` carries one (see section 5). When the author writes the `Doc` pattern, the author chooses it. When a doc comment produces the `Doc` (3.4), something has to mint it, and a minted identity has to be stable and unique within the file or two comments would collide.
- **Inline `Doc`s.** An identity is needed only to be named from elsewhere. A `Doc` written inline among a containing graph's elements needs none, so it can stay anonymous there. At the top level, an anonymous `Doc` is a warning, since nothing can name it.
- **Belonging to more than one graph.** With many graphs in a file, one element can plausibly be a member of several. Whether it may is open; the orphan rule below needs only that it belong to at least one.
- **Where the rules are checked.** The content of a comment is checked by a comment grammar, and the content of a record by whatever describes records of this kind. 3.4 puts both in play for one piece of meaning.
- **Two spellings, one set of data.** 3.2 and 3.3 hold the same data, and a tool that reads one must decide whether to accept the other.

## 5. Directions Worth Pursuing

3.4 over 3.2 looks like the strongest combination: the model is an annotation pattern in record form, which survives every parser, keeps a duplicate-key check, and has an `@` convention, and the doc comment is the comfortable way to write it. 3.5's layer can be added later without changing the `Doc` pattern, since a glossary only names `Doc` patterns. 3.3 is the form with the most open problems. A top-level `Doc` carries an identity. An ontology file holds many `Graph` patterns, which list their members by identity, and a `Doc` without one could only appear inline among a graph's elements. An anonymous top-level `Doc` is a warning. The same containment gives a general rule: a top-level node, relationship, or annotation that is not referenced from within any `Graph` is orphaned, and an orphan is a warning. This favors 3.2, 3.3, and 3.5, where the `Doc` is a pattern with an identity, and makes the identity of a comment-produced `Doc` something 3.4 has to solve. Not settled: the exact desugaring, how an identity is minted for a comment, and how a property is addressed (map entry or separate pattern); a revision of any of them could change which framing leads.

## 6. Disposition

Candidate RFC, not yet written: the shape of a documentation pattern (`Doc`): its fields, the `@` convention, per-property entries, and how a doc comment reads into it. A narrower candidate follows from it, in the gramdoc grammar: `@language`, a quoted localized term after `@property`'s key, and the rule that a comment on a reference documents the referenced element. Items to settle first:

- The exact desugaring from a doc comment to a `Doc` pattern, including what happens when a file holds both.
- How a `Doc`'s identity is chosen when a comment produces it.
- The orphan rule: a top-level node, relationship, or annotation not referenced from within a `Graph` is a warning, and an anonymous top-level `Doc` is a warning. These are rules for the ontology profile to check, and they apply to every kind of top-level pattern, not only to documentation.
- Whether one element may belong to several graphs in a file.
- Property addressing: map entries, or one `Doc` per property.
- Whether the language belongs on each `Doc`, on a glossary, or either.

## 7. References

- The Neo4j ontology-graph-spec: `description`, `aliases`, and `reference` on nodes, relationships, and properties, and a property `name` distinct from its id.
- SKOS and RDFS labels: `rdfs:label`, `rdfs:comment`, `skos:prefLabel`, and `skos:altLabel`, with language tags on the literal. Prior art for a localized term and its alternatives, attached from outside the thing named.
- OWL annotation properties: statements about an entity that carry no logical meaning, attached to the entity from outside it.
