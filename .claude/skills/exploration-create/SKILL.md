---
name: "exploration-create"
description: "Create or update an Exploration: divergent thinking about a problem space, before it has been reduced to one RFC-worthy statement — the escape-hatch phase that precedes RFC's Discover/Define work."
argument-hint: "Describe the question, hunch, or space to explore, or 'update EXP-001 ...' to revise an existing one"
compatibility: "Requires design/exploration/ directory (created automatically on first use)"
metadata:
  author: "gram-ontology"
user-invocable: true
disable-model-invocation: false
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). If empty, ask the user what question, hunch, or space they want to explore — make clear no demonstrated gap or existing pain point is required to start one.

## Purpose

An Exploration is divergent thinking about a problem space before it has been reduced to one RFC-worthy statement. It holds several possibly-incompatible framings of that space at once, on purpose, without picking a winner. Unlike an RFC (see `rfc-create`), it needs no reactive trigger — a hunch or a "what if we looked at this differently" is a complete and sufficient reason to write one — and it carries **no obligation to check what already exists downstream**. It is deliberately written as if no RFC or ADR touching this ground exists yet, even when one does. That's what makes it a genuine escape hatch rather than a review step: writing one never creates debt to reconcile with anything else, and it is allowed to rediscover ground an RFC already stands on.

This does not mean ignoring real evidence. Citing a spike's findings, a piece of code, a data file, or an observed result is exactly the kind of raw material an Exploration should draw on — that's data, not a design decision. What it must not do is organize itself around, defer to, or frame itself as reconciling an existing RFC or ADR's stated problem, taxonomy, or decision.

## Voice and audience

Same discipline as `rfc-create`'s "Voice and audience" section — write for a newcomer, teach before locating, keep cross-references non-load-bearing — with two differences specific to this document type:

1. **Motivation may be a hunch, not a cost.** RFC's "motivation is evidence, not confession" rule assumed a demonstrated problem exists; here the Starting Point can legitimately be "this feels worth re-deriving from scratch" — but it must still be concrete enough to actually explore, not a vague mood ("things feel messy lately").
2. **Never adopt a sibling document's vocabulary or structure as inherited.** If an Exploration's own reasoning happens to land on the same terms an existing RFC/ADR already uses, that's a coincidence worth noting in passing (per Disposition), never a premise to build on. A sentence that only parses with knowledge of a specific sibling RFC's structure has smuggled in the exact awareness this format exists to avoid.

## Outline

1. **Require a non-main branch before making any changes**:
   - Run `git branch --show-current` (skip this whole check silently if not inside a git
     repository).
   - Determine the repository's main branch: prefer the short name from
     `git symbolic-ref refs/remotes/origin/HEAD`; fall back to `main` if that's unavailable.
   - If the current branch **is** the main branch, **STOP** — do not create or edit any file.
     Tell the user: Explorations should be authored on a feature branch, not directly on the main
     branch, so this design work goes through the same PR-based review as everything else. Ask
     them to create/switch to a branch (e.g. `git checkout -b <short-descriptive-name>`) and
     re-invoke `/exploration-create`.
   - Otherwise, proceed — this applies to both create and update.

2. **Determine create vs. update**:
   - Scan `$ARGUMENTS` for an `EXP-<N>` reference (bare like `EXP-1`, zero-padded like `EXP-001`, a bare filename, or an `@design/exploration/EXP-001-*.md` path).
   - If found, resolve it against files in `design/exploration/` (normalize the numeric part; ignore leading zeros when matching). If it resolves, this is an **update** — read the existing file and apply the requested changes, preserving the `id` and `created` front-matter fields unless the user explicitly asks to change them.
   - If an `EXP-<N>` token is present but does not resolve to any file, tell the user and stop — do not silently create a new Exploration under a guessed number.
   - If no `EXP-<N>` token is present at all, this is a **create**.

3. **For create**: determine the next sequential number.
   - `mkdir -p design/exploration` if it doesn't exist.
   - Scan `design/exploration/` for files matching `EXP-(\d+)-*.md`, take the max numeric value, add 1, zero-pad to 3 digits.
   - Generate a concise short name (2-4 words, kebab-case) from the question/hunch, same conventions as `rfc-create`'s short-name generation.
   - Target file: `design/exploration/EXP-<NNN>-<short-name>.md`.

