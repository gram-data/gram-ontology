# Feature Specification: `gram-ontology check` cleanup against RFC-001 §5.6

**Feature Branch**: `002-ontology-check-cleanup`

**Created**: 2026-11-02

**Status**: Draft

**Input**: User description: "Clean up `gram-ontology check` to match RFC-001 section 5.6: add the `undefined-reference` error check; report each syntax error once; add fixtures for a node and relationship sharing a name and for the record key `::`; make the `duplicate-identifier` help fit the kind of definition clashing."

**Design authority**: `design/rfc/RFC-001-ontology-profile.md` (draft), sections 5.1, 5.4, 5.6. The RFC wins wherever anything differs. This spec says what the command reports and why; it does not say how.

**Builds on**: `specs/001-ontology-check-rfc-001/spec.md`. This feature **supersedes that spec's Assumption A6** (see Assumptions) and otherwise leaves its requirements, severities and check names as they are.

## Background

Feature 001 made `gram-ontology check` cover every check in RFC-001 §5.6 as it stood. Since then §5.6 gained a check, "a reference to an identifier that no definition has, whether in a `Graph`'s elements, a `Doc` or an annotation", an error. A review of the command against the RFC also found three smaller faults: a syntax error can be printed twice at one position, two cases the earlier spec lists have no test files, and the `duplicate-identifier` advice is wrong for a node.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Be told when a reference points at nothing (Priority: P1)

An author gives a relationship an explicit identifier, `(Person)=[ownsCar:OWNS]=>(Car)`. Afterwards `OWNS` is only a type, so a `Doc` written `[:Doc {...} | OWNS]` describes nothing, and the description it carries is silently attached to no element (RFC §5.6). The author runs the check and wants to be told.

**Why this priority**: it is the one RFC §5.6 check the command lacks, and the mistake is silent and easy to make the moment an explicit identifier is introduced, which §5.1 asks authors to do.

**Independent Test**: run the check on a file whose `Doc` refers to `OWNS` when only `ownsCar` is an identifier; one error is reported at the reference. Run it on the corrected file; nothing is reported.

**Acceptance Scenarios**:

1. **Given** `(Person)=[ownsCar:OWNS]=>(Car)` and, inside a `Graph`, `[:Doc {...} | OWNS]`, **When** the check runs, **Then** an `undefined-reference` **error** is reported at `OWNS`, naming it, and the exit status is failure.
2. **Given** a `Graph` whose elements list an identifier that no definition has (a mistyped member), **When** the check runs, **Then** an `undefined-reference` error is reported at that member.
3. **Given** an annotated `Doc` (or other pattern) whose elements include a reference to an identifier that no definition has, **When** the check runs, **Then** an `undefined-reference` error is reported at that reference.
4. **Given** the same `Doc` written `[:Doc {...} | ownsCar]`, **When** the check runs, **Then** no `undefined-reference` is reported: a reference by explicit identifier resolves.
5. **Given** a reference to a definition by its own identifier (a node's identifying label, or a relationship's type when it has no explicit identifier), **When** the check runs, **Then** it resolves, and so does a reference to a `Graph` or a `Doc` by that pattern's identifier.
6. **Given** a doc comment on a definition, **When** the check runs, **Then** the comment's implicit reference to that definition resolves and is never reported.
7. **Given** a reference in a region of the file that failed to parse, **When** the check runs, **Then** no `undefined-reference` is derived from that region (as feature 001 FR-027).

---

### User Story 2 - See each syntax error once (Priority: P2)

An author writes a bare `Actor?` among a `Graph`'s elements, as in `[g:Graph {name:"g"} | Actor?, Person]`. The check currently prints `unexpected: "?"` twice at the same line and column. Duplicates make a single mistake look like two.

**Why this priority**: it is noise rather than a missed finding, but it is what an author sees first when a file is malformed.

**Independent Test**: run the check on that file; exactly one `syntax-error` is reported at the `?`.

**Acceptance Scenarios**:

1. **Given** that file, **When** the check runs, **Then** one `syntax-error` is reported at that position, in both text and JSON output.
2. **Given** a file with two different syntax errors at different positions, **When** the check runs, **Then** both are still reported.

---

### User Story 3 - Two remaining cases have test files (Priority: P3)

Feature 001 lists two cases it does not exercise with files: a node and a relationship sharing a name, and `::` between a record key and its type. A maintainer wants good and bad test files for each, so a regression is caught.

**Why this priority**: it protects behavior rather than adding it.

**Independent Test**: the new files run through the existing fixture tests; bad files produce exactly the named check, good files produce nothing.

**Acceptance Scenarios**:

1. **Given** `(Follows)` and `[Follows]` (a node and a relationship, neither with an explicit identifier), **When** the check runs, **Then** `duplicate-identifier` is reported (error); **Given** the same two with explicit identifiers, **Then** nothing is reported.
2. **Given** a node record containing `name:: cypher`STRING``, **When** the check runs, **Then** no `misplaced-label-separator` is reported; **Given** a misplaced label separator in the same file, **Then** only that is reported.

---

### User Story 4 - Advice that fits the clash (Priority: P3)

When two definitions share an identifier, the diagnostic suggests an explicit identifier. Today it always suggests a relationship example (`ownsCar:OWNS`), which misleads an author whose clash is between nodes.

**Why this priority**: wording only, but SC-006 of feature 001 asks that a message say what to change.

**Independent Test**: trigger the clash with two nodes, two relationships, and a node and a relationship; each message's suggestion matches the kind of the definition it is reported on.

