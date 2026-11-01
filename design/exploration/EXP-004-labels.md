---
id: EXP-004
title: Labels in an Ontology
status: exploring
created: 2026-11-01
---

# EXP-004: Labels in an Ontology

## 1. Starting Point

A node in a graph can carry several labels, and an ontology has to say which of them matter and how. The Neo4j ontology-graph-spec gives a node three kinds of label: one that identifies the node, labels every node of the kind also has, and labels that only some nodes have. So far the ontology work in this repository covers the identifying label and nothing else. The question here is what gram notation should do about the other two, and about documenting a label, so that a node definition says everything the spec's labels say.

## 2. Landscape

**What the spec says.** A node has exactly one identifying label (`labels.identifier`, with a `label` shorthand). It can also have `implied` labels, which every node of the kind has and which express a hierarchy (an `Actor` that implies `Person` says every actor is a person), and `optional` labels, which only some nodes have, such as a role or a state (`Person` nodes may also be `Director` nodes). Only the identifying label, like a relationship's type, is enforced by the database; `implied` and `optional` are descriptive and nothing infers over them. A relationship end names the node it attaches to by id and may repeat its label. A description, aliases and a reference belong to the node, not to a single label. Importing RDF flattens `rdfs:subClassOf` into `implied` labels.

**What gram says.** A node is written `(identifier:Label:Other)`. In plain gram the identifier names the node and each label is a separate matter. The grammar accepts `:` and `::` as the same label separator, so `::` carries no meaning and `(Actor::Person)` is a node identified as `Actor` with one label, `Person`. A label that is not a plain symbol is written in backticks, `` `Director?` ``. gram has no way to mark one label as different from another.

**The ontology convention.** In an ontology file the identifier implies an identifying label of the same name, so the name is not written twice. `Actor` is then both a gram identifier and the identifying label. `(Actor::Person)` reads as `(Actor::Actor:Person)`: identifying label `Actor`, implied label `Person`. The double colon is a stylistic choice for ontology files, in the same way that `=>` is preferred for relationships, because it makes the difference from plain gram visible.

```gram
(Actor::Person)
```

with an optional label added:

```gram
(Actor::Person:`Director?`)
```

**Labels that are also definitions.** `Person` can appear as an implied label of `Actor` and be defined as a node of its own elsewhere. There is one `Person`, so the label is a reference to that definition and not a second one. A reference to an element that is defined elsewhere in the file is the ordinary meaning of a bare name in gram.

**Where a relationship attaches.** In a pattern a relationship's ends are the elements it lists, which are references to node definitions. A separate label on the end would repeat what the reference already says.

**Documenting.** A gramdoc comment can already document a label, through an `@label name text` tag.

## 3. Divergent Framings

### 3.1 Labels written as labels

The labels stay labels, in the notation gram already has. The identifier carries the identifying label, labels after the `::` are implied, and a label that may be absent is marked.

```gram
(Actor::Person:`Director?`)
```

Nothing new is needed in gram for the implied labels, since `(Actor::Person)` parses today and only its reading changes in an ontology file. The marker for an optional label is the open part, see 3.4. Everything about the node is in one place, and a reader who knows Cypher recognizes it. The cost is that a single construct carries three kinds of label by position and by a suffix, and an optional marker spelled inside backticks is a convention rather than syntax.

### 3.2 Implied and optional labels as wrapping annotations

The node definition stays minimal, and what the labels mean is written as annotations that wrap it, in the same tier as cardinality.

```gram
@implies(["Person"])
@optional(["Director"])
(Actor)
```

The annotations are anonymous and intrinsic, and an array holds as many labels as are needed. A tool that reads the node reads its annotations. The cost is that the labels are no longer on the node: a reader looking at `(Actor)` sees no labels, and the generic tools that show labels show none.

### 3.3 Implication as a pattern between definitions

A label is itself a kind of thing, so a hierarchy is a relationship between definitions and not a list on one of them.

```gram
(Actor)=[`@implies`]=>(Person)
(Person)=[`@optional`]=>(Director)
```

The collision between a label and a definition disappears, because both ends are definitions. Documentation attaches to `Person` once and applies wherever it is used. The relationship types here are the ontology's own, and a plain name such as `IMPLIES` would take it from the domain, which may well have a relationship of that name, so they need a spelling a domain cannot use, such as the `@`-prefixed backtick-quoted type shown, as `@` already marks the ontology's own keys in a `Doc`. The cost is that a node no longer lists its labels, a tool must walk the graph to find them, the spec's flat `implied` list is spread across relationships, and an ontology of many labels reads as a graph of labels before it reads as a model.

### 3.4 How an optional label is marked

Inside 3.1 there are several candidate marks, which also reach beyond the ontology.

