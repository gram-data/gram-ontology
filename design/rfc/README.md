# RFCs — Requests for Comment

RFCs capture **WHAT** a problem is and **WHY** it matters, and the design that answers it: the [design process](../README.md)'s primary document. An RFC carries the design itself, such as the shape of a notation or data model, the rules a tool must check, and examples. Programming-language source code, module or file layout, and the plan for building the design stay out of it and are handled by the speckit skills.

## Audience

Write for a working engineer who has just found this repository. Assume they are technically fluent and comfortable with property graphs and with gram notation as a general idea (patterns, subjects, records, annotations), and assume nothing about this project's specifics: not the `*.ontology.gram` conventions, not the doc-comment tags, not any sibling RFC. Name project artifacts rather than assuming them; one clause on first use is enough.

The failure mode to avoid is a document addressed to its own authors: one that argues with past decisions, defends choices nobody has questioned, and cites sibling documents before naming its subject. `/rfc-review` criterion 8 checks this, and `/rfc-create`'s "Voice and audience" section holds the rules. Section 1 in particular has to teach before it locates: strip every `§`-reference and RFC number out of it, and what remains must still explain the topic to a newcomer.

## When to write an RFC

- A new capability or subsystem is being considered and the problem or scope is not settled yet.
- Multiple genuinely different directions exist and the trade-off is worth recording.

## Format

Created and updated via `/rfc-create`. File: `design/rfc/RFC-NNN-short-title.md`, `NNN` zero-padded.

**Front-matter** (YAML):

| Field | Description |
|---|---|
| `id` | `RFC-NNN` |
| `title` | Short descriptive title |
| `status` | `draft` \| `review` \| `accepted` \| `superseded` |
| `created` | ISO date (`YYYY-MM-DD`) |

**Required sections** (numbered `##`, renumber if one is omitted):

1. Summary
2. Problem / Motivation
3. Goals
4. Non-Goals
5. Proposed Approach, split into numbered `###` subsections when more than one distinct axis or strategy is explored
6. Alternatives Considered
7. Open Questions, each individually numbered (`7.1.`, `7.2.`, ...), never a plain bullet list, so other documents can cite exactly which question they address

Optional appendices may follow as further numbered sections.

Use `/rfc-review` before moving a draft to `review` or `accepted`.

## Index

| RFC | Title | Status |
|---|---|---|
| [RFC-001](RFC-001-ontology-profile.md) | An Ontology Profile for gram | draft |
