# Fixtures

One small file per scenario, named `<check-name>.<good|bad>.ontology.gram`. A `.bad.` file holds
one instance of the mistake; a `.good.` file holds the nearest valid form and must produce no
diagnostics. Extra variants insert a word before `good`/`bad` (for example
`implied-label-cycle.self.bad`).
