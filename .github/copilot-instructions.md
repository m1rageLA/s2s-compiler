# TypeScript to Rust Compiler AI Assistant Instructions

This is a TypeScript to Rust compiler project with a GUI frontend. Here's what you need to know to be productive:

## Project Architecture

The compiler follows a traditional multi-pass architecture:

1. **Parser** (`src/parser/`) - Parses TypeScript code into an AST using SWC
2. **IR** (`src/ir/`) - Internal representation layer that bridges TypeScript and Rust semantics
3. **Lowering** (`src/lowering/`) - Converts TypeScript AST to IR
4. **Codegen** (`src/codegen/`) - Generates Rust code from IR
5. **Runtime** (`src/runtime/`) - Runtime support library for TypeScript features in Rust
6. **GUI** (`gui/`) - Tauri-based GUI frontend using React + TypeScript

## Key Workflows

### Building and Testing
- Build: `cargo build` from root directory
- Run tests: `cargo test`
- Run GUI: `cd gui && npm run tauri dev`

### Code Generation Pipeline
1. TypeScript code is parsed using SWC (See `parser::ast()` in `src/parser/src/lib.rs`)
2. AST is lowered to IR (`lowering::ast_to_ir()` in `src/lowering/src/lib.rs`)
3. IR is converted to Rust code (`ModuleGenerator` in `src/codegen/src/lib.rs`)
4. Generated code is compiled and run using `rustc`

## Common Patterns

### IR Node Structure
IR nodes follow a consistent pattern demonstrated in `src/ir/src/expr.rs`:
- Each node type has a corresponding struct
- Nodes implement common traits like `Codegen`
- Documentation shows TypeScript to Rust mapping

### Runtime Support
TypeScript features are mapped to Rust via the runtime library:
- Console API in `src/runtime/src/console.rs`
- TypeScript value types in `src/runtime/src/value.rs`
- Math functions in `src/runtime/src/math.rs`

### Testing Conventions
Test files in `tests/` mirror source structure:
- `binops.rs` - Binary operations
- `expressions.rs` - Expression evaluation
- `functions.rs` - Function declarations and calls
- `variables.rs` - Variable declarations and scope

## Integration Points
- SWC for TypeScript parsing (`swc_ecma_ast`, `swc_ecma_parser`)
- Tauri for GUI-native integration
- `rustc` for final compilation step

## Project-Specific Notes
- Generated code outputs to `out/` directory
- Error handling uses `anyhow` for error propagation
- GUI state management uses React patterns defined in `gui/src/App.tsx`