**Acceptance Scenarios**:

1. **Given** two clashing nodes, **When** the check runs, **Then** the suggestion is an explicit identifier on a node, such as `actor:Actor`.
2. **Given** two clashing relationships, **Then** the suggestion is an explicit identifier on a relationship, such as `ownsCar:OWNS`.
3. **Given** a node clashing with a relationship, **Then** the suggestion fits the definition the diagnostic is reported on (the later one).

---

### Edge Cases

- A bare reference at the top level remains a syntax error (grammar), reported once; it is not also an `undefined-reference`.
- One identifier defined twice and referenced: it resolves (the `duplicate-identifier` error covers the clash; no `undefined-reference` is added).
- A reference whose name equals only an implied or non-identifying label, or a property name: not an identifier, so it is reported.
- A reference to an identifier given in backticks: compared as written without backticks, as feature 001 A9 does.
- A reference inside a `Graph` that is itself nested or listed in another `Graph`: resolves if any `Graph` or `Doc` has that identifier.
- A reference to an identifier defined only inside a `Graph`'s elements (not top-level): resolves, since that is a definition.
- A file with no references: unchanged output.

## Requirements *(mandatory)*

### Functional Requirements

**Undefined references (RFC §5.6)**

- **FR-001**: The command MUST report `undefined-reference` (**error**) for each reference, in a `Graph`'s elements, as the target of a `Doc`, or wrapped by an annotation, to an identifier that no definition, `Graph` or `Doc` in the file has.
- **FR-002**: An identifier is the explicit identifier when a definition has one, and otherwise the identifying label (node) or type (relationship), exactly as feature 001 FR-005 defines it. The identifier of a `Graph` or `Doc` is its own identifier.
- **FR-003**: The command MUST NOT report a reference that resolves under FR-002, nor the implicit reference of a doc comment to the definition it documents.
- **FR-004**: The diagnostic MUST point at the reference and name the unresolved identifier, and its help MUST tell the author what to change, mentioning that a definition with an explicit identifier is referred to by that identifier.
- **FR-005**: The command MUST NOT derive `undefined-reference` from regions of the file that failed to parse (feature 001 FR-027).

**Syntax errors**

- **FR-006**: The command MUST report each syntax error once. Two syntax errors with the same range and message are one error; errors at different positions are all kept.

**Test material**

- **FR-007**: The repository MUST hold a bad and a good test file for each of: `undefined-reference`; `duplicate-identifier` between a node and a relationship sharing a name; and the record-key `::` that must not trigger `misplaced-label-separator`. They follow the existing naming, and the existing tests exercise them. A bad file for the syntax-error case exercises FR-006.

**Advice**

- **FR-008**: The `duplicate-identifier` help MUST suggest an explicit identifier suited to the kind of the definition the diagnostic is reported on: a node example for a node, a relationship example for a relationship.

**Listing the check**

- **FR-009**: `undefined-reference` MUST be listed, with its error severity, wherever the checks are listed: the single severity table in the code, the command's contract, and the README.

### Severity table (addition to feature 001's)

| Check name | Severity | Source |
|------------|----------|--------|
| `undefined-reference` | error | RFC §5.6 ("an error because nothing else reports it") |

### Key Entities

- **Reference**: a bare identifier written among another pattern's elements (in a `Graph`, as a `Doc`'s target, or wrapped by an annotation).
- **Identifier**: as in feature 001; the set of identifiers is what references resolve against.
- **Diagnostic**: as in feature 001.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every check bullet in RFC-001 §5.6 maps to a check the command reports (19 names including `opt-in-mismatch`; `duplicate-key` inherited), verified by the severity table and tests.
- **SC-002**: The RFC's own example (`ownsCar` with `[:Doc {...} | OWNS]`) is reported, and its corrected form is clean.
- **SC-003**: Running the command on `examples/pattern-rs-spike/movies.ontology.gram` and on every existing "good" test file still reports zero diagnostics.
- **SC-004**: No run reports two diagnostics with the same check name, range and message.
- **SC-005**: Every existing test file still produces the diagnostics it produced before, other than ones that now also trip `undefined-reference` because they referred to nothing (none are expected; any found is a defect in the file to be reported, not hidden).

## Assumptions

- **A6 superseded**: feature 001's Assumption A6 said a reference to an identifier no definition has "is not a §5.6 check". RFC-001 §5.6 now makes it one (an error). A6 of `specs/001-ontology-check-rfc-001/spec.md` is superseded by FR-001 here, and that spec is annotated to say so. Its statement that a bare top-level reference is a syntax error stands.
- **Severity**: error, as the RFC states ("Each is an error unless it is marked a warning"); fixed per check, with no configuration (feature 001 A2).
- **"Annotation"**: gram's grammar does not allow an annotation on a bare reference (nor inside a bracketed element list), so an annotation reaches a reference only by wrapping a pattern, such as a `Doc`, whose elements hold it; those are checked like any other. An annotation's own arguments (`@unique(title)`) name properties, not definitions, and are not references.
- **Which elements are references**: every bare identifier among a pattern's elements (a `Graph`, a `Doc`, or a relationship written `[:R | A, B]`), per RFC §5.1.
- **Syntax error once**: reporting a finding once is about identical range and message; this feature does not change which syntax errors the parser finds.
- **Scope**: one file at a time (feature 001 A5); references are resolved only within the file.
- **RFC**: not changed by this feature. Anything it gets wrong is reported separately.
