mod declarations;
mod expressions;
mod infer;
mod module;
mod statements;
mod types;
mod params;

pub use module::ast_to_ir;

#[cfg(test)]
mod tests;
