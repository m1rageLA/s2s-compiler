pub mod expression;
pub mod function;
pub mod statements;
pub use statements as stmt;
pub mod runtime;
mod typing;

use std::fmt;

use ir::{IrExpression, IrItem, IrModule, IrType};
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
        typing::reset();
        let mut generator = ModuleGenerator::new();
        for item in &self.items {
            // Register top-level bindings so later expressions can be type-checked during codegen.
            match item {
                IrItem::Variable(var) => {
                    typing::define(&var.name, var.ty);
                    if let Some(ret) = function_return_from_initializer(var.value.as_ref()) {
                        typing::define_function_return(&var.name, ret);
                    }
                }
                IrItem::Function(func) => {
                    typing::define_function_return(&func.name, func.ret);
                    typing::define(&func.name, IrType::Value);
                }
                _ => {}
            }
            generator.add_element(item.codegen());
        }
        generator.finish()
    }
}

fn function_return_from_initializer(expr: Option<&IrExpression>) -> Option<IrType> {
    match expr {
        Some(IrExpression::Function(func)) => Some(func.ret),
        Some(IrExpression::Arrow { params, body }) => infer_arrow_return(params, body),
        _ => None,
    }
}

fn infer_arrow_return(params: &[ir::IrParam], body: &ir::IrArrowBody) -> Option<IrType> {
    typing::push_scope();
    for param in params {
        typing::define(&param.name, param.ty);
    }
    let ty = match body {
        ir::IrArrowBody::Expr(expr) => typing::infer_expression_type(expr),
        ir::IrArrowBody::Block(stmts) => infer_returns(stmts),
    };
    typing::pop_scope();
    ty
}

