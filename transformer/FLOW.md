## Transformer flow (tl;dr)

1) **Entry**: `heavy.cjs` runs as a TS plugin, visits `heavy(fn, args?)` calls.
2) **Args preamble**: `argsResolver.cjs` converts the `args` object literal into `const` bindings (only literals/identifiers with literal initializers).
3) **Body prep**: `lib/body.cjs` turns the function body into a statement list.
4) **Return capture**: `lib/returnCapture.cjs` injects `__heavyOutput` declaration and rewrites the first `return` to assign into it.
5) **Logging**: `lib/logging.cjs` appends `console.log("__HEAVY_OUTPUT__:", <joined output>)`.
6) **Snippet build**: `lib/snippet.cjs` stitches preamble + body + log into one string literal.
7) **Output**: The transformer replaces the original `heavy` call with `heavy("<snippet>")`; the string goes to the Rust pipeline unchanged.

### Inputs / Outputs
- **Input**: A call `heavy(fn, args?)` where `fn` is sync (arrow/function) and `args` is an object literal.
- **Output**: A single template literal containing the generated snippet, with `__heavyOutput` logged.

### Notes / Guardrails
- If `fn` has a return type annotation, it’s applied to `__heavyOutput`.
- `args` resolution is static and limited to literals/identifiers with literal initializers; complex runtime data must be serialized before calling `heavy`.
