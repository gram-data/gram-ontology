# Feature Specification: `gram-ontology check` covers every RFC-001 §5.6 check

**Feature Branch**: `001-ontology-check-rfc-001`

**Created**: 2026-11-02

**Status**: Draft

**Input**: User description: "Make `gram-ontology check` make every check listed in RFC-001 section 5.6 (An Ontology Profile for gram), so that an author running it on a `*.ontology.gram` file is told about every mistake the profile says a conforming tool must report."

**Design authority**: `design/rfc/RFC-001-ontology-profile.md` (draft), sections 5.1, 5.4, 5.5 and 5.6. Where EXP-003 or EXP-004 differ from the RFC, the RFC wins. This spec says what the command reports and why; it does not say how.

## Background

Today `gram-ontology check` reports only: syntax errors (as `parse-error`), the opt-in mismatch between file name and `kind`, and malformed doc comments. Everything else in §5.6 is accepted silently, so a mistyped property name, a repeated record key, an orphaned pattern or a dangling label passes as clean. The profile exists precisely so that these are caught (RFC §6, "Conventions without a profile").

## Terms

Used as RFC-001 uses them. In short: a *definition* is a top-level node or relationship that declares an element; its *identifier* is the name used to refer to it (by default its identifying label or type, or an explicit `identifier:Label`); the *identifying label* is a node's first label, after `::` shorthand is expanded; *implied labels* are the later labels; a *`Doc`* is a pattern carrying meaning for an element; an *annotation* wraps a definition (`@cardinality`, `@unique`, …); a *graph* is a `Graph` pattern listing members by identity.

## Clarifications

### Session 2026-11-02

- Q: Should the checks whose severity the RFC doesn't state be errors or warnings? → A: Errors (all nine of them, excluding `duplicate-key`, which follows `gram lint`).
- Q: Should `duplicate-doc` (two indistinguishable `Doc`s) be a warning or an error? → A: Warning.
- Q: Should an author be able to change a check's severity in this first version? → A: No; severities are fixed per check, and configuration is a separate future feature.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Catch mistakes in how elements are identified and referenced (Priority: P1)

An ontology author edits a `*.ontology.gram` file and runs the check. They want to be told when two definitions claim the same identifier, when a pattern no graph refers to is silently dead, and when a `Doc` is anonymous, because these make the model ambiguous or incomplete without any syntax error.

**Why this priority**: Identity is the foundation of the profile (§5.1). Every other check (documented properties, labels, cardinality) assumes identifiers are unique and referenced. Duplicate identifiers and orphans are also the most common authoring slips.

**Independent Test**: Run the check on small files each containing one identity mistake; each produces exactly the expected diagnostic, and a file with none produces none.

**Acceptance Scenarios**:

1. **Given** two definitions with the same identifier (for instance `(Person)=[OWNS]=>(Car)` and `(Company)=[OWNS]=>(Car)`, both identified `OWNS`), **When** the author runs the check, **Then** a `duplicate-identifier` diagnostic is reported for the later definition, naming the identifier and pointing at the earlier definition's line.
2. **Given** the same two relationships with explicit identifiers `ownsCar` and `ownsFleet`, **When** the check runs, **Then** no `duplicate-identifier` diagnostic is reported.
3. **Given** a top-level node, relationship or annotation that no `Graph` pattern lists, **When** the check runs, **Then** an `orphaned-pattern` **warning** is reported at that pattern.
4. **Given** a top-level `Doc` with no identity, **When** the check runs, **Then** an `anonymous-doc` **warning** is reported at that `Doc`.
5. **Given** a definition that belongs to several graphs, **When** the check runs, **Then** it is not reported as orphaned.

---

### User Story 2 - Catch mistakes in a node's labels (Priority: P1)

An author writes node definitions with implied labels (`(Actor::Person)`), optional labels (`` `Director?` ``) and explicit identifiers. They want the check to report labels that point nowhere, point at more than one definition, loop, or are written with the wrong separator.

