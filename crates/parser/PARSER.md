//! # Parser
//!
//! ## Purpose
//! Converts a token stream into an Abstract Syntax Tree (AST).
//!
//! ## Input
//! - `Source code file .ts`
//!
//! ## Output
//! - `Ast (Module type)`
//!
//! ## Responsibilities
//! - Creates token stream.
//! - Parse tokens.
//! - Normalizes module-AST (the result of parsing).
//! - Report errors.