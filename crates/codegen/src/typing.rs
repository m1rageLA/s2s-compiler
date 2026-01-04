use std::cell::RefCell;
use std::collections::HashMap;

use ir::{
    ArrayCall, ConsoleCall, IrArrayKind, IrArrowBody, IrBinOp, IrExpression, IrLiteral, IrStmt,
    IrTemplatePart, IrType, MathCall, RuntimeNamespace, StringCall, ValueCall,
};
use proc_macro2::TokenStream;
use quote::quote;

thread_local! {
    static TYPE_STACK: RefCell<Vec<HashMap<String, IrType>>> = RefCell::new(vec![HashMap::new()]);
    static FN_RETURNS: RefCell<Vec<HashMap<String, IrType>>> = RefCell::new(vec![HashMap::new()]);
    static RETURN_STACK: RefCell<Vec<IrType>> = RefCell::new(vec![]);
}

pub fn reset() {
    TYPE_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    FN_RETURNS.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    RETURN_STACK.with(|stack| stack.borrow_mut().clear());
}

pub fn push_scope() {
    TYPE_STACK.with(|stack| stack.borrow_mut().push(HashMap::new()));
    FN_RETURNS.with(|stack| stack.borrow_mut().push(HashMap::new()));
}

pub fn pop_scope() {
    TYPE_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
    FN_RETURNS.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
}

pub fn define(name: &str, ty: IrType) {
    TYPE_STACK.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), ty);
        }
    });
}

pub fn define_function_return(name: &str, ty: IrType) {
    FN_RETURNS.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), ty);
        }
    });
}

pub fn lookup(name: &str) -> Option<IrType> {
    TYPE_STACK.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(*ty);
            }
        }
        None
    })
}

pub fn lookup_function_return(name: &str) -> Option<IrType> {
    FN_RETURNS.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(*ty);
            }
        }
        None
    })
}

pub fn push_return_type(ty: IrType) {
    RETURN_STACK.with(|stack| stack.borrow_mut().push(ty));
}

pub fn pop_return_type() {
    RETURN_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        if !stack.is_empty() {
            stack.pop();
        }
    });
}

pub fn current_return_type() -> Option<IrType> {
    RETURN_STACK.with(|stack| stack.borrow().last().copied())
}

pub fn infer_expression_type(expr: &IrExpression) -> Option<IrType> {
    match expr {
        IrExpression::Literal(literal) => match literal {
            IrLiteral::Number(_) => Some(IrType::Number),
            IrLiteral::Str(_) => Some(IrType::Str),
            IrLiteral::Bool(_) => Some(IrType::Bool),
        },
        IrExpression::Identifier(name) => {
            if name == "undefined" {
                Some(IrType::Unit)
            } else {
                lookup(name)
            }
        }
        IrExpression::Binary { op, left, right } => infer_binary(*op, left, right),
        IrExpression::Conditional {
            consequent,
            alternate,
            ..
        } => infer_conditional(consequent, alternate),
        IrExpression::Object(_) => Some(IrType::Value),
        IrExpression::Array(elements) => Some(IrType::Array(infer_array_kind(elements))),
        IrExpression::Template(_) => Some(IrType::Str),
        IrExpression::RuntimeCall(runtime) => infer_runtime(runtime),
        IrExpression::Arrow { body, .. } => infer_arrow_body(body),
        IrExpression::Function(func) => Some(func.ret),
        IrExpression::Assignment { right, .. } => infer_expression_type(right),
        IrExpression::Call { callee, .. } => infer_call_return(callee),
        IrExpression::Member { .. } => None,
        IrExpression::SuperCall { .. } => None,
        IrExpression::ArrayExpr(elements) => Some(IrType::Array(infer_array_kind(elements))),
        IrExpression::PostfixUnary { left, .. } => infer_expression_type(left),
        IrExpression::Paren(inner) => infer_expression_type(inner),
    }
}

fn infer_arrow_body(body: &IrArrowBody) -> Option<IrType> {
    match body {
        IrArrowBody::Expr(expr) => infer_expression_type(expr),
        IrArrowBody::Block(stmts) => infer_return_types(stmts),
    }
}

pub fn infer_arrow_body_type(body: &IrArrowBody) -> Option<IrType> {
    infer_arrow_body(body)
}

