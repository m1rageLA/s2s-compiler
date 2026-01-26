use std::cell::RefCell;
use std::collections::HashMap;

use ir::{
    ArrayCall, ConsoleCall, IrArrayKind, IrArrowBody, IrBinOp, IrExpression, IrLiteral, IrParam,
    IrStmt, IrTemplatePart, IrType, IrTypeAlias, IrTypeAliasDef, MathCall, RuntimeNamespace,
    StringCall, ValueCall,
};
use proc_macro2::TokenStream;
use quote::quote;

thread_local! {
    static TYPE_STACK: RefCell<Vec<HashMap<String, IrType>>> = RefCell::new(vec![HashMap::new()]);
    static FN_RETURNS: RefCell<Vec<HashMap<String, IrType>>> = RefCell::new(vec![HashMap::new()]);
    static FN_PARAMS: RefCell<Vec<HashMap<String, Vec<IrType>>>> = RefCell::new(vec![HashMap::new()]);
    static FN_PARAM_PASSES: RefCell<Vec<HashMap<String, Vec<ParamPass>>>> =
        RefCell::new(vec![HashMap::new()]);
    static BINDING_PASSES: RefCell<Vec<HashMap<String, ParamPass>>> = RefCell::new(vec![HashMap::new()]);
    static OBJECT_ALIASES: RefCell<Vec<HashMap<String, ObjectAlias>>> = RefCell::new(vec![HashMap::new()]);
    static ARRAY_INDEX_ALIASES: RefCell<Vec<HashMap<(String, String), String>>> =
        RefCell::new(vec![HashMap::new()]);
    static RETURN_STACK: RefCell<Vec<IrType>> = RefCell::new(vec![]);
    static TYPE_ALIASES: RefCell<Vec<HashMap<u32, IrTypeAlias>>> = RefCell::new(vec![HashMap::new()]);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamPass {
    Value,
    Ref,
    MutRef,
}

#[derive(Debug, Clone)]
pub struct ObjectAlias {
    pub target: IrExpression,
    pub index: IrExpression,
    pub element: Option<IrArrayKind>,
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
    FN_PARAMS.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    FN_PARAM_PASSES.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    BINDING_PASSES.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    OBJECT_ALIASES.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    ARRAY_INDEX_ALIASES.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    RETURN_STACK.with(|stack| stack.borrow_mut().clear());
    TYPE_ALIASES.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
}

pub fn push_scope() {
    TYPE_STACK.with(|stack| stack.borrow_mut().push(HashMap::new()));
    FN_RETURNS.with(|stack| stack.borrow_mut().push(HashMap::new()));
    FN_PARAMS.with(|stack| stack.borrow_mut().push(HashMap::new()));
    FN_PARAM_PASSES.with(|stack| stack.borrow_mut().push(HashMap::new()));
    BINDING_PASSES.with(|stack| stack.borrow_mut().push(HashMap::new()));
    OBJECT_ALIASES.with(|stack| stack.borrow_mut().push(HashMap::new()));
    ARRAY_INDEX_ALIASES.with(|stack| stack.borrow_mut().push(HashMap::new()));
    TYPE_ALIASES.with(|stack| stack.borrow_mut().push(HashMap::new()));
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
    FN_PARAMS.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
    FN_PARAM_PASSES.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
    BINDING_PASSES.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
    OBJECT_ALIASES.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
    ARRAY_INDEX_ALIASES.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
    TYPE_ALIASES.with(|stack| {
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

pub fn define_binding_pass(name: &str, pass: ParamPass) {
    BINDING_PASSES.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), pass);
        }
    });
}

pub fn define_object_alias(name: &str, alias: ObjectAlias) {
    OBJECT_ALIASES.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), alias);
        }
    });
}

pub fn lookup_object_alias(name: &str) -> Option<ObjectAlias> {
    OBJECT_ALIASES.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(alias) = scope.get(name) {
                return Some(alias.clone());
            }
        }
        None
    })
}

pub fn define_array_index_alias(array: &str, index: &str, alias: &str) {
    ARRAY_INDEX_ALIASES.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert((array.to_string(), index.to_string()), alias.to_string());
        }
    });
}

