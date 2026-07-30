# Commit message style

When generating commit messages:

- Use Conventional Commits.
- Format: <type>(<scope>): <summary>
- Types: feat, fix, refactor, docs, test, chore.
- Scope: optional, but if used, should be a name of a crate, like codegen, parser, schema, logger, project - (if architecture) etc.
- Summary in imperative mood.
- Maximum 72 characters.
- Do not end with a period.

Examples:

feat(parser): support generic attributes
fix(ast): handle empty token stream