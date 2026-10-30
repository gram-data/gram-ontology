---
name: "exploration-review"
description: "Review a draft Exploration for genuine divergence, absence of premature convergence or downstream-RFC awareness, and readiness to conclude. Produces concrete suggested revisions, not just a report."
argument-hint: "Reference the Exploration to review (EXP-NNN, filename, or @design/exploration/EXP-NNN-*.md path)"
compatibility: "Requires design/exploration/ (see design/exploration/README.md for format)"
metadata:
  author: "gram-ontology"
user-invocable: true
disable-model-invocation: false
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding. If it's empty, or doesn't resolve to an existing Exploration, list the Explorations in `design/exploration/` (see its index in `design/exploration/README.md`) and ask which one to review.

## Purpose

Evaluate a draft Exploration against a consistent set of quality criteria — but the bar is different from an RFC's. An RFC review checks for convergence: one well-scoped problem, real alternatives, no implementation detail. An Exploration review checks the opposite failure modes: has it actually diverged, or does it just restate one idea under several labels? Has it stayed free of the exact downstream awareness `exploration-create`'s guardrail exists to prevent? Is it honestly unresolved where it should be, rather than prematurely tidy? This decides whether the document is ready to move from `exploring` to `concluded` (or should instead be `abandoned`, or simply isn't done yet) — it doesn't replace `exploration-create`, it decides whether what it produced is actually doing the job this document type exists for.

## Workflow

1. Resolve the Exploration reference against `design/exploration/` (same bare/zero-padded/filename/`@`-path resolution as `rfc-create`/`exploration-create`).
2. Read the Exploration and `design/exploration/README.md` to confirm structural compliance (numbered sections present in order, front-matter fields populated, no borrowed forward-linkage field).
3. Evaluate the Exploration against each criterion below.
4. For each criterion that fails or is weak, draft a specific suggested revision — quote the problematic text and propose replacement text or a concrete addition.
5. Summarize findings: what's strong, what needs revision, and what's blocking vs. advisory.

Do not edit the Exploration yourself — report findings and suggested revisions; let the user (or a follow-up `exploration-create` call) apply them.

## Review Criteria

### 1. Genuine divergence, not restated unity

Are the Divergent Framings (§3) actually different perspectives that would change what gets measured or built, or is the same idea relabeled under two or three headings?

**Flag:** Two framings are trivially compatible or one is just a narrower case of the other, presented as if competing. Test: "would choosing Framing A over Framing B actually change a downstream decision?" If not, they aren't genuinely divergent — either merge them or find the framing that actually contests the first.
**Flag:** Only one framing is present at all, with no acknowledgment that this is a deliberate choice (some spaces may genuinely have only one live framing) versus an oversight.

### 2. No demonstrated-pain requirement smuggled in

**Flag:** Starting Point (§1) reads like an RFC's Problem/Motivation section — justifying itself against an existing gap or cost ("X is broken, so...") — rather than standing on its own as a question or hunch. This is RFC-shaped convergence pressure leaking into a document type that doesn't need it; rewrite as a self-sufficient starting question.

### 3. No downstream awareness — the central guardrail unique to this document type

**Flag:** The document cites an existing RFC or ADR by id as its organizing frame, or explicitly describes itself as "reframing," "consolidating," or "resolving" one — even when phrased carefully. Distinguish this from legitimate use of concrete project evidence (spike results, code, data files, observed numbers), which is always fine to cite.
**Test:** could this document be dropped into an unrelated repository with the same underlying data/code and still make sense entirely on its own terms? If a sentence only parses with knowledge of a specific sibling RFC's structure or vocabulary, that coupling should be stripped — restate the same point without the sibling document as premise.
**Flag:** An optional References appendix cites this project's own RFC/ADR/Spike documents — that appendix exists only for external prior art per `design/exploration/README.md`; an internal citation there is the same violation as anywhere else in the document, just relocated.
**Flag:** An optional appendix beyond References or Candidate Names appears without being genuinely earned — per `design/exploration/README.md`, an appendix is included only when it holds real content, not added by default.

### 4. Tensions are preserved, not resolved away

**Flag:** §4 Tensions is empty, or every listed tension is immediately "resolved" within the same section. A Tension that survives to the end of the document, honestly stated, is the correct and even desirable outcome of an Exploration — a document that resolves everything has probably converged prematurely into something that should just be an RFC instead (suggest `/rfc-create` in that case, and note that this Exploration may be ready for `concluded` sooner than expected).
**Do not flag:** a genuine methodological or measurement-protocol convention settled explicitly, with its own stated reasoning, as safe to fix precisely because it's orthogonal to the domain framings in §3 — e.g. "this exploration only considers files, not live graphs" says nothing about which framing wins or whether findings generalize across the problem space; it only fixes the unit any answer would need anyway. Distinguish this from a domain-level tension resolved for convenience: the test is whether settling it forecloses any of the substantive questions in §3/§4, or only fixes a unit/convention that any answer to those questions would need anyway.

### 5. Directions Worth Pursuing aren't a foregone conclusion

**Flag:** §5 reads as if the "right answer" was predetermined and the framings in §3 were decorative alternatives nobody could seriously have chosen (mirrors `rfc-review`'s "alternatives are strawmen" criterion, applied to §3's framings instead of a separate Alternatives section).
**Flag:** "No clear direction yet, revisit later" is treated as a failure requiring more work before review, when per `design/exploration/README.md` it's a complete, legitimate conclusion in its own right — don't push the document toward manufacturing a direction it hasn't earned.

### 6. Disposition doesn't overcommit

**Flag:** §6 Disposition asserts an RFC *will* be written, or begins drafting one inline, rather than naming it as a candidate, optional, later step someone still has to explicitly choose to take.
**Flag:** Front-matter or Disposition adds a forward-linkage field (`spawned:`, `informs:`, or similar) — by design, an Exploration carries no such obligation, even once an RFC is later written from it. Flag any hand-added field beyond `id`/`title`/`status`/`created`.

### 7. Structural compliance

**Flag:** Front-matter is missing a required field, or carries one that doesn't belong (see criterion 6).
**Flag:** Numbered `##` sections are missing, out of order, or a present-but-inapplicable section wasn't omitted per the template's own instruction.
**Flag:** More than one framing exists in §3 but isn't split into numbered `###` subsections, making it impossible for §4/§5 to cite a specific one unambiguously.
**Flag:** `status` isn't one of `exploring` / `concluded` / `abandoned`.

### 8. Voice and audience

Same checks as `rfc-review`'s criterion 8 (grammatical subject is the topic, not the project or document; meta-talk is minimal; cross-references are non-load-bearing; unintroduced terms don't stall a newcomer) — with one adjustment: do **not** flag a hunch-based Starting Point as unmotivated confession the way `rfc-review` would flag a weakly-evidenced RFC Problem section. A hunch is legitimate here; flag it only if it's so vague it gives §2/§3 nothing concrete to work from.

