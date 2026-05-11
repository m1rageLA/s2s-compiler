use ir::{
    ArrayCall, IrArrayKind, IrAssignOp, IrBinOp, IrExpression, IrForInit, IrLiteral, IrPostfixOp,
    IrPrefixOp, IrStmt, IrType, RuntimeNamespace,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::collect_stmt_tokens;
use super::expression::expression_stmt_tokens;
use super::label::label_lifetime;
use super::var_decl::var_decl_tokens;
use crate::{Codegen, typing};

pub fn for_loop_tokens(
    init: Option<&IrForInit>,
    condition: Option<&IrExpression>,
    update: Option<&IrExpression>,
    body: &[IrStmt],
    label: Option<&str>,
) -> TokenStream {
    typing::push_scope();
    let init_tokens = render_for_init(init);
    let condition_tokens = condition.map(|expr| expr.codegen());
    let update_tokens = update
        .map(|expr| expression_stmt_tokens(expr))
        .unwrap_or_default();
    let (array_alias_prelude, body_tokens) = if let Some((array_name, i_name, j_name)) =
        detect_disjoint_object_indices(init, condition, update, body)
    {
        let left_ident = format_ident!("ts_2_rs_left");
        let right_ident = format_ident!("ts_2_rs_right");
        let bi_ident = format_ident!("ts_2_rs_bi");
        let bj_ident = format_ident!("ts_2_rs_bj");
        let array_ident = format_ident!("{}", array_name);
        let i_ident = format_ident!("{}", i_name);
        let j_ident = format_ident!("{}", j_name);

        typing::push_scope();
        typing::define_array_index_alias(&array_name, &i_name, "ts_2_rs_bi");
        typing::define_array_index_alias(&array_name, &j_name, "ts_2_rs_bj");
        let body_tokens = collect_stmt_tokens(body);
        typing::pop_scope();

        let prelude = quote! {
            let (#left_ident, #right_ident) = #array_ident.split_at_mut(#j_ident);
            let #bi_ident = &mut #left_ident[#i_ident];
            let #bj_ident = &mut #right_ident[0];
        };
        (Some(prelude), body_tokens)
    } else {
        (None, collect_stmt_tokens(body))
    };
    typing::pop_scope();

    if let Some(condition_tokens) = condition_tokens {
        match label {
            Some(name) => {
                let lifetime = label_lifetime(name);
                quote! {
                    {
                        #init_tokens
                        #lifetime: while #condition_tokens {
                            #array_alias_prelude
                            #(#body_tokens)*
                            #update_tokens
                        }
                    }
                }
            }
            None => quote! {
                {
                    #init_tokens
                    while #condition_tokens {
                        #array_alias_prelude
                        #(#body_tokens)*
                        #update_tokens
                    }
                }
            },
        }
    } else {
        match label {
            Some(name) => {
                let lifetime = label_lifetime(name);
                quote! {
                    {
                        #init_tokens
                        #lifetime: loop {
                            #array_alias_prelude
                            #(#body_tokens)*
                            #update_tokens
                        }
                    }
                }
            }
            None => quote! {
                {
                    #init_tokens
                    loop {
                        #array_alias_prelude
                        #(#body_tokens)*
                        #update_tokens
                    }
                }
            },
        }
    }
}

fn detect_disjoint_object_indices(
    init: Option<&IrForInit>,
    condition: Option<&IrExpression>,
    update: Option<&IrExpression>,
    body: &[IrStmt],
) -> Option<(String, String, String)> {
    let (j_name, i_name) = extract_inner_loop_indices(init, condition, update)?;
    let array_name = find_object_array_with_indices(body, &i_name, &j_name)?;
    Some((array_name, i_name, j_name))
}

