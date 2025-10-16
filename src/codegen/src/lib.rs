pub mod expr;
pub mod function;
pub mod stmt;

use std::fmt;

use ir::{IrItem, IrModule};
use proc_macro2::TokenStream;
use quote::quote;

pub trait Codegen {
    type Output;

    fn codegen(&self) -> Self::Output;
}

#[derive(Debug, Clone)]
pub enum ModuleElement {
    Item(TokenStream),
    Statement(TokenStream),
    Empty,
}

impl ModuleElement {
    pub fn item(tokens: TokenStream) -> Self {
        Self::Item(tokens)
    }

    pub fn statement(tokens: TokenStream) -> Self {
        Self::Statement(tokens)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, ModuleElement::Empty)
    }
}

impl fmt::Display for ModuleElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleElement::Item(tokens) | ModuleElement::Statement(tokens) => {
                write!(f, "{}", tokens)
            }
            ModuleElement::Empty => Ok(()),
        }
    }
}

#[derive(Default)]
pub struct ModuleGenerator {
    items: Vec<TokenStream>,
    main_body: Vec<TokenStream>,
}

impl ModuleGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_element(&mut self, element: ModuleElement) {
        match element {
            ModuleElement::Item(tokens) => self.items.push(tokens),
            ModuleElement::Statement(tokens) => self.main_body.push(tokens),
            ModuleElement::Empty => {}
        }
    }

    pub fn finish(self) -> TokenStream {
        let Self { items, main_body } = self;
        quote! {
            #(#items)*

            fn main() {
                #(#main_body)*
            }
        }
    }
}

impl Codegen for IrModule {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        let mut generator = ModuleGenerator::new();
        for item in &self.items {
            generator.add_element(item.codegen());
        }
        generator.finish()
    }
}

impl Codegen for IrItem {
    type Output = ModuleElement;

    fn codegen(&self) -> ModuleElement {
        match self {
            IrItem::Function(func) => ModuleElement::item(func.codegen()),
            IrItem::Expression(expr) => {
                let expr_tokens = expr.codegen();
                ModuleElement::statement(quote! { #expr_tokens; })
            }
            IrItem::Block(stmts) => {
                let stmt_tokens = stmts.iter().map(|stmt| stmt.codegen());
                ModuleElement::statement(quote! { { #(#stmt_tokens)* } })
            }
            IrItem::Variable(var) => ModuleElement::statement(var.codegen()),
        }
    }
}

#[cfg(test)]
mod tests;