- **A backtick-quoted name that ends in `?`.** `` `Director?` `` works today, in any parser, with no change to gram. A tool must recognize the suffix inside a quoted name, and the name as the graph sees it is `Director?`.
- **A terminating `?` on the label.** `Director?`, in gram itself. Plain gram would treat it as decorative. It needs a grammar change, with a corpus test and a release.
- **A terminating `!` for the required case.** Labels are required by default, so `!` would be redundant, and it already means required in a property type such as `"STRING!"`, which would give one mark two meanings in a file.
- **Cypher's label expression separators**, extended with `?`. Those separators are query syntax: `|` means either label and a leading `!` means not this label. They describe what to match, so in a definition they have no meaning, and plain gram could not safely treat them as decorative.
- **A wrapping annotation** `@optional([...])`, as in 3.2.

The identifying label cannot be optional, since a node has exactly one, so whatever the mark is, it is not allowed on the identifier.

### 3.5 Where a label's documentation lives

- **A comment tag.** `@label Person A human being.` in a doc comment, which gramdoc parses today.
- **On the label's own definition.** Because an implied label is a reference to a defined node, the node's own `Doc` documents it, and there is no separate label documentation.
- **An entry in the documented node's `Doc`**, keyed by label as properties are, for the case where a label has no definition of its own.

Which of these is needed, and whether `@label` desugars to anything, is not yet known and wants an experiment.

## 4. Tensions

- **Labels on the node, or beside it.** 3.1 keeps everything in one pattern and tools that show labels keep working. 3.2 and 3.3 keep the node clean and move meaning elsewhere, at the price of a node that does not show its own labels.
- **A convention of position versus a mark.** In 3.1 the identifying and implied labels are told apart only by the ontology reading of the identifier and the `::`. The same text in plain gram means something else, so a tool must know it is reading an ontology file.
- **A hierarchy as a list or as a graph.** 3.1 and 3.2 follow the spec's flat list. 3.3 follows the structure the labels describe, and stays closer to RDF's subclass relation, but it is no longer a node's own field.
- **Cost of a grammar change.** A terminating `?` is the cleaner mark and the only one that needs a published grammar to change. The backtick form costs nothing now and is easy to find and replace later.
- **Reference or definition.** An implied label that matches a definition is a reference. One that matches none is either an undefined label to warn about, or a label that is never defined because nothing needs it.
- **The ontology's names against the domain's.** 3.3 puts relationship types of the ontology's own among the domain's, so they need a reserved spelling. 3.1 and 3.2 avoid this, since labels and annotations live in positions a domain's names do not occupy.
- **Descriptive only.** Neither implied nor optional labels are enforced by the database, so a model that says more about them than a graph can check is a statement of intent, and tools must not claim otherwise.

## 5. Directions Worth Pursuing

3.1 looks right as the primary form: it uses the notation as it is, needs nothing new for implied labels, and reads like the data it describes. The `::` is an ontology-file convention, and the identifier implies the identifying label. For optional labels, use the backtick form with a trailing `?` for now, and treat a terminating `?` as the intended later change to gram, with plain gram reading it as decorative and the ontology profile giving it meaning. The marker is not allowed on the identifier. A label that matches a definition is a reference to it. The relationship end carries no separate label.

3.3 is worth keeping in view for tools that want to traverse a hierarchy, and nothing prevents deriving it from 3.1. 3.2 stays available for anything that is intrinsic but is not a label.

## 6. Disposition

Candidate RFC: a section on labels in the ontology profile, or an addition to the profile's foundational RFC, covering the `::` reading, implied and optional labels, the optional marker, and the collision rule. The tree-sitter-gram change for a terminating `?` would be its own, smaller change, to be made when the marker is settled.

Settled:

- **Documenting a label:** a label usually has a defining node elsewhere with the full description, but any label may be documented where it is used, to give context there. How a comment's `@label` reads into a `Doc` is not decided; `person.ontology.gram` shows the comment form.
- **An implied label with no definition:** a warning.
- **The optional marker:** the backtick form, `` `Director?` ``, for now. A grammar change for a terminating `?` is considered only if review pushes back.
- **Reading the backtick form:** every other tool sees the `?` as part of the label, which is acceptable. In an ontology file a label with a trailing `?` is always optional, so a label genuinely named that way is not allowed.
- **The `label` shorthand:** not needed, since the identifier already names the identifying label.

## 7. References

- Neo4j ontology-graph-spec: node labels (`identifier`, `implied`, `optional`), the `label` shorthand, relationship ends, and the RDFS mapping of `rdfs:subClassOf` to implied labels.
- Cypher label expressions: the `&`, `|`, `!` and `%` operators for matching labels.