pub fn lookup_array_index_alias(array: &str, index: &str) -> Option<String> {
    ARRAY_INDEX_ALIASES.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(alias) = scope.get(&(array.to_string(), index.to_string())) {
                return Some(alias.clone());
            }
        }
        None
    })
}

pub fn object_alias_is_copy(id: u32) -> bool {
    lookup_type_alias(id)
        .and_then(|alias| match alias.def {
            IrTypeAliasDef::Object(fields) => Some(
                fields
                    .iter()
                    .all(|field| is_copy_type(&field.ty)),
            ),
            _ => None,
        })
        .unwrap_or(false)
}

pub fn define_function_return(name: &str, ty: IrType) {
    FN_RETURNS.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), ty);
        }
    });
}

pub fn define_function_params(name: &str, params: &[IrParam]) {
    FN_PARAMS.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), params.iter().map(|p| p.ty).collect());
        }
    });
}

pub fn define_function_param_passes(name: &str, passes: &[ParamPass]) {
    FN_PARAM_PASSES.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), passes.to_vec());
        }
    });
}

pub fn define_type_alias(alias: &IrTypeAlias) {
    TYPE_ALIASES.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(alias.id, alias.clone());
        }
    });
}

pub fn lookup_type_alias(id: u32) -> Option<IrTypeAlias> {
    TYPE_ALIASES.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(alias) = scope.get(&id) {
                return Some(alias.clone());
            }
        }
        None
    })
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

pub fn lookup_function_params(name: &str) -> Option<Vec<IrType>> {
    FN_PARAMS.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(params) = scope.get(name) {
                return Some(params.clone());
            }
        }
        None
    })
}

pub fn lookup_function_param_passes(name: &str) -> Option<Vec<ParamPass>> {
    FN_PARAM_PASSES.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(passes) = scope.get(name) {
                return Some(passes.clone());
            }
        }
        None
    })
}

pub fn lookup_binding_pass(name: &str) -> Option<ParamPass> {
    BINDING_PASSES.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(pass) = scope.get(name) {
                return Some(*pass);
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
            IrLiteral::Number(value) => Some(infer_number_literal(*value)),
            IrLiteral::Str(_) => Some(IrType::Str),
            IrLiteral::Bool(_) => Some(IrType::Bool),
            IrLiteral::Null => Some(IrType::Value),
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
        IrExpression::Member { object, property } => {
            if let Some(IrType::Object(id)) = infer_expression_type(object) {
                if let Some(alias) = lookup_type_alias(id) {
                    if let IrTypeAliasDef::Object(fields) = alias.def {
                        if let Some(field) = fields.iter().find(|field| field.name == *property) {
                            return Some(field.ty);
                        }
                    }
                }
            }
            None
        }
        IrExpression::SuperCall { .. } => None,
        IrExpression::ArrayExpr(elements) => Some(IrType::Array(infer_array_kind(elements))),
        IrExpression::PostfixUnary { left, .. } => infer_expression_type(left),
        IrExpression::Paren(inner) => infer_expression_type(inner),
        IrExpression::PrefixUnary { arg, .. } => infer_expression_type(arg),
        IrExpression::Unary { op, expr } => match op {
            ir::IrUnaryOp::TypeOf => Some(IrType::Str),
            ir::IrUnaryOp::Void => Some(IrType::Value),
            ir::IrUnaryOp::BitwiseNot => Some(IrType::Number),
        }
        .or_else(|| infer_expression_type(expr)),
        IrExpression::Sequence(exprs) => exprs.last().and_then(|expr| infer_expression_type(expr)),
        IrExpression::Delete(_) => Some(IrType::Bool),
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
            IrStmt::Switch { cases, .. } => {
                for case in cases {
                    if let Some(case_ty) = infer_return_types(&case.consequent) {
                        if !unify(&mut inferred, Some(case_ty)) {
                            return None;
                        }
                        saw_return = true;
                    }
                }
            }
            IrStmt::Try {
                try_block,
                catch,
                finally,
            } => {
                if let Some(try_ty) = infer_return_types(try_block) {
                    if !unify(&mut inferred, Some(try_ty)) {
                        return None;
                    }
                    saw_return = true;
                }
                if let Some(catch) = catch {
                    if let Some(catch_ty) = infer_return_types(&catch.body) {
                        if !unify(&mut inferred, Some(catch_ty)) {
                            return None;
                        }
                        saw_return = true;
                    }
                }
                if let Some(finally) = finally {
                    if let Some(finally_ty) = infer_return_types(finally) {
                        if !unify(&mut inferred, Some(finally_ty)) {
                            return None;
                        }
                        saw_return = true;
                    }
                }
            }
            IrStmt::While(_, body)
            | IrStmt::DoWhile(body, _)
            | IrStmt::For { body, .. }
            | IrStmt::ForIn { body, .. } => {
                if let Some(body_ty) = infer_return_types(body) {
                    if !unify(&mut inferred, Some(body_ty)) {
                        return None;
                    }
                    saw_return = true;
                }
            }
            IrStmt::Labeled { body, .. } => {
                if let Some(label_ty) = infer_return_types(std::slice::from_ref(body)) {
                    if !unify(&mut inferred, Some(label_ty)) {
                        return None;
                    }
                    saw_return = true;
                }
            }
            _ => {}
        }
    }

    if saw_return { inferred } else { None }
}

