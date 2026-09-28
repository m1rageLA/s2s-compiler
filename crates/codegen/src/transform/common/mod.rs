use proc_macro2::TokenStream;

pub mod identifier;

impl From<swc_ecma_ast::Ident> for CommonEnum {
    fn from(ident: swc_ecma_ast::Ident) -> Self {
        CommonEnum::Identifier(ident)
    }
}

pub enum CommonEnum {
    Identifier(swc_ecma_ast::Ident)
}
pub fn emit(node: impl Into<CommonEnum>) -> TokenStream {
    match node.into() {
        CommonEnum::Identifier(ident) => identifier::emit(ident)
    }
}   