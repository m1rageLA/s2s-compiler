use ir::{
    ConsoleCall, IrBinOp, IrExpression, IrLiteral, IrPostfixOp, IrStmt, IrType, RuntimeNamespace
};

pub(crate) fn infer_function_return_type(body: &[IrStmt]) -> Option<IrType> {
    let mut inferred: Option<IrType> = None;
    let mut saw_return = false;

    if !collect_return_types(body, &mut inferred, &mut saw_return) {
        return None;
    }

    if saw_return {
        inferred
    } else {
        Some(IrType::Unit)
    }
}

fn collect_return_types(
    stmts: &[IrStmt],
    inferred: &mut Option<IrType>,
    saw_return: &mut bool,
) -> bool {
    for stmt in stmts {
        match stmt {
            IrStmt::Return(Some(expr)) => {
                *saw_return = true;
                let ty = match infer_expression_type(expr) {
                    Some(value) => value,
                    None => return false,
                };
                if !unify_type(inferred, ty) {
                    return false;
                }
            }
            IrStmt::Return(None) => {
                *saw_return = true;
                if !unify_type(inferred, IrType::Unit) {
                    return false;
                }
            }
            IrStmt::Block(inner) => {
                if !collect_return_types(inner, inferred, saw_return) {
                    return false;
                }
            }
            IrStmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                if !collect_return_types(then_branch, inferred, saw_return) {
                    return false;
                }
                if let Some(else_branch) = else_branch {
                    if !collect_return_types(else_branch, inferred, saw_return) {
                        return false;
                    }
                }
            }
            IrStmt::While(_, body) => {
                if !collect_return_types(body, inferred, saw_return) {
                    return false;
                }
            }
            IrStmt::DoWhile(body, _) => {
                if !collect_return_types(body, inferred, saw_return) {
                    return false;
                }
            }
            IrStmt::For { body, .. } => {
                if !collect_return_types(body, inferred, saw_return) {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

fn unify_type(current: &mut Option<IrType>, new_ty: IrType) -> bool {
    if let Some(existing) = current {
        *existing == new_ty
    } else {
        *current = Some(new_ty);
        true
    }
}

fn infer_expression_type(expr: &IrExpression) -> Option<IrType> {
    match expr {
        IrExpression::Literal(IrLiteral::Number(_)) => Some(IrType::Number),
        IrExpression::Literal(IrLiteral::Str(_)) => Some(IrType::Str),
        IrExpression::Literal(IrLiteral::Bool(_)) => Some(IrType::Bool),
        IrExpression::Identifier(name) if name == "undefined" => Some(IrType::Unit),
        IrExpression::Identifier(_) => None,
        IrExpression::ArrayExpr(_) => None,
        IrExpression::PostfixUnary { left, op } => None, 
        IrExpression::Binary { op, left, right } => match op {
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
        },
        IrExpression::Conditional {
            consequent,
            alternate,
            ..
        } => {
            let consequent_ty = infer_expression_type(consequent)?;
            let alternate_ty = infer_expression_type(alternate)?;
            if consequent_ty == alternate_ty {
                Some(consequent_ty)
            } else {
                None
            }
        }
        IrExpression::Template(_) => Some(IrType::Str),
        IrExpression::RuntimeCall(RuntimeNamespace::Console(ConsoleCall::Log(_))) => {
            Some(IrType::Unit)
        }
        IrExpression::Array(_) => None,
        IrExpression::Arrow { .. } => None,
        IrExpression::Call { .. } => None,
        IrExpression::Member { .. } => None,
        IrExpression::SuperCall { .. } => None,
    }
}
    
