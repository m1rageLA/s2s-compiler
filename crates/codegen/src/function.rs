use ir::{IrArrayKind, IrExpression, IrFunction, IrType, IrVariable};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{analysis, Codegen, typing};

impl Codegen for IrFunction {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        let name = format_ident!("{}", self.name);
        typing::push_scope();
        let param_usages = analysis::infer_param_usages(&self.params, &self.body);
        for param in &self.params {
            typing::define(&param.name, param.ty);
        }
        typing::push_return_type(self.ret);

        let params = self.params.iter().zip(param_usages.iter()).map(|(param, usage)| {
            let ident = format_ident!("{}", param.name);
            let ty = render_param_type(&param.ty, usage.pass);
            let mutability = (usage.mutated && matches!(usage.pass, typing::ParamPass::Value))
                .then(|| quote! { mut });
            quote! { #mutability #ident: #ty }
        });
        let return_ty = render_type(&self.ret);
        let body: Vec<_> = self.body.iter().map(|stmt| stmt.codegen()).collect();

        typing::pop_return_type();
        typing::pop_scope();

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
        let expr_ty = self.value.as_ref().and_then(|expr| typing::infer_expression_type(expr));
        let value = self
            .value
            .as_ref()
            .map(|expr| {
                let mut tokens = expr.codegen();
                if matches!((&self.ty, expr), (IrType::Number | IrType::Str, IrExpression::Identifier(_)))
                    || matches!(self.ty, IrType::Array(_))
                {
                    tokens = quote! { (#tokens).clone() };
                }
                typing::coerce_to_type(tokens, &self.ty, expr_ty)
            })
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
        IrType::UInt => quote! { usize },
        IrType::Str => quote! { ::std::string::String },
        IrType::Bool => quote! { bool },
        IrType::Unit => quote! { () },
        IrType::Any => quote! { runtime::value::Value },
        IrType::Value => quote! { runtime::value::Value },
        IrType::Array(kind) => match kind {
            IrArrayKind::Number => quote! { ::std::vec::Vec<f64> },
            IrArrayKind::Str => quote! { ::std::vec::Vec<::std::string::String> },
            IrArrayKind::Bool => quote! { ::std::vec::Vec<bool> },
            IrArrayKind::Value | IrArrayKind::Any | IrArrayKind::Unknown => {
                quote! { ::std::vec::Vec<runtime::value::Value> }
            }
        },
    }
}

fn render_param_type(ty: &IrType, pass: typing::ParamPass) -> TokenStream {
    if let IrType::Array(_) = ty {
        let inner = render_type(ty);
        match pass {
            typing::ParamPass::MutRef => quote! { &mut #inner },
            typing::ParamPass::Ref => quote! { & #inner },
            typing::ParamPass::Value => inner,
        }
    } else {
        render_type(ty)
    }
}

fn default_value(ty: &IrType) -> TokenStream {
    match ty {
        IrType::Number => quote! { 0.0f64 },
        IrType::UInt => quote! { 0usize },
        IrType::Str => quote! { ::std::string::String::new() },
        IrType::Bool => quote! { false },
        IrType::Unit => quote! { () },
        IrType::Any | IrType::Value => quote! { runtime::value::Value::Undefined },
        IrType::Array(IrArrayKind::Number) => quote! { ::std::vec::Vec::<f64>::new() },
        IrType::Array(IrArrayKind::Str) => {
            quote! { ::std::vec::Vec::<::std::string::String>::new() }
        }
        IrType::Array(IrArrayKind::Bool) => quote! { ::std::vec::Vec::<bool>::new() },
        IrType::Array(IrArrayKind::Value | IrArrayKind::Any | IrArrayKind::Unknown) => {
            quote! { ::std::vec::Vec::<runtime::value::Value>::new() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrBinOp, IrExpression, IrFunction, IrParam, IrStmt, IrType, IrVariable};
    use quote::{ToTokens, quote};
    use syn::{Expr, FnArg, Item, ItemFn, ReturnType, Stmt, Type};

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

        let expected_ty = quote!(f64).to_string();

        for arg in [first, second] {
            match arg {
                FnArg::Typed(pat) => match pat.ty.as_ref() {
                    Type::Path(path) => {
                        assert_eq!(path.to_token_stream().to_string(), expected_ty);
                    }
                    _ => panic!("unexpected param type"),
                },
                _ => panic!("unexpected argument"),
            }
        }

        match &func.sig.output {
            ReturnType::Type(_, ty) => match ty.as_ref() {
                Type::Path(path) => {
                    assert_eq!(path.to_token_stream().to_string(), expected_ty);
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
            Expr::Binary(bin) => {
                assert!(matches!(bin.op, syn::BinOp::Add(_)));
            }
            _ => panic!("expected binary add in return"),
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
            (IrType::UInt, quote! { usize }),
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
