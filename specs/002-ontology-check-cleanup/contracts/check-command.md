# Contract delta: `gram-ontology check`

Command, exit status, text and JSON output are unchanged from `specs/001-ontology-check-rfc-001/contracts/check-command.md`. Additions to its check catalogue:

| Check name | Severity | Source |
|------------|----------|--------|
| `undefined-reference` | error | RFC §5.6 |

Behavior changes: a finding repeated with the same check name, range and message is reported once; the `duplicate-identifier` help suggests an explicit identifier suited to the reported definition's kind.