fn extract_inner_loop_indices(
    init: Option<&IrForInit>,
    condition: Option<&IrExpression>,
    update: Option<&IrExpression>,
) -> Option<(String, String)> {
    let (j_name, i_name) = match init {
        Some(IrForInit::VarDecl(vars)) if vars.len() == 1 => {
            let var = &vars[0];
            let j_name = var.name.clone();
            let value = var.value.as_ref()?;
            match strip_paren_expr(value) {
                IrExpression::Binary {
                    op: IrBinOp::Add,
                    left,
                    right,
                } => {
                    let i_name = match strip_paren_expr(left.as_ref()) {
                        IrExpression::Identifier(name) => name.clone(),
                        _ => return None,
                    };
                    if !is_one_literal(right.as_ref()) {
                        return None;
                    }
                    (j_name, i_name)
                }
                _ => return None,
            }
        }
        _ => return None,
    };

    if let Some(cond) = condition {
        match strip_paren_expr(cond) {
            IrExpression::Binary {
                op: IrBinOp::LessThan,
                left,
                ..
            } => {
                if !matches!(strip_paren_expr(left.as_ref()), IrExpression::Identifier(name) if name == &j_name)
                {
                    return None;
                }
            }
            _ => return None,
        }
    }

    if let Some(update) = update {
        let update = strip_paren_expr(update);
        let ok = match update {
            IrExpression::PostfixUnary { left, op } => {
                matches!(op, IrPostfixOp::Increment)
                    && matches!(strip_paren_expr(left.as_ref()), IrExpression::Identifier(name) if name == &j_name)
            }
            IrExpression::PrefixUnary { arg, op } => {
                matches!(op, IrPrefixOp::Increment)
                    && matches!(strip_paren_expr(arg.as_ref()), IrExpression::Identifier(name) if name == &j_name)
            }
            IrExpression::Assignment { op, left, right } => {
                matches!(op, IrAssignOp::AddAssign)
                    && matches!(strip_paren_expr(left.as_ref()), IrExpression::Identifier(name) if name == &j_name)
                    && is_one_literal(right.as_ref())
            }
            _ => false,
        };
        if !ok {
            return None;
        }
    }

    Some((j_name, i_name))
}

fn is_one_literal(expr: &IrExpression) -> bool {
    matches!(
        strip_paren_expr(expr),
        IrExpression::Literal(IrLiteral::Number(n)) if (*n - 1.0).abs() < f64::EPSILON
    )
}

fn strip_paren_expr(expr: &IrExpression) -> &IrExpression {
    match expr {
        IrExpression::Paren(inner) => strip_paren_expr(inner.as_ref()),
        _ => expr,
    }
}

fn find_object_array_with_indices(body: &[IrStmt], i_name: &str, j_name: &str) -> Option<String> {
    let mut found = std::collections::HashMap::<String, std::collections::HashSet<String>>::new();
    for stmt in body {
        collect_array_indices(stmt, &mut found);
    }
    for (array, indices) in found {
        if indices.contains(i_name) && indices.contains(j_name) {
            return Some(array);
        }
    }
    None
}

fn collect_array_indices(
    stmt: &IrStmt,
    found: &mut std::collections::HashMap<String, std::collections::HashSet<String>>,
) {
    match stmt {
        IrStmt::Leteral(var) => {
            if let Some(expr) = var.value.as_ref() {
                collect_array_indices_expr(expr, found);
            }
        }
        IrStmt::VarDecl(vars) => {
            for var in vars {
                if let Some(expr) = var.value.as_ref() {
                    collect_array_indices_expr(expr, found);
                }
            }
        }
        IrStmt::Expression(expr) => collect_array_indices_expr(expr, found),
        IrStmt::Return(expr) => {
            if let Some(expr) = expr {
                collect_array_indices_expr(expr, found);
            }
        }
        IrStmt::Block(stmts) => {
            for stmt in stmts {
                collect_array_indices(stmt, found);
            }
        }
        IrStmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_array_indices_expr(condition, found);
            for stmt in then_branch {
                collect_array_indices(stmt, found);
            }
            if let Some(else_branch) = else_branch {
                for stmt in else_branch {
                    collect_array_indices(stmt, found);
                }
            }
        }
        IrStmt::While(condition, body) | IrStmt::DoWhile(body, condition) => {
            collect_array_indices_expr(condition, found);
            for stmt in body {
                collect_array_indices(stmt, found);
            }
        }
        IrStmt::For { body, .. } => {
            for stmt in body {
                collect_array_indices(stmt, found);
            }
        }
        IrStmt::Switch {
            discriminant,
            cases,
        } => {
            collect_array_indices_expr(discriminant, found);
            for case in cases {
                for stmt in &case.consequent {
                    collect_array_indices(stmt, found);
                }
            }
        }
        _ => {}
    }
}

