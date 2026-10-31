# Explorations — Divergent Thinking Before an RFC

An Exploration is where a problem space gets looked at fresh, before it has been reduced to one statement worth an [RFC](../rfc/README.md). It is the earliest, most divergent step in the [design process](../README.md): upstream of an RFC's own Discover/Define work, not a stage inside it.

## Why this is a separate phase

An RFC already presumes convergence: "the problem and why it matters, plus high-level candidate approaches" is a single problem statement with a short list of directions to weigh. Getting there sometimes needs a messier prior step: holding several genuinely incompatible framings of a space at once, without picking a winner, and without owing an answer to any existing document.

**An Exploration is deliberately written as if nothing downstream exists.** It does not check whether an RFC already covers its territory, does not organize itself around an existing RFC's taxonomy, and does not owe anything to prior art beyond it being useful raw material (evidence, findings, data — not settled framing). This is what makes it a genuine escape hatch rather than a review step: writing one never creates an obligation to reconcile it with anything else, and it is allowed to rediscover ground an RFC already stands on. If that happens, it is an interesting outcome to note in passing, not a failure to avoid.

An Exploration never requires a reactive trigger ("X is broken, so..."). A hunch, a "what if we looked at this whole space differently," or restlessness with an existing frame is a complete and sufficient reason to write one.

## When to write an Exploration

- A problem space feels worth re-deriving from scratch, whether or not something already exists that touches it.
- Several candidate ways to carve up a space exist and none should be prematurely discarded yet.
- Before committing to an RFC's single problem statement, when the shape of that statement itself is still in question.

## Format

Created and updated via `/exploration-create`. File: `design/exploration/EXP-NNN-short-title.md`, `NNN` zero-padded, a single file. An Exploration is a thinking document, not an experiment with its own code or fixtures.

**Front-matter** (YAML):

| Field | Description |
|---|---|
| `id` | `EXP-NNN` |
| `title` | Short descriptive title |
| `status` | `exploring` \| `concluded` \| `abandoned` |
| `created` | ISO date (`YYYY-MM-DD`) |

No forward-linkage field of any kind. If an RFC is later written from an Exploration's ground, the RFC may cite it informally; the Exploration itself is never edited to record that.

**Sections** (numbered `##`):

1. Starting Point — the question, hunch, or restlessness that prompted this; no demonstrated gap needed.
2. Landscape — the problem space, described on its own terms.
3. Divergent Framings — multiple candidate ways to carve up the space, side by side, allowed to conflict. Numbered `###` subsections per framing.
4. Tensions — where the framings actively disagree, named rather than resolved.
5. Directions Worth Pursuing — which framing(s), or a synthesis, seem worth carrying forward. "No clear direction yet, revisit later" is a complete conclusion.
6. Disposition — what happens next: shelved, one or more RFCs to write, or abandoned. Naming a candidate RFC here does not create one.

Optional appendices follow as further numbered sections (`## 7. References`, `## 8. Candidate Names`), included only when earned. References are external prior art, never this project's own documents.

Use `/exploration-review` when deciding whether a draft is ready to move from `exploring` to `concluded`. It checks for genuine divergence and the absence of downstream-RFC awareness, not for the convergence an RFC review checks for.

## Index

| Exploration | Title | Status |
|---|---|---|
| [EXP-001](EXP-001-structural-ontologies.md) | Structural Ontologies for a Multi-Graph Space | exploring |
| [EXP-002](EXP-002-ontology-tooling-over-the-cst.md) | Ontology Tooling over the gram Syntax Tree | exploring |
| [EXP-003](EXP-003-meaning-as-annotation.md) | Meaning as Annotation | exploring |
