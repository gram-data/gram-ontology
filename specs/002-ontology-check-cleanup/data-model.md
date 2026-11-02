# Data model

- **Reference** (new in the file model): `name`, `range`. Collected from graph elements (annotation wrappers looked through) and from `Doc` targets written as patterns.
- **Identifier set**: identifiers of all definitions, plus the identifiers of `Graph` and `Doc` patterns.
- **Diagnostic**: unchanged. `undefined-reference`: severity error, range of the reference, message names the identifier, help says a definition with an explicit identifier is referred to by it.