4. **Write the Exploration** with this structure (omit a section entirely rather than leaving it as a placeholder if it doesn't apply):

   ```markdown
   ---
   id: EXP-<NNN>
   title: <Title>
   status: exploring
   created: <DATE>
   ---

   # EXP-<NNN>: <Title>

   ## 1. Starting Point

   The question, hunch, or restlessness that prompted this — needs no demonstrated gap or
   reactive justification, but must be concrete enough to actually explore.

   ## 2. Landscape

   The problem space, described on its own terms — never as a reframing of an existing document.

   ## 3. Divergent Framings

   Multiple candidate ways to carve up the space, presented side by side, explicitly allowed to
   conflict with each other. Use numbered `###` subsections per framing (`### 3.1 Framing A — ...`,
   `### 3.2 Framing B — ...`) once there is more than one.

   ## 4. Tensions

   Where the framings in §3 actively disagree — named, not resolved. An unresolved tension,
   honestly reported, is a correct outcome here, not an unfinished one.

   ## 5. Directions Worth Pursuing

   Which framing(s), or a synthesis, seem worth carrying forward. "No clear direction yet, revisit
   later" is a complete and legitimate conclusion.

   ## 6. Disposition

   What happens next: shelved, one or more RFCs worth writing, or abandoned. Naming a candidate
   RFC here does not create one — that stays a separate, later, explicitly-chosen step. If this
   exploration's reasoning happened to land near an existing RFC/ADR's ground, note that here as
   an observation, not as something the document set out to reconcile.
   ```

   Number every top-level `##` section sequentially. `###` subsections under Divergent Framings are required once more than one framing exists, so each can be cited unambiguously from Tensions or Directions Worth Pursuing.

   Any number of optional appendices may follow Disposition as additional numbered sections
   (`## 7. References`, `## 8. Candidate Names`, etc.) — same convention as RFC's format. Two
   recognized so far:
   - **References** — external prior art, literature, or benchmarks the exploration draws on.
     Never a citation of this project's own RFC/ADR/Spike documents; that's exactly the
     downstream awareness this format exists to avoid.
   - **Candidate Names** — naming ideas held the same way Disposition holds a candidate RFC:
     listed, not committed to.
   Include an appendix only when it's genuinely earned; omit rather than force one.

5. **For update**: apply the requested edits directly to the existing sections. Common updates:
   - Status transitions: `exploring` → `concluded` or `exploring` → `abandoned`. There is no `review`/`accepted` stage — an Exploration doesn't get "approved," it either concludes with something worth carrying forward or doesn't.
   - Adding a new Divergent Framing, or a newly-surfaced Tension, as thinking continues.
   - Do **not** add a forward-linkage field (`rfcs:`, `spawned:`, or similar) to the front-matter — by design, an Exploration carries no such obligation, even after an RFC is written from it.

6. **Maintain the index**: `design/exploration/README.md` holds a living index table (`Exploration | Title | Status`).
   - If `design/exploration/README.md` doesn't exist yet, create it (format doc + an empty index table, matching `design/rfc/README.md`'s style) before continuing.
   - Create: append a new row.
   - Update: refresh this Exploration's existing row (title/status may have changed) in place — don't duplicate or reorder other rows.

7. **Guardrail — reject downstream awareness, not implementation detail**: unlike `rfc-create`'s guardrail (which rejects implementation detail), this skill's guardrail is the opposite direction. If the draft cites an existing RFC or ADR by id as the organizing frame, or describes itself as "reframing," "consolidating," or "resolving" one, strip that framing — restate the same point on its own terms, without deferring to the sibling document's authority. Concrete project evidence (spike results, code, data files, observed data) is always fine to cite; a sibling design decision as the premise is not.

## Completion Report

Report:
- The Exploration file path (created or updated).
- Its `status`.
- If newly created and it reached a Disposition naming a candidate RFC: mention `/rfc-create` as the separate, later, explicitly-chosen next step — never imply this skill call already took it.