fn infer_return_types(stmts: &[IrStmt]) -> Option<IrType> {
    let mut inferred: Option<IrType> = None;
    let mut saw_return = false;

    for stmt in stmts {
        match stmt {
            IrStmt::Return(Some(expr)) => {
                let ty = infer_expression_type(expr);
                if !unify(&mut inferred, ty) {
                    return None;
                }
                saw_return = true;
            }
            IrStmt::Return(None) => {
                if !unify(&mut inferred, Some(IrType::Unit)) {
                    return None;
                }
                saw_return = true;
            }
            IrStmt::Block(inner) => {
                if let Some(inner_ty) = infer_return_types(inner) {
                    if !unify(&mut inferred, Some(inner_ty)) {
                        return None;
                    }
                    saw_return = true;
                }
            }
            IrStmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                let then_ty = infer_return_types(then_branch);
                let else_ty = else_branch.as_deref().and_then(infer_return_types);
                match (then_ty, else_ty) {
                    (Some(t1), Some(t2)) if t1 == t2 => {
                        if !unify(&mut inferred, Some(t1)) {
                            return None;
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

fn infer_binary(op: IrBinOp, left: &IrExpression, right: &IrExpression) -> Option<IrType> {
    match op {
        IrBinOp::Add => {
            let left_ty = infer_expression_type(left);
            let right_ty = infer_expression_type(right);

            if left_ty == Some(IrType::Str) || right_ty == Some(IrType::Str) {
                Some(IrType::Str)
            } else if left_ty == Some(IrType::Bool) || right_ty == Some(IrType::Bool) {
                None
            } else if left_ty == Some(IrType::Number)
                || right_ty == Some(IrType::Number)
                || (left_ty.is_none() && right_ty.is_none())
            {
                Some(IrType::Number)
            } else {
                None
            }
        }
        IrBinOp::Sub
        | IrBinOp::Mul
        | IrBinOp::Div
        | IrBinOp::Mod
        | IrBinOp::Exp
        | IrBinOp::LeftShift
        | IrBinOp::RightShift
        | IrBinOp::BitwiseOr
        | IrBinOp::BitwiseXor
        | IrBinOp::BitwiseAnd
        | IrBinOp::UnsignedRightShift => Some(IrType::Number),
        IrBinOp::Equal
        | IrBinOp::StrictEqual
        | IrBinOp::NotEqual
        | IrBinOp::StrictNotEqual
        | IrBinOp::LessThan
        | IrBinOp::LessThanOrEqual
        | IrBinOp::GreaterThan
        | IrBinOp::GreaterThanOrEqual
        | IrBinOp::In
        | IrBinOp::InstanceOf => Some(IrType::Bool),
        IrBinOp::LogicalOr | IrBinOp::LogicalAnd | IrBinOp::Unsupported => None,
    }
}

fn infer_conditional(consequent: &IrExpression, alternate: &IrExpression) -> Option<IrType> {
    let cons = infer_expression_type(consequent);
    let alt = infer_expression_type(alternate);
    match (cons, alt) {
        (Some(a), Some(b)) if a == b => Some(a),
        _ => None,
    }
}

fn infer_array_kind(elements: &[IrExpression]) -> IrArrayKind {
    if elements.is_empty() {
        return IrArrayKind::Unknown;
    }

    let mut kind = IrArrayKind::Unknown;
    for element in elements {
        match infer_expression_type(element) {
            Some(IrType::Number) => {
                kind = match kind {
                    IrArrayKind::Unknown | IrArrayKind::Number => IrArrayKind::Number,
                    _ => return IrArrayKind::Any,
                };
            }
            Some(IrType::Str) => {
                kind = match kind {
                    IrArrayKind::Unknown | IrArrayKind::Str => IrArrayKind::Str,
                    _ => return IrArrayKind::Any,
                };
            }
            Some(IrType::Bool) => {
                kind = match kind {
                    IrArrayKind::Unknown | IrArrayKind::Bool => IrArrayKind::Bool,
                    _ => return IrArrayKind::Any,
                };
            }
            Some(IrType::Value | IrType::Any) => return IrArrayKind::Any,
            Some(IrType::Array(_) | IrType::Unit) | None => return IrArrayKind::Any,
        }
    }

    match kind {
        IrArrayKind::Unknown => IrArrayKind::Any,
        other => other,
    }
}

fn infer_call_return(callee: &IrExpression) -> Option<IrType> {
    match callee {
        IrExpression::Identifier(name) => lookup_function_return(name),
        _ => None,
    }
}

fn infer_runtime(call: &RuntimeNamespace) -> Option<IrType> {
    match call {
        RuntimeNamespace::Console(ConsoleCall::Log(_)) => Some(IrType::Unit),
        RuntimeNamespace::Array(ArrayCall::Push { .. }) => Some(IrType::Number),
        RuntimeNamespace::Array(ArrayCall::Length { target }) => match infer_expression_type(target) {
            Some(IrType::Array(IrArrayKind::Number | IrArrayKind::Str | IrArrayKind::Bool)) => {
                Some(IrType::Number)
            }
            _ => Some(IrType::Value),
        },
        RuntimeNamespace::Array(ArrayCall::Index { element, target, .. }) => match element {
            Some(IrArrayKind::Number) => Some(IrType::Number),
            Some(IrArrayKind::Str) => Some(IrType::Str),
            Some(IrArrayKind::Bool) => Some(IrType::Bool),
            _ => match infer_expression_type(target) {
                Some(IrType::Array(kind)) => match kind {
                    IrArrayKind::Number => Some(IrType::Number),
                    IrArrayKind::Str => Some(IrType::Str),
                    IrArrayKind::Bool => Some(IrType::Bool),
                    _ => Some(IrType::Value),
                },
                _ => Some(IrType::Value),
            },
        },
        RuntimeNamespace::Array(ArrayCall::Pop { target, .. }) => match infer_expression_type(target) {
            Some(IrType::Array(kind)) => Some(match kind {
                IrArrayKind::Number => IrType::Number,
                IrArrayKind::Str => IrType::Str,
                IrArrayKind::Bool => IrType::Bool,
                IrArrayKind::Value => IrType::Value,
                IrArrayKind::Any | IrArrayKind::Unknown => IrType::Any,
            }),
            _ => Some(IrType::Value),
        },
        RuntimeNamespace::Array(ArrayCall::Map { target, .. })
        | RuntimeNamespace::Array(ArrayCall::Filter { target, .. }) => {
            match infer_expression_type(target) {
                Some(IrType::Array(kind)) => Some(IrType::Array(kind)),
                _ => Some(IrType::Array(IrArrayKind::Value)),
            }
        }
        RuntimeNamespace::Array(ArrayCall::Join { .. }) => Some(IrType::Str),
        RuntimeNamespace::Value(value_call) => match value_call {
            ValueCall::Coerce { expr } => infer_expression_type(expr).or(Some(IrType::Value)),
            ValueCall::Add { left, right } => {
                let left_ty = infer_expression_type(left);
                let right_ty = infer_expression_type(right);
                if left_ty == Some(IrType::Number) && right_ty == Some(IrType::Number) {
                    Some(IrType::Number)
                } else {
                    Some(IrType::Value)
                }
            }
            ValueCall::GetProperty { .. } => Some(IrType::Value),
            ValueCall::Sub { .. }
            | ValueCall::Mul { .. }
            | ValueCall::Div { .. }
            | ValueCall::Mod { .. } => Some(IrType::Value),
            ValueCall::Equal { .. }
            | ValueCall::StrictEqual { .. }
            | ValueCall::NotEqual { .. }
            | ValueCall::StrictNotEqual { .. }
            | ValueCall::LessThan { .. }
            | ValueCall::LessThanOrEqual { .. }
            | ValueCall::GreaterThan { .. }
            | ValueCall::GreaterThanOrEqual { .. }
            | ValueCall::LogicalNot { .. } => Some(IrType::Bool),
        },
        RuntimeNamespace::String(call) => match call {
            StringCall::Length { .. } => Some(IrType::Number),
            StringCall::ToUpperCase { .. }
            | StringCall::ToLowerCase { .. }
            | StringCall::Replace { .. }
            | StringCall::Concat { .. }
            | StringCall::Slice { .. }
            | StringCall::Substr { .. } => Some(IrType::Str),
            StringCall::Split { .. } => Some(IrType::Array(IrArrayKind::Str)),
            StringCall::Includes { .. } => Some(IrType::Bool),
        },
        RuntimeNamespace::Math(MathCall::Random) => Some(IrType::Number),
        RuntimeNamespace::Math(MathCall::Sqrt { .. }) => Some(IrType::Number),
    }
}

fn unify(current: &mut Option<IrType>, new_ty: Option<IrType>) -> bool {
    match new_ty {
        Some(ty) => {
            if let Some(existing) = current {
                *existing == ty
            } else {
                *current = Some(ty);
                true
            }
        }
        None => false,
    }
}

#[allow(dead_code)]
pub fn infer_template_type(parts: &[IrTemplatePart]) -> Option<IrType> {
    if parts.is_empty() {
        return Some(IrType::Str);
    }

    let mut contains_expr = false;
    for part in parts {
        if let IrTemplatePart::Expr(expr) = part {
            contains_expr = true;
            if infer_expression_type(expr).is_none() {
                return None;
            }
        }
    }

    if contains_expr {
        Some(IrType::Str)
    } else {
        Some(IrType::Str)
    }
}

pub fn coerce_to_type(
    expr_tokens: TokenStream,
    target: &IrType,
    expr_type: Option<IrType>,
) -> TokenStream {
    match target {
        IrType::Number => {
            if matches!(expr_type, Some(IrType::Number)) {
                expr_tokens
            } else {
                quote! { runtime::value::into_value(#expr_tokens).into_number() }
            }
        }
        IrType::Str => {
            if matches!(expr_type, Some(IrType::Str)) {
                expr_tokens
            } else {
                quote! { runtime::console::stringify(& runtime::value::into_value(#expr_tokens)) }
            }
        }
        IrType::Bool => {
            if matches!(expr_type, Some(IrType::Bool)) {
                expr_tokens
            } else {
                quote! { !runtime::value::ops::logical_not(#expr_tokens) }
            }
        }
        IrType::Unit => quote!({ #expr_tokens; () }),
        IrType::Array(_) | IrType::Any | IrType::Value => expr_tokens,
    }
}