fn collect_array_indices_expr(
    expr: &IrExpression,
    found: &mut std::collections::HashMap<String, std::collections::HashSet<String>>,
) {
    match expr {
        IrExpression::Member { object, .. } => {
            collect_array_index_from_member(object.as_ref(), found);
            collect_array_indices_expr(object.as_ref(), found);
        }
        IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index {
            target,
            index,
            element,
        })) => {
            if let Some(id) = object_array_id(target.as_ref(), *element) {
                if typing::object_alias_is_copy(id) {
                    if let (IrExpression::Identifier(array), IrExpression::Identifier(index_name)) =
                        (target.as_ref(), index.as_ref())
                    {
                        found
                            .entry(array.clone())
                            .or_default()
                            .insert(index_name.clone());
                    }
                }
            }
        }
        IrExpression::Binary { left, right, .. } | IrExpression::Assignment { left, right, .. } => {
            collect_array_indices_expr(left, found);
            collect_array_indices_expr(right, found);
        }
        IrExpression::Call { callee, args } => {
            collect_array_indices_expr(callee, found);
            for arg in args {
                collect_array_indices_expr(arg, found);
            }
        }
        IrExpression::Array(items)
        | IrExpression::ArrayExpr(items)
        | IrExpression::Sequence(items) => {
            for item in items {
                collect_array_indices_expr(item, found);
            }
        }
        IrExpression::Conditional {
            test,
            consequent,
            alternate,
        } => {
            collect_array_indices_expr(test, found);
            collect_array_indices_expr(consequent, found);
            collect_array_indices_expr(alternate, found);
        }
        IrExpression::Unary { expr, .. }
        | IrExpression::Paren(expr)
        | IrExpression::PrefixUnary { arg: expr, .. }
        | IrExpression::PostfixUnary { left: expr, .. } => {
            collect_array_indices_expr(expr, found);
        }
        _ => {}
    }
}

fn collect_array_index_from_member(
    object: &IrExpression,
    found: &mut std::collections::HashMap<String, std::collections::HashSet<String>>,
) {
    if let IrExpression::Identifier(name) = object {
        if let Some(alias) = typing::lookup_object_alias(name) {
            if let (IrExpression::Identifier(array), IrExpression::Identifier(index_name)) =
                (alias.target, alias.index)
            {
                found.entry(array).or_default().insert(index_name);
                return;
            }
        }
    }
    if let IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index {
        target,
        index,
        element,
    })) = object
    {
        if let Some(id) = object_array_id(target.as_ref(), *element) {
            if typing::object_alias_is_copy(id) {
                if let (IrExpression::Identifier(array), IrExpression::Identifier(index_name)) =
                    (target.as_ref(), index.as_ref())
                {
                    found
                        .entry(array.clone())
                        .or_default()
                        .insert(index_name.clone());
                }
            }
        }
    }
}

fn object_array_id(target: &IrExpression, element: Option<IrArrayKind>) -> Option<u32> {
    if let Some(IrArrayKind::Object(id)) = element {
        return Some(id);
    }
    match typing::infer_expression_type(target) {
        Some(IrType::Array(IrArrayKind::Object(id))) => Some(id),
        _ => None,
    }
}

fn render_for_init(init: Option<&IrForInit>) -> TokenStream {
    match init {
        Some(IrForInit::VarDecl(vars)) => var_decl_tokens(vars),
        Some(IrForInit::Expr(expr)) => {
            let expr_tokens = expr.codegen();
            quote! { #expr_tokens; }
        }
        None => TokenStream::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrBinOp, IrExpression, IrLiteral, IrPostfixOp, IrType, IrVariable};

    #[test]
    fn for_loop_with_condition_becomes_while() {
        let init_var = IrVariable {
            name: "i".into(),
            mutable: true,
            ty: IrType::Number,
            value: Some(IrExpression::Literal(IrLiteral::Number(0.0))),
        };

        let body = vec![IrStmt::Expression(IrExpression::Identifier("i".into()))];

        let tokens = for_loop_tokens(
            Some(&IrForInit::VarDecl(vec![init_var])),
            Some(&IrExpression::Binary {
                op: IrBinOp::LessThan,
                left: Box::new(IrExpression::Identifier("i".into())),
                right: Box::new(IrExpression::Literal(IrLiteral::Number(5.0))),
            }),
            Some(&IrExpression::PostfixUnary {
                left: Box::new(IrExpression::Identifier("i".into())),
                op: IrPostfixOp::Increment,
            }),
            &body,
            None,
        );

        let output = tokens.to_string();
        assert!(
            output.contains("let mut i : f64 = (0) as f64"),
            "unexpected init: {output}"
        );
        assert!(
            output.contains("(i) < ((5) as f64)"),
            "unexpected condition: {output}"
        );
        assert!(output.contains("+= 1.0"), "unexpected increment: {output}");
    }
}
