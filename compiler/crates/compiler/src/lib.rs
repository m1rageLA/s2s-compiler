//! Unified API surface for the ts2rust toolchain.
//!
//! This crate wires together the individual compiler stages
//! (parser → lowering → codegen) and exposes a simple pipeline-style API.

use codegen::Codegen;
use ir::IrModule;
use proc_macro2::TokenStream;
use swc_ecma_ast::Module;

// =============================
// ENTRY POINT from /api/src/lib
// =============================
pub fn compile_typescript(source: &str) -> Compilation {
    let ast = parse_typescript(source);
    let ir = lower_ast(&ast);
    let tokens = generate_rust_module(&ir);
    Compilation { ast, ir, tokens }
}

fn format_rust_tokens(tokens: &TokenStream) -> String {
    match syn::parse2::<syn::File>(tokens.clone()) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => tokens.to_string(),
    }
}

pub use codegen::{ModuleElement, ModuleGenerator};

/// Result of a full TypeScript → Rust compilation pass.
pub struct Compilation {
    pub ast: Module,
    pub ir: IrModule,
    pub tokens: TokenStream,
}

impl Compilation {
    /// Returns the generated Rust module as a `String`.
    pub fn into_rust_string(self) -> String {
        format_rust_tokens(&self.tokens)
    }

    /// Returns the generated Rust module as a formatted `String` without consuming self.
    pub fn rust_string(&self) -> String {
        format_rust_tokens(&self.tokens)
    }

    /// Returns a reference to the generated token stream.
    pub fn rust_tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

/// Parse TypeScript source into an SWC module AST.
pub fn parse_typescript(source: &str) -> Module {
    parser::ast(source)
}

/// Parse and downlevel TypeScript source, returning normalized JavaScript code.
// pub fn normalize_js(source: &str) -> String {
//     parser::downleveled_js(source)
// }

/// Lower a TypeScript AST module into the project IR.
pub fn lower_ast(module: &Module) -> IrModule {
    lowering::ast_to_ir(module)
}

/// Turn an IR module into Rust tokens (without writing them anywhere).
pub fn generate_rust_module(ir_module: &IrModule) -> TokenStream {
    ir_module.codegen()
}

/// Format a token stream into a Rust source string.
pub fn format_tokens(tokens: &TokenStream) -> String {
    format_rust_tokens(tokens)
}

/// Commonly used exports for consumers who need more granular control.
pub mod prelude {
    pub use super::{
        Compilation, compile_typescript, format_tokens, generate_rust_module, lower_ast,
        parse_typescript,
    };
    pub use crate::runtime;
    pub use codegen::{Codegen, ModuleElement, ModuleGenerator};
    pub use ir::{IrItem, IrModule};
}

pub use codegen;
pub use ir;
pub use lowering;
pub use parser;
pub use runtime;
