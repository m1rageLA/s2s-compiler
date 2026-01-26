## Overview
- `heavy.cjs` — entry point transformer; delegates to small helpers in `lib/`.
- `argsResolver.cjs` — turns the `args` object literal from the call site into const bindings in the generated snippet. Accepts plain object literals or `heavy.prepareArgs(() => ({ ... }))`, which is executed at transform-time and must return a JSON-like object (identifiers as keys, primitives/arrays/objects as values).
- `bodyTransformer.cjs` — legacy helper (kept for compatibility).
- `lib/` — small, focused helpers:
  - `body.cjs` — builds the function body statements.
  - `returnCapture.cjs` — captures return into `__heavyOutput` and declares it.
  - `logging.cjs` — builds the final `console.log` statement.
  - `typeUtils.cjs` — safe cloning of type nodes and return-type extraction.
  - `snippet.cjs` — orchestrates helpers to produce the final snippet string.

## How it works (short)
1) At compile time, the transformer walks the AST, finds `heavy(...)`.
2) It extracts the function body, rewrites `return` into an assignment to `__heavyOutput`, and adds a final `console.log` with the `__HEAVY_OUTPUT__:` prefix.
3) It prepends const declarations for each entry in `args` using `argsResolver.cjs`.
4) The whole snippet is emitted as a template literal and sent through the Rust pipeline.

## Typical edits
- Change how return values are captured: edit `rewriteReturns` and the `outputDecl` in `heavy.cjs`.
- Adjust argument parsing rules: edit `resolveArgLiteral` and `parseArgsMap` in `argsResolver.cjs`.
- Tweak logging: the log statement is assembled near the end of `buildSnippet` in `heavy.cjs`.

## Usage
- Plugin path in TS config: `config/tsconfig.heavy.json` already points to `../transformer/heavy.cjs`.
- Run demo with transformer: `npm run demo:ts`.

## Debug tips
- Temporary logging inside the transformer: sprinkle `console.log` in `buildSnippet` to inspect the generated snippet string.
- To inspect generated Rust, print `result.rust` after calling `heavy`.

## Guardrails
- `args` must stay an object literal or come from `heavy.prepareArgs(() => ({ ... }))`; the resolver only supports literals/identifiers with literal initializers or transform-time functions that return JSON-like objects.
- The function passed to `heavy` must be synchronous; async code inside won’t run in the Rust pipeline.
