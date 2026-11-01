# Design process

gram-ontology's design is worked out in the open, in documents, before and alongside code. The process is borrowed from another project of the same author (an Exploration, then an RFC, with more layers available when they are needed) and is deliberately light.

| Document | Question it answers | Where |
|---|---|---|
| **Exploration** (`EXP-NNN`) | What are the several ways to look at this space? Divergent on purpose; owes nothing to any existing document. | [`exploration/`](exploration/README.md) |
| **RFC** (`RFC-NNN`) | What is the problem, why does it matter, and what design answers it? The design itself (notation, rules, checks), not source code or a build plan. | [`rfc/`](rfc/README.md) |

Later layers (a concrete elaboration of one candidate framing, an architecture decision record, time-boxed spikes) can be added the same way when there is something for them to hold. Each is a directory under `design/` with its own README and index.

The current Explorations are [EXP-001, structural ontologies for a multi-graph space](exploration/EXP-001-structural-ontologies.md), [EXP-002, ontology tooling over the gram syntax tree](exploration/EXP-002-ontology-tooling-over-the-cst.md), [EXP-003, meaning and structure as annotation](exploration/EXP-003-meaning-as-annotation.md), and [EXP-004, labels in an ontology](exploration/EXP-004-labels.md).

## Tooling

The four skills in `.claude/skills/` create and review these documents:

- `/exploration-create`, `/exploration-review`
- `/rfc-create`, `/rfc-review`

They keep each directory's index table in sync, so do not hand-edit those tables. Create documents on a branch, not on `main`, so design work goes through the same review as code.

## Where to start

The model comes first: the shape of a valid `*.ontology.gram` file is the source of truth, and JSON is only one serialization of it. `kind` will eventually be associated with a gram schema, and this project is one of the hand-built kinds that schema is distilled from. EXP-001 starts from structural ideas (spans, layering informal semantics before tightening boundaries) that bear on that shape, and EXP-002 covers where the tooling sits. An RFC on the shape itself is the natural next document.