**Why this priority**: Labels are how definitions refer to each other (§5.4). A dangling or ambiguous implied label silently breaks the model's inheritance-like structure, and `::` misuse changes the meaning of a node.

**Independent Test**: Run the check on files each exhibiting one label mistake and on the two example files; only the intended diagnostics appear.

**Acceptance Scenarios**:

1. **Given** `(Actor::Person)` and no node definition whose identifying label is `Person`, **When** the check runs, **Then** an `undefined-implied-label` **warning** is reported at the `Person` label.
2. **Given** `(Actor::Person)` and `(Person)`, **When** the check runs, **Then** no label diagnostic is reported.
3. **Given** `(actor:Actor:Person)` and `(Person)`, **When** the check runs, **Then** `Person` resolves to the definition whose *identifying label* is `Person` regardless of its identifier, and no diagnostic is reported.
4. **Given** two node definitions whose identifying label is `Person` (for example `(p1:Person)` and `(p2:Person)`) and a node `(Actor::Person)`, **When** the check runs, **Then** an `ambiguous-implied-label` **warning** is reported at `Person`.
5. **Given** `(A::B)`, `(B::C)` and `(C::A)`, **When** the check runs, **Then** one `implied-label-cycle` diagnostic is reported per cycle (not per node), naming the labels in the cycle.
6. **Given** `` (`Person?`) `` or `` (actor:`Actor?`) `` (an identifying label ending in `?`), **When** the check runs, **Then** an `optional-identifying-label` diagnostic is reported; and given `` (Actor::`Director?`) `` it is not.
7. **Given** `(actor:Actor::Person)` (a node label separator `::` that does not directly follow the identifier), **When** the check runs, **Then** a `misplaced-label-separator` **warning** is reported at the separator.
8. **Given** `(Actor::Person)` and `(actor::Actor:Person)`, **When** the check runs, **Then** no `misplaced-label-separator` is reported, because `::` directly follows the identifier.
9. **Given** `name:: cypher\`STRING\`` inside a record, **When** the check runs, **Then** that `::` is not treated as a label separator.
10. **Given** a relationship written `[ownsCar::OWNS]` (a `::` before the relationship's type), **When** the check runs, **Then** a `relationship-double-colon` diagnostic is reported; `[ownsCar:OWNS]` is not reported.

---

### User Story 3 - Catch mistakes in property declarations and records (Priority: P1)

An author declares properties with `key:: type` and documents them. They want the check to report records with repeated keys, values written where declarations belong, reserved `@` names, and documentation for properties that do not exist.

**Why this priority**: These are the "mistyped property name" class of error that motivated the profile (§2). They are cheap to detect and the most likely to survive review unnoticed.

**Independent Test**: Run the check on files with each mistake; the diagnostics point at the offending key or entry.

**Acceptance Scenarios**:

1. **Given** a record `{ title:: "STRING", title:: "INTEGER" }` (or any record anywhere in the file with a repeated key), **When** the check runs, **Then** a `duplicate-key` diagnostic is reported at the second occurrence, naming the key and the line of the first. Parsing the file as plain gram still succeeds.
2. **Given** a node or relationship definition whose record contains `title: "STRING"` with a single colon, **When** the check runs, **Then** a `property-value-in-definition` **warning** is reported at that entry, suggesting `title::`.
3. **Given** a `Graph` or `Doc` record containing `name: "x"`, **When** the check runs, **Then** no `property-value-in-definition` is reported (those records hold values).
4. **Given** a definition declaring a property named `` `@id` `` , **When** the check runs, **Then** a `reserved-property-name` diagnostic is reported at that key. A `Doc` using `` `@description` `` as its own field is not reported.
5. **Given** a `Doc` for `Person` whose record documents property `nmae` while `Person` declares only `name`, **When** the check runs, **Then** a `documented-property-missing` diagnostic is reported at the `nmae` entry, naming the element.
6. **Given** a doc comment with `@property nmae "nom": …` above `[:Doc | Person]` and `Person` declaring `name`, **When** the check runs, **Then** the same diagnostic is reported, located in the comment.
7. **Given** a documented property that does exist on the element (including properties declared on a definition shared between graphs), **When** the check runs, **Then** no diagnostic is reported.
8. **Given** a doc comment that is not attached to any pattern it can describe, **When** the check runs, **Then** a `doc-comment-no-target` diagnostic is reported at the comment.
9. **Given** a doc comment on an annotation-wrapped definition, **When** the check runs, **Then** the comment is read through the annotations to the definition and is not reported as targetless.

---

### User Story 4 - Catch mistakes in structure annotations (Priority: P2)

An author writes `@cardinality([...])` and `@self` style annotations on relationships. They want the check to report a cardinality array whose length does not match the pattern and a self-loop declared on a relationship whose ends differ.

**Why this priority**: Cardinality drives exporters and any later enforcement (§5.3), but it is used on fewer definitions than identity, labels and properties.

**Independent Test**: Run the check on relationships of arity 2 and 3, with correct and incorrect cardinality arrays and self-loop declarations.

**Acceptance Scenarios**:

1. **Given** `@cardinality([0..., 1...])` on `(Person)=[ACTED_IN]=>(Movie)`, **When** the check runs, **Then** no diagnostic is reported.
2. **Given** `@cardinality([0...])` on the same relationship (one range for two pattern elements), **When** the check runs, **Then** a `cardinality-length-mismatch` diagnostic is reported at the annotation, stating the expected and actual counts.
3. **Given** a relationship of more than two pattern elements and a cardinality array with one range per element, **When** the check runs, **Then** no diagnostic is reported (the rule is arity-independent).
4. **Given** a self-loop declared on `(Person)=[KNOWS]=>(Person)` where both ends are the same node definition, **When** the check runs, **Then** no diagnostic is reported.
5. **Given** a self-loop declared on `(Person)=[WORKS_FOR]=>(Company)`, **When** the check runs, **Then** a `self-loop-ends-differ` diagnostic is reported at the self-loop annotation.
6. **Given** a cardinality annotation placed on a definition that sits inside other annotations, **When** the check runs, **Then** the checks look through the annotation wrappers to the definition.

---

### User Story 5 - Catch `Doc`s that cannot be told apart (Priority: P2)

An author accumulates several `Doc`s for an element, one per language, audience or version. They want the check to flag two that are indistinguishable, since nothing could choose between them, without flagging legitimate variants.

**Why this priority**: Accumulating `Doc`s (§5.2) is the profile's main idea for meaning; an indistinguishable pair is almost always a copy-paste slip, but the rule is subtle, so it ranks behind identity, labels and properties.

**Independent Test**: Run the check on sets of `Doc`s that differ in language, audience, graph, or nothing.

**Acceptance Scenarios**:

1. **Given** two `Doc`s for `Person` in the same graph with the same `@language` and the same values for every other `@`-prefixed field except description, labels and property entries, **When** the check runs, **Then** a `duplicate-doc` diagnostic is reported at the second, pointing at the first.
2. **Given** two `Doc`s for `Person` with different `@language` values, **When** the check runs, **Then** no diagnostic is reported.
3. **Given** two `Doc`s with the same `@language` but different values for another `@`-prefixed field (for example `@audience`), **When** the check runs, **Then** no diagnostic is reported.
4. **Given** one doc comment's `Doc` that appears inline in two graphs that both contain the definition, **When** the check runs, **Then** the copies are not reported as duplicates.
5. **Given** two `Doc`s that differ only in their descriptions, **When** the check runs, **Then** they are reported as duplicates (the description is not a distinguishing field).
6. **Given** a doc comment on `Person` and a written `Doc` for `Person` in the same graph, neither with a `@language`, **When** the check runs, **Then** `duplicate-doc` is reported on the later one, because with no `@language` and no other `@`-prefixed field they cannot be told apart. Giving one a `@language` clears it.

---

### User Story 6 - Get diagnostics I can act on, in the form I need (Priority: P1)

Every check reports through the same channel as today: human-readable lines for an author at a terminal, and the JSON report for editors and CI. Each diagnostic carries the file, the line, a severity and a stable check name, so authors can find the problem and tools can match on the name.

**Why this priority**: Without a uniform, stable diagnostic shape, none of the other stories are useful to a tool, and renaming a check later would break its consumers.

**Independent Test**: Run the check on a file with several mistakes in text and JSON modes and compare the two outputs and the exit status.

**Acceptance Scenarios**:

1. **Given** a file with several mistakes, **When** the author runs the check, **Then** each diagnostic line shows file, line, column, severity, check name and message, in file order.
2. **Given** the same file with the JSON option, **When** the check runs, **Then** the report lists the same diagnostics with the same names, severities and ranges, in the report schema the command already emits.
3. **Given** a file with only warnings, **When** the check runs, **Then** the exit status is success; **Given** a file with at least one error, **Then** it is failure.
4. **Given** several files on the command line, **When** the check runs, **Then** each file is checked independently and results are reported per file, and one file's mistakes do not hide another's.
5. **Given** `examples/pattern-rs-spike/movies.ontology.gram`, **When** the check runs, **Then** no diagnostic is reported. `person.ontology.gram` is a discussion file that sets alternatives side by side, so it is expected to be reported, and is left as is.
6. **Given** a file with a syntax error, **When** the check runs, **Then** the syntax error is reported as today and checks that need a well-formed tree are not allowed to produce misleading diagnostics from the broken region.

---

### Edge Cases

- A file that is not an ontology (neither named `*.ontology.gram` nor `kind: ontology`): the existing opt-in mismatch behavior is unchanged, and checks apply as they do for any file the author asks to check.
- An empty file, or a file containing only the top-level record.
- A bare reference at the top level is a syntax error (grammar), while top-level `(a)` and `()-[r]->()` are valid patterns and are treated as definitions. A reference to an identifier that no definition has is now the `undefined-reference` error (feature 002; formerly Assumption A6).
- A definition with an explicit identifier whose identifying label equals another definition's identifier: identity and label lookups are separate namespaces (§5.4).
- A node and a relationship sharing a name, with and without an explicit identifier.
- `::` appearing in a node's record (property declaration) in the same node as a `::` label separator: both are read by their own rule.
- Backticked identifiers and labels containing spaces or `?`: `` `Director?` `` is a label whose trailing `?` marks it optional for these checks.
- A cycle of length one (`(A::A)`): it is a cycle.
- A cycle through an optional label (`` (A::`B?`) `` and `(B::A)`): `?` marks optionality, not a different label, so the cycle is still reported.
- One mistake that triggers two checks (for instance a `::` misplaced separator whose label is also undefined): each applicable check reports; neither suppresses the other.
- Repeated keys in nested records, and in records on `Doc`s, annotations, graphs and definitions alike.
- Diagnostics for content inside a doc comment point to the correct file line, not to a position inside the comment's own text.
- Very large files: the command remains usable (see SC-005).

## Requirements *(mandatory)*

### Functional Requirements

Each requirement names the diagnostic's stable check name. Severities are as RFC-001 states them; where the RFC does not state one, see the severity table below and Assumption A1.

**Opt-in (already implemented, retained)**

- **FR-001**: The command MUST report `opt-in-mismatch` (warning) when exactly one of "file named `*.ontology.gram`" and "top-level record declares `kind: ontology`" holds. This existing behavior MUST NOT regress.

**Identity and graphs (§5.1, §5.6)**

- **FR-002**: The command MUST report `duplicate-key` when any record in the file has the same key twice, even though the file parses as gram. This is the finding `gram lint` already makes under the same name, and it is covered by FR-028 rather than by a separate rule.
- **FR-003**: The command MUST report `orphaned-pattern` (warning) for each top-level node, relationship or annotation that no `Graph` pattern in the file refers to. A file with no `Graph` pattern reports every definition as orphaned. A top-level `Doc` that has an identifier is not reported as an orphan, because only nodes, relationships and annotations are (RFC §5.1).
- **FR-004**: The command MUST report `anonymous-doc` (warning) for each top-level `Doc` with no identity.
- **FR-005**: The command MUST report `duplicate-identifier` when two definitions have the same identifier, where the identifier is the explicit identifier if present and otherwise the identifying label (node) or type (relationship).
- **FR-006**: The command MUST NOT report a definition as orphaned merely because it appears in more than one graph, and MUST NOT report a definition as a duplicate identifier merely because two definitions share a type but carry different explicit identifiers.

**Meaning (§5.2, §5.6)**

- **FR-007**: The command MUST report `documented-property-missing` when a `Doc` (written as a pattern or as a doc comment) documents a property that the element it references does not declare.
- **FR-008**: The command MUST report `doc-comment-no-target` for a doc comment that has no pattern to describe, reading through annotation wrappers to the wrapped definition.
- **FR-009**: The command MUST report `duplicate-doc` for two `Doc`s referring to one element in one graph with the same `@language` and the same values for every other `@`-prefixed field except description, labels and property entries. It MUST NOT report copies of one comment's `Doc` in different graphs, nor `Doc`s differing by audience, version or any other `@`-prefixed field.
- **FR-010**: The command MUST report `reserved-property-name` for a declared property whose name begins with `@`, in any definition. `@`-prefixed keys that are a `Doc`'s own metadata fields are not property names and MUST NOT be reported.

**Structure (§5.3, §5.6)**

- **FR-011**: The command MUST report `cardinality-length-mismatch` when the number of ranges in a `@cardinality` annotation differs from the number of elements in the pattern it wraps, for relationships of any arity, looking through nested annotations.
- **FR-012**: The command MUST report `self-loop-ends-differ` when a self-loop is declared on a relationship whose ends are not the same node.

**Labels (§5.4, §5.6)**

- **FR-013**: The command MUST report `undefined-implied-label` (warning) when an implied label matches no node definition's identifying label.
- **FR-014**: The command MUST resolve an implied label by identifying label (not by identifier), and MUST report `ambiguous-implied-label` (warning) when it matches more than one definition's identifying label.
- **FR-015**: The command MUST report `implied-label-cycle` when implied labels form a cycle, once per cycle.
- **FR-016**: The command MUST report `optional-identifying-label` when a node's identifying label ends in `?`. A trailing `?` on an implied label is valid.
- **FR-017**: The command MUST report `misplaced-label-separator` (warning) for a node label separator `::` that does not directly follow the node's identifier, and MUST NOT treat the `::` between a record key and its type as a label separator.
- **FR-018**: The command MUST report `relationship-double-colon` for a `::` before a relationship's type.

**Declarations (§5.5, §5.6)**

- **FR-019**: The command MUST report `property-value-in-definition` (warning) for a `key: value` entry with a single colon in the record of a node or relationship definition, and MUST NOT report single-colon entries in records of `Graph`s, `Doc`s or other non-definition patterns.

**Diagnostics and behavior**

- **FR-020**: Every diagnostic MUST carry the file path, a start line and column, an end position, a severity (warning or error), a stable check name and a human-readable message. For diagnostics located in a doc comment the position MUST be in the host file.
- **FR-021**: Check names are stable identifiers. They MUST be unique per check and kebab-case. Syntax findings use `gram lint`'s names (`syntax-error`, `missing-node`), replacing this tool's former `parse-error`, which has no consumers other than this project. The other existing names (`opt-in-mismatch`, `gramdoc-parse-error`, `io-error`) are kept.
- **FR-022**: Diagnostics MUST be available as human-readable lines and as the command's existing JSON report, with identical content.
- **FR-023**: The exit status MUST be failure if and only if at least one error-severity diagnostic is reported; warnings alone MUST NOT fail the command.
- **FR-024**: The command MUST read every file named on the command line and report on each independently.
- **FR-025**: The command MUST NOT modify any file.
- **FR-026**: A file that is valid per the profile MUST yield no diagnostics from this feature's checks (no false positives on conforming input), including `examples/pattern-rs-spike/movies.ontology.gram`.
- **FR-027**: Checks that depend on resolving references (labels, documented properties, orphans, duplicates) MUST NOT report diagnostics derived from regions of the file that failed to parse.
- **FR-028**: The command MUST report every finding that `gram lint` makes on the same file, with lint's own check name, severity and meaning (currently `syntax-error`, `missing-node`, `duplicate-element` and `duplicate-key`), in addition to the profile checks. It is a superset of `gram lint`. A finding that both lint and a profile check would make for the same location is reported once, under lint's name.
- **FR-029**: Where `gram lint` gains or changes a check, the command MUST follow it, so that the superset claim holds for the lint version the command is built against.

### Severity table

| Check name | Severity | Source |
|------------|----------|--------|
| `syntax-error`, `missing-node`, `duplicate-element` | as `gram lint` | inherited (FR-028) |
| `opt-in-mismatch` | warning | RFC §5.1 |
| `orphaned-pattern` | warning | RFC §5.1 |
| `anonymous-doc` | warning | RFC §5.1 |
| `misplaced-label-separator` | warning | RFC §5.4 |
| `undefined-implied-label` | warning | RFC §5.4 |
| `ambiguous-implied-label` | warning | RFC §5.4 |
| `property-value-in-definition` | warning | RFC §5.5 |
| `duplicate-doc` | warning | Clarified 2026-11-02 (RFC §7.5 open) |
| `duplicate-key` | as `gram lint` | inherited (FR-028); RFC states no severity |
| `duplicate-identifier` | error | Clarified 2026-11-02 (RFC states none) |
| `documented-property-missing` | error | Clarified 2026-11-02 (RFC states none) |
| `doc-comment-no-target` | error | Clarified 2026-11-02 (RFC states none) |
| `cardinality-length-mismatch` | error | Clarified 2026-11-02 (RFC states none) |
| `self-loop-ends-differ` | error | Clarified 2026-11-02 (RFC states none) |
| `implied-label-cycle` | error | Clarified 2026-11-02 (RFC states none) |
| `optional-identifying-label` | error | Clarified 2026-11-02 (RFC states none) |
| `relationship-double-colon` | error | Clarified 2026-11-02 (RFC states none) |
| `reserved-property-name` | error | RFC §5.2 says "not allowed"; clarified 2026-11-02 |

The severities marked "Clarified" were decided by the author in the 2026-11-02 clarification session; the RFC may later state them differently (Assumption A1).

### Key Entities

- **Diagnostic**: one finding. Has a file, a position range, a severity, a stable check name and a message. It may point at a second location (the earlier definition, key or `Doc`) for duplicate-class checks.
- **Definition**: a top-level node or relationship that declares an element, with an identifier, labels (nodes) or type (relationships), a record of property declarations, and any wrapping annotations.
- **Doc**: a pattern (written out or read from a doc comment) that references an element and carries `@`-prefixed fields and documented-property entries.
- **Graph**: a pattern listing members by identity; the membership relation decides what is orphaned.
- **Check**: one named rule from §5.6, with a severity.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every check bullet in RFC-001 §5.6 is traceable to at least one check name in this spec (18 checks including `opt-in-mismatch`, of which `duplicate-key` is inherited from `gram lint`), and each check has at least one passing and one failing acceptance scenario that is exercised by an automated test.
- **SC-002**: An author running the command on a file seeded with one instance of every mistake sees all of them reported in a single run, each at the correct line.
- **SC-003**: Running the command on `examples/pattern-rs-spike/movies.ontology.gram` and on every "valid" acceptance-scenario file reports zero diagnostics (no false positives).
- **SC-004**: The text and JSON outputs of any run agree: same count, order, check names, severities and positions.
- **SC-005**: Checking a file of 10,000 lines completes in under 2 seconds on an ordinary developer laptop.
- **SC-006**: A reader can learn what any diagnostic means, and what to change, from its message alone, without consulting the RFC (verified by review of each message).
- **SC-007**: For any file, the set of `gram lint` diagnostics (check name, severity, position) is contained in this command's output (verified on every acceptance-scenario file and the two examples).

## Assumptions

- **A1 (severity defaults)**: §5.6 states a severity only for some checks. The author decided that every check whose severity the RFC leaves unstated is an **error** (see Clarifications), so each such rule violation fails the run. `duplicate-key` takes `gram lint`'s severity (FR-028). `duplicate-doc` is a **warning**, also decided by the author, though RFC §7.5 is still open. Severities MUST be easy to change in one place, since the RFC may state them differently later.
- **A2 (configurable severity, RFC §7.5)**: Decided by the author: this feature does **not** add configuration. Severities are fixed per check as in the table; configuration (per-check levels, switching a check off, or a treat-warnings-as-errors switch) is a separate future feature, to be added without renaming checks.
- **A3 (open syntax questions, RFC §7.1-7.4)**: The spec follows the RFC as written: the optional mark is the backticked `` `Director?` ``, a self-loop is written `@self(true)` (EXP-003), until RFC §7.2 settles a shorter form, only block doc comments exist (§7.3), and a required property is folded into its type, `STRING!` (§7.4). Checks that depend on these (optional label detection, self-loop detection, documented-property lookup) apply to the spellings the RFC currently uses and are revisited if §7.1-7.4 change.
- **A4 (standalone, a superset of `gram lint`)**: `gram-ontology check` runs completely on its own: it does not require `gram lint` or any other tool to have run first, and does not assume a clean lint result. It reports everything `gram lint` reports (FR-028) plus the profile checks, so one run is sufficient. The lint findings come from the same lint implementation `gram` uses, so the two cannot disagree; reuse versus reimplementation is a planning matter.
- **A4a (future gram schema)**: gram has no schema specification yet. Until it does, this command is the complete implementation of the profile's checks, and its behavior is expected to inform that schema work. If a gram schema later covers some checks, adopting it is a separate feature; this spec does not anticipate it.
- **A5 (scope of the command)**: The command checks one file at a time against the profile. It does not resolve references across files, merge ontologies, or check data against an ontology (RFC §4 Non-Goals).
- **A6 (references outside §5.6)**: A reference is never a top-level element (RFC §5.1), and the gram grammar itself enforces it: top-level elements can only be patterns, so a bare reference at the top level is reported as a syntax error (FR-028) and needs no profile check. A top-level `(a)` or `()-[r]->()` is a pattern, not a reference, and is treated as a definition. A reference to an identifier that no definition has is not a §5.6 check, so this spec adds none. **Superseded by `specs/002-ontology-check-cleanup/spec.md`**: RFC-001 §5.6 now makes an undefined reference an error (`undefined-reference`), so that statement no longer holds.
- **A7 (existing behavior)**: The existing opt-in-mismatch and doc-comment parse checks, their names and severities stay as they are; syntax findings change name to lint's (FR-021). No other consumer exists, so the rename needs no compatibility period. Whether the opt-in mismatch should remain a warning is an open question elsewhere (EXP-002) and not decided here.
- **A8 (output)**: The JSON report keeps its existing schema version and shape; this feature adds diagnostics, not fields. The "second location" for duplicate-class checks goes in the diagnostic's `help` field.
- **A9 (definition of "duplicate" for identifiers)**: Identifiers are compared exactly (case-sensitive, after removing backticks), as gram compares identities.
- **A10 (test material)**: Acceptance scenarios here are the source for automated tests; example snippets are illustrative and will be completed into full files at planning.