fn infer_binary(op: IrBinOp, left: &IrExpression, right: &IrExpression) -> Option<IrType> {
    let left_ty = infer_expression_type(left);
    let right_ty = infer_expression_type(right);
    let left_numeric = matches!(left_ty, Some(IrType::Number | IrType::UInt));
    let right_numeric = matches!(right_ty, Some(IrType::Number | IrType::UInt));
    let both_uint = matches!(left_ty, Some(IrType::UInt)) && matches!(right_ty, Some(IrType::UInt));

    match op {
        IrBinOp::Add => {
            if left_ty == Some(IrType::Str) || right_ty == Some(IrType::Str) {
                Some(IrType::Str)
            } else if left_ty == Some(IrType::Bool) || right_ty == Some(IrType::Bool) {
                None
            } else if left_numeric && right_numeric {
                if both_uint {
                    Some(IrType::UInt)
                } else {
                    Some(IrType::Number)
                }
            } else if left_numeric || right_numeric || (left_ty.is_none() && right_ty.is_none()) {
                Some(IrType::Number)
            } else {
                None
            }
        }
        IrBinOp::Sub
        | IrBinOp::Div
        | IrBinOp::Exp
        | IrBinOp::LeftShift
        | IrBinOp::RightShift
        | IrBinOp::BitwiseOr
        | IrBinOp::BitwiseXor
        | IrBinOp::BitwiseAnd
        | IrBinOp::UnsignedRightShift => Some(IrType::Number),
        IrBinOp::Mul => {
            if both_uint {
                Some(IrType::UInt)
            } else {
                Some(IrType::Number)
            }
        }
        IrBinOp::Mod => {
            if left_numeric && right_numeric && both_uint {
                Some(IrType::UInt)
            } else {
                Some(IrType::Number)
            }
        }
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

pub(crate) fn infer_array_kind(elements: &[IrExpression]) -> IrArrayKind {
    if elements.is_empty() {
        return IrArrayKind::Unknown;
    }

    let mut kind = IrArrayKind::Unknown;
    for element in elements {
        match infer_expression_type(element) {
            Some(IrType::Number | IrType::UInt) => {
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
            Some(IrType::Object(id)) => {
                kind = match kind {
                    IrArrayKind::Unknown => IrArrayKind::Object(id),
                    IrArrayKind::Object(existing) if existing == id => IrArrayKind::Object(id),
                    _ => return IrArrayKind::Any,
                };
            }
            Some(IrType::Value | IrType::Any) => return IrArrayKind::Any,
            Some(IrType::Array(_) | IrType::Unit) | None => {
                return IrArrayKind::Any
            }
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
        RuntimeNamespace::Array(ArrayCall::Push { target, .. }) => match infer_expression_type(target) {
            Some(IrType::Array(
                IrArrayKind::Number | IrArrayKind::Str | IrArrayKind::Bool | IrArrayKind::Object(_),
            )) => {
                Some(IrType::UInt)
            }
            _ => Some(IrType::Value),
        },
        RuntimeNamespace::Array(ArrayCall::Length { target }) => match infer_expression_type(target) {
            Some(IrType::Array(
                IrArrayKind::Number | IrArrayKind::Str | IrArrayKind::Bool | IrArrayKind::Object(_),
            )) => {
                Some(IrType::UInt)
            }
            _ => Some(IrType::Value),
        },
        RuntimeNamespace::Array(ArrayCall::Index { element, target, .. }) => match element {
            Some(IrArrayKind::Number) => Some(IrType::Number),
            Some(IrArrayKind::Str) => Some(IrType::Str),
            Some(IrArrayKind::Bool) => Some(IrType::Bool),
            Some(IrArrayKind::Object(id)) => Some(IrType::Object(*id)),
            _ => match infer_expression_type(target) {
                Some(IrType::Array(kind)) => match kind {
                    IrArrayKind::Number => Some(IrType::Number),
                    IrArrayKind::Str => Some(IrType::Str),
                    IrArrayKind::Bool => Some(IrType::Bool),
                    IrArrayKind::Object(id) => Some(IrType::Object(id)),
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
                IrArrayKind::Object(id) => IrType::Object(id),
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
            ValueCall::Coerce { .. } => Some(IrType::Value),
            ValueCall::Add { left, right } => {
                let left_ty = infer_expression_type(left);
                let right_ty = infer_expression_type(right);
                let numeric = matches!(left_ty, Some(IrType::Number | IrType::UInt))
                    && matches!(right_ty, Some(IrType::Number | IrType::UInt));
                if numeric {
                    Some(IrType::Number)
                } else {
                    Some(IrType::Value)
                }
            }
            ValueCall::GetProperty { .. } => Some(IrType::Value),
            ValueCall::GetPropertyDynamic { .. } => Some(IrType::Value),
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
            StringCall::Length { .. } => Some(IrType::UInt),
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
                if *existing == ty {
                    true
                } else if numeric_pair(*existing, ty) {
                    *current = Some(IrType::Number);
                    true
                } else {
                    false
                }
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
            } else if matches!(expr_type, Some(IrType::UInt)) {
                quote! { (#expr_tokens) as f64 }
            } else {
                quote! { runtime::value::into_value(#expr_tokens).into_number() }
            }
        }
        IrType::UInt => {
            if matches!(expr_type, Some(IrType::UInt)) {
                expr_tokens
            } else if matches!(expr_type, Some(IrType::Number)) {
                quote! { (#expr_tokens) as usize }
            } else {
                quote! { runtime::value::into_value(#expr_tokens).to_number() as usize }
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
        IrType::Array(_) | IrType::Any | IrType::Object(_) => expr_tokens,
        IrType::Value => {
            if matches!(expr_type, Some(IrType::Value)) {
                expr_tokens
            } else {
                quote! { runtime::value::into_value(#expr_tokens) }
            }
        }
    }
}

pub fn is_copy_type(ty: &IrType) -> bool {
    matches!(ty, IrType::Number | IrType::UInt | IrType::Bool | IrType::Unit)
}

pub fn expr_is_copy_type(expr: &IrExpression) -> bool {
    infer_expression_type(expr)
        .as_ref()
        .map(is_copy_type)
        .unwrap_or(false)
}

fn numeric_pair(a: IrType, b: IrType) -> bool {
    matches!(
        (a, b),
        (IrType::Number, IrType::UInt) | (IrType::UInt, IrType::Number)
    )
}

fn infer_number_literal(value: f64) -> IrType {
    if is_non_negative_int(value) {
        IrType::UInt
    } else {
        IrType::Number
    }
}

fn is_non_negative_int(value: f64) -> bool {
    value.is_finite() && value >= 0.0 && value.fract() == 0.0
}