Cheap mechanical pre-pass, same idea as `rfc-review`'s, adapted to this directory:

```bash
printf "%-56s %6s %6s %6s %6s %6s\n" FILE lines rthr notX self proj
for f in design/exploration/EXP-*.md; do
  printf "%-56s %6s %6s %6s %6s %6s\n" "$f" \
    "$(grep -c . "$f")" \
    "$(grep -o "rather than" "$f" | wc -l | tr -d ' ')" \
    "$(grep -oE ", not [a-z]" "$f" | wc -l | tr -d ' ')" \
    "$(grep -oiE "this (RFC|ADR|EXP|exploration|subsection|section|document)" "$f" | wc -l | tr -d ' ')" \
    "$(grep -oiE "th(is|e) project('s)?" "$f" | wc -l | tr -d ' ')"
done
```

A document that's an outlier on several columns at once, normalized for length, is drifting toward insider voice or downstream awareness. Judge the prose, not the counts.

## Output Format

```
## Exploration Review: [EXP-NNN title]

### Summary
[2-3 sentences: overall assessment and whether it's ready for `concluded` status, should be
`abandoned`, or genuinely isn't done exploring yet.]

### Blocking Issues
[Must be resolved first. For each: criterion, problem (quote), suggested revision.]

### Advisory Issues
[Worth addressing but not blocking. Same format.]

### Strengths
[What's working well and should be preserved — an honestly unresolved Tension or a legitimately
open Direction counts as a strength here, not a gap.]
```
