use ir::{IrArrowBody, IrParam};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{analysis, Codegen, function::render_type, typing};

pub(crate) fn arrow_tokens(params: &[IrParam], body: &IrArrowBody) -> TokenStream {
    typing::push_scope();
    let ret = typing::infer_arrow_body_type(body).unwrap_or(ir::IrType::Any);
    typing::push_return_type(ret);
    let param_usages = analysis::infer_param_usages_for_arrow(params, body);
    let param_bindings: Vec<TokenStream> = params
        .iter()
        .zip(param_usages.iter())
        .map(|(param, usage)| {
            let ident = format_ident!("{}", param.name);
            let ty = render_param_type(&param.ty, usage.pass);
            let mutability = (usage.mutated && matches!(usage.pass, typing::ParamPass::Value))
                .then(|| quote! { mut });
            typing::define(&param.name, param.ty);
            quote! { #mutability #ident: #ty }
        })
        .collect();

    let tokens = match body {
        IrArrowBody::Expr(expr) => {
            let params = &param_bindings;
            let expr_tokens = expr.codegen();
            quote! { | #( #params ),* | { #expr_tokens } }
        }
        IrArrowBody::Block(stmts) => {
            let params = &param_bindings;
            let stmt_tokens = stmts.iter().map(|stmt| stmt.codegen());
            quote! { | #( #params ),* | { #( #stmt_tokens )* } }
        }
    };

    typing::pop_return_type();
    typing::pop_scope();
    tokens
}

fn render_param_type(ty: &ir::IrType, pass: typing::ParamPass) -> TokenStream {
    if let ir::IrType::Array(_) = ty {
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

#[test]
fn test_arrow_tokens() {
    use ir::IrStmt;

    let params = vec![IrParam {
        name: "a".to_string(),
        ty: ir::IrType::Str,
    }];

    let body_block = IrArrowBody::Block(vec![IrStmt::Return(Some(ir::IrExpression::Binary {
        left: Box::new(ir::IrExpression::Literal(ir::IrLiteral::Number(1.0))),
        right: Box::new(ir::IrExpression::Literal(ir::IrLiteral::Number(2.0))),
        op: ir::IrBinOp::Add,
    }))]);

    let body_expr = IrArrowBody::Expr(Box::new(ir::IrExpression::Literal(ir::IrLiteral::Number(
        1.0,
    ))));

    let tokens_block = arrow_tokens(&params, &body_block);
    let tokens_expr = arrow_tokens(&params, &body_expr);

    assert_eq!(
        tokens_block.to_string(),
        "| a : :: std :: string :: String | { return (1) + (2) ; }",
    );
    assert_eq!(
        tokens_expr.to_string(),
        "| a : :: std :: string :: String | { 1 }",
    );
}
