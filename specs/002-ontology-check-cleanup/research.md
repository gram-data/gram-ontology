# Research

**R1 What is a reference, and what does it resolve against?** Decision: references are bare identifiers among a `Graph`'s elements (also when wrapped by annotations) and a `Doc`'s target. They resolve against definitions' identifiers (explicit identifier, else identifying label / type), `Graph` identifiers and `Doc` identifiers. Rationale: RFC §5.1 and §5.6. Alternatives: resolving against labels too — rejected, §5.4 keeps identifier and label namespaces apart.

**R2 Doc-comment references**: a comment on a definition is read into a `Doc` whose target is that definition by construction; those are not user-written references, so they are excluded.

**R3 Duplicate syntax error**: reproduced with the issue's file: both findings arrive from the lint library with identical rule, range and message. Decision: drop exact repeats where lint's findings are taken. Alternative: dedupe the merged output — rejected, it would hide the genuine pair of profile findings the existing tests rely on.

**R4 Help text**: the existing message and help already name the later definition; choose the example by its kind (`actor:Actor` for a node, `ownsCar:OWNS` for a relationship).

**R5 Annotations**: `@unique(title)` arguments name properties and are not references; only the element an annotation wraps can be one.

**R6 RFC observations**: none contradicting; no RFC change made.
