mod function;
mod variable;

pub(crate) use function::fn_decl_to_ir;
pub(crate) use variable::var_decl_to_ir;

#[cfg(test)]
mod tests;