fn infer_returns(stmts: &[ir::IrStmt]) -> Option<IrType> {
    let mut inferred: Option<IrType> = None;
    let mut saw_return = false;

    for stmt in stmts {
        match stmt {
            ir::IrStmt::Return(Some(expr)) => {
                let ty = typing::infer_expression_type(expr);
                if let Some(found) = ty {
                    if let Some(existing) = inferred {
                        if existing != found {
                            return None;
                        }
                    } else {
                        inferred = Some(found);
                    }
                } else {
                    return None;
                }
                saw_return = true;
            }
            ir::IrStmt::Return(None) => {
                if let Some(existing) = inferred {
                    if existing != IrType::Unit {
                        return None;
                    }
                } else {
                    inferred = Some(IrType::Unit);
                }
                saw_return = true;
            }
            ir::IrStmt::Block(inner) => {
                if let Some(inner_ty) = infer_returns(inner) {
                    if let Some(existing) = inferred {
                        if existing != inner_ty {
                            return None;
                        }
                    } else {
                        inferred = Some(inner_ty);
                    }
                    saw_return = true;
                }
            }
            ir::IrStmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                let then_ty = infer_returns(then_branch);
                let else_ty = else_branch.as_deref().and_then(infer_returns);
                match (then_ty, else_ty) {
                    (Some(a), Some(b)) if a == b => {
                        if let Some(existing) = inferred {
                            if existing != a {
                                return None;
                            }
                        } else {
                            inferred = Some(a);
                        }
                        saw_return = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    if saw_return { inferred } else { Some(IrType::Unit) }
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
mod tests {
    use super::*;
    use ir::{
        IrExpression, IrFunction, IrFunctionExpr, IrItem, IrLiteral, IrModule, IrParam, IrStmt,
        IrType,
    };

    fn parse(tokens: TokenStream) -> syn::File {
        syn::parse2(tokens).expect("generated Rust should parse")
    }

    #[test]
    fn module_codegen_emits_main_and_function_items() {
        let helper_function = IrItem::Function(IrFunction {
            name: "helper".into(),
            params: Vec::new(),
            ret: IrType::Unit,
            body: vec![IrStmt::Return(None)],
        });

        let call_expression = IrItem::Expression(IrExpression::Call {
            callee: Box::new(IrExpression::Identifier("helper".into())),
            args: Vec::new(),
        });

        let ir_module = IrModule {
            items: vec![helper_function, call_expression],
        };

        let file = parse(ir_module.codegen());

        let helper_fn = file
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Fn(func) if func.sig.ident == "helper" => Some(func),
                _ => None,
            })
            .expect("expected helper function to be generated");

        let main_fn = file
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Fn(func) if func.sig.ident == "main" => Some(func),
                _ => None,
            })
            .expect("expected main function to be generated");

        match helper_fn.block.stmts.first() {
            Some(syn::Stmt::Expr(expr, _)) => match expr {
                syn::Expr::Return(ret) => {
                    assert!(ret.expr.is_none(), "helper should return without value");
                }
                _ => panic!("expected return expression inside helper"),
            },
            _ => panic!("expected return statement inside helper"),
        }

        match main_fn.block.stmts.first() {
            Some(syn::Stmt::Expr(expr, _)) => match expr {
                syn::Expr::Call(call) => {
                    let called = match call.func.as_ref() {
                        syn::Expr::Path(path) => path.path.get_ident().map(|i| i.to_string()),
                        syn::Expr::Paren(paren) => match &*paren.expr {
                            syn::Expr::Path(path) => path.path.get_ident().map(|i| i.to_string()),
                            _ => None,
                        },
                        _ => None,
                    }
                    .expect("call should target identifier");
                    assert_eq!(called, "helper");
                    assert!(
                        call.args.is_empty(),
                        "helper call should not pass arguments"
                    );
                }
                _ => panic!("expected helper call expression inside main"),
            },
            _ => panic!("expected helper call inside main"),
        }
    }

    #[test]
    fn function_expression_codegen_emits_closure_literal() {
        let function_expr = IrExpression::Function(Box::new(IrFunctionExpr {
            name: None,
            params: vec![IrParam {
                name: "value".into(),
                ty: IrType::Number,
            }],
            ret: IrType::Number,
            body: vec![IrStmt::Return(Some(IrExpression::Identifier(
                "value".into(),
            )))],
        }));

        let module = IrModule {
            items: vec![IrItem::Expression(function_expr)],
        };

        let file = parse(module.codegen());

        let main_fn = file
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Fn(func) if func.sig.ident == "main" => Some(func),
                _ => None,
            })
            .expect("expected generated main function");

        match main_fn.block.stmts.first() {
            Some(syn::Stmt::Expr(expr, _)) => match expr {
                syn::Expr::Closure(closure) => {
                    assert!(closure.capture.is_none(), "closure should not force move");
                    assert_eq!(closure.inputs.len(), 1);

                    match &closure.inputs[0] {
                        syn::Pat::Type(pat_type) => match pat_type.ty.as_ref() {
                            syn::Type::Path(path) => {
                                assert_eq!(
                                    quote!(#path).to_string(),
                                    quote!(f64).to_string()
                                );
                            }
                            _ => panic!("expected closure arg type to be f64"),
                        },
                        _ => panic!("expected typed closure argument"),
                    }

                    match &closure.output {
                        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
                            syn::Type::Path(path) => {
                                assert_eq!(
                                    quote!(#path).to_string(),
                                    quote!(f64).to_string()
                                );
                            }
                            _ => panic!("expected closure return type to be f64"),
                        },
                        _ => panic!("expected explicit return type"),
                    }
                }
                _ => panic!("expected closure expression"),
            },
            _ => panic!("expected closure statement"),
        }
    }

    #[test]
    fn function_expression_codegen_omits_return_type_for_any() {
        let function_expr = IrExpression::Function(Box::new(IrFunctionExpr {
            name: None,
            params: vec![
                IrParam {
                    name: "a".into(),
                    ty: IrType::Number,
                },
                IrParam {
                    name: "b".into(),
                    ty: IrType::Number,
                },
            ],
            ret: IrType::Any,
            body: vec![IrStmt::Return(Some(IrExpression::Literal(
                IrLiteral::Number(42.0),
            )))],
        }));

        let module = IrModule {
            items: vec![IrItem::Expression(function_expr)],
        };

        let file = parse(module.codegen());

        let main_fn = file
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Fn(func) if func.sig.ident == "main" => Some(func),
                _ => None,
            })
            .expect("expected generated main function");

        match main_fn.block.stmts.first() {
            Some(syn::Stmt::Expr(expr, _)) => match expr {
                syn::Expr::Closure(closure) => match &closure.output {
                    syn::ReturnType::Default => {}
                    _ => panic!("expected no return type, found return type annotation"),
                },
                _ => panic!("expected closure expression"),
            },
            _ => panic!("expected closure statement"),
        }
    }
}
