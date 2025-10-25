use ir::{IrFunction, IrType, IrVariable};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::Codegen;

impl Codegen for IrFunction {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let params = self.params.iter().map(|param| {
            let ident = format_ident!("{}", param.name);
            let ty = render_type(&param.ty);
            quote! { #ident: #ty }
        });
        let return_ty = render_type(&self.ret);
        let body = self.body.iter().map(|stmt| stmt.codegen());

        quote! {
            fn #name( #( #params ),* ) -> #return_ty {
                #( #body )*
            }
        }
    }
}

impl Codegen for IrVariable {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        let ident = format_ident!("{}", self.name);
        let value = self
            .value
            .as_ref()
            .map(|expr| expr.codegen())
            .unwrap_or_else(|| default_value(&self.ty));

        let mutability = self.mutable.then(|| quote! { mut });
        match &self.ty {
            IrType::Any => quote! {
                let #mutability #ident = #value;
            },
            _ => {
                let ty = render_type(&self.ty);
                quote! {
                    let #mutability #ident: #ty = #value;
                }
            }
        }
    }
}

pub(crate) fn render_type(ty: &IrType) -> TokenStream {
    match ty {
        IrType::Number => quote! { f64 },
        IrType::Str => quote! { runtime::value::Value },
        IrType::Bool => quote! { bool },
        IrType::Unit => quote! { () },
        IrType::Any => quote! { runtime::value::Value },
        IrType::Value => quote! { runtime::value::Value },
        IrType::Array(_) => quote! { ::std::vec::Vec<runtime::value::Value> },
    }
}

fn default_value(ty: &IrType) -> TokenStream {
    match ty {
        IrType::Number => quote! { 0.0f64 },
        IrType::Str => quote! { ::std::string::String::new() },
        IrType::Bool => quote! { false },
        IrType::Unit => quote! { () },
        IrType::Any | IrType::Value => quote! { runtime::value::Value::Undefined },
        IrType::Array(_) => quote! { ::std::vec::Vec::<runtime::value::Value>::new() },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrBinOp, IrExpression, IrFunction, IrParam, IrStmt, IrType, IrVariable};
    use quote::quote;
    use syn::{
        BinOp, Expr, ExprBinary, ExprParen, ExprPath, FnArg, Item, ItemFn, ReturnType, Stmt, Type,
    };

    fn parse_function(tokens: TokenStream) -> ItemFn {
        match syn::parse2::<Item>(tokens).expect("function should parse") {
            Item::Fn(func) => func,
            _ => panic!("expected function item"),
        }
    }

    #[test]
    fn function_codegen_emits_rust_function() {
        let function = IrFunction {
            name: "add".into(),
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
            ret: IrType::Number,
            body: vec![IrStmt::Return(Some(IrExpression::Binary {
                op: IrBinOp::Add,
                left: Box::new(IrExpression::Identifier("a".into())),
                right: Box::new(IrExpression::Identifier("b".into())),
            }))],
        };

        let func = parse_function(function.codegen());
        assert_eq!(func.sig.ident.to_string(), "add");
        assert_eq!(func.sig.inputs.len(), 2);

        let mut inputs = func.sig.inputs.iter();
        let first = inputs.next().unwrap();
        let second = inputs.next().unwrap();

        for arg in [first, second] {
            match arg {
                FnArg::Typed(pat) => match pat.ty.as_ref() {
                    Type::Path(path) => {
                        let ident = path.path.get_ident().expect("type ident");
                        assert_eq!(ident.to_string(), "f64");
                    }
                    _ => panic!("unexpected param type"),
                },
                _ => panic!("unexpected argument"),
            }
        }

        match &func.sig.output {
            ReturnType::Type(_, ty) => match ty.as_ref() {
                Type::Path(path) => {
                    let ident = path.path.get_ident().expect("return ident");
                    assert_eq!(ident.to_string(), "f64");
                }
                _ => panic!("unexpected return type"),
            },
            ReturnType::Default => panic!("expected explicit return type"),
        }

        let stmt = func
            .block
            .stmts
            .first()
            .expect("function should contain return statement");

        let expr = match stmt {
            Stmt::Expr(expr, _) => expr,
            _ => panic!("unexpected statement in function body"),
        };

        let return_expr = match expr {
            Expr::Return(ret) => ret,
            _ => panic!("expected return expression"),
        };

        let value_expr = return_expr
            .expr
            .as_ref()
            .expect("return should carry value")
            .as_ref();

        match value_expr {
            Expr::Binary(ExprBinary {
                left, op, right, ..
            }) => {
                assert!(matches!(op, BinOp::Add(_)));

                let left_ident = match left.as_ref() {
                    Expr::Paren(ExprParen { expr, .. }) => match expr.as_ref() {
                        Expr::Path(ExprPath { path, .. }) => {
                            path.get_ident().expect("left ident").to_string()
                        }
                        _ => panic!("unexpected left operand"),
                    },
                    _ => panic!("expected paren-wrapped left operand"),
                };

                let right_ident = match right.as_ref() {
                    Expr::Paren(ExprParen { expr, .. }) => match expr.as_ref() {
                        Expr::Path(ExprPath { path, .. }) => {
                            path.get_ident().expect("right ident").to_string()
                        }
                        _ => panic!("unexpected right operand"),
                    },
                    _ => panic!("expected paren-wrapped right operand"),
                };

                assert_eq!(left_ident, "a");
                assert_eq!(right_ident, "b");
            }
            _ => panic!("expected binary addition in return"),
        }
    }

    #[test]
    fn variable_codegen_defaults_when_value_missing() {
        let variable = IrVariable {
            name: "flag".into(),
            mutable: false,
            ty: IrType::Bool,
            value: None,
        };

        let tokens = variable.codegen();
        assert_eq!(
            tokens.to_string(),
            quote! { let flag: bool = false; }.to_string()
        );
    }

    #[test]
    fn variable_codegen_for_any_uses_value_default() {
        let variable = IrVariable {
            name: "payload".into(),
            mutable: true,
            ty: IrType::Any,
            value: None,
        };

        let tokens = variable.codegen();
        assert_eq!(
            tokens.to_string(),
            quote! { let mut payload = runtime::value::Value::Undefined; }.to_string()
        );
    }

    #[test]
    fn render_type_maps_all_variants() {
        let cases = vec![
            (IrType::Number, quote! { f64 }),
            (IrType::Str, quote! { ::std::string::String }),
            (IrType::Bool, quote! { bool }),
            (IrType::Unit, quote! { () }),
            (IrType::Any, quote! { runtime::value::Value }),
            (IrType::Value, quote! { runtime::value::Value }),
        ];

        for (ty, expected) in cases {
            assert_eq!(render_type(&ty).to_string(), expected.to_string());
        }
    }
}
