mod declarations;
pub mod expressions;
mod infer;
mod module;
mod params;
mod statements;
#[cfg(test)]
mod test_utils;
mod types;

pub use module::ast_to_ir;
