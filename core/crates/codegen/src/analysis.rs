use ir::{
    ArrayCall, ConsoleCall, IrArrowBody, IrExpression, IrParam, IrStmt, IrType, RuntimeNamespace,
    StringCall, ValueCall,
};

use crate::typing;
use crate::typing::ParamPass;

#[derive(Debug, Clone, Copy)]
pub struct ParamUsage {
    pub pass: ParamPass,
    pub mutated: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct Usage {
    mutated: bool,
    escapes: bool,
}

impl Usage {
    fn combine(self, other: Usage) -> Usage {
        Usage {
            mutated: self.mutated || other.mutated,
            escapes: self.escapes || other.escapes,
        }
    }
}

pub fn infer_param_usages(params: &[IrParam], body: &[IrStmt]) -> Vec<ParamUsage> {
    params
        .iter()
        .map(|param| {
            let usage = usage_in_stmts(&param.name, body);
            param_usage_from(param, usage)
        })
        .collect()
}

pub fn infer_param_usages_for_arrow(params: &[IrParam], body: &IrArrowBody) -> Vec<ParamUsage> {
    params
        .iter()
        .map(|param| {
            let usage = match body {
                IrArrowBody::Expr(expr) => usage_in_expr(&param.name, expr.as_ref()),
                IrArrowBody::Block(stmts) => usage_in_stmts(&param.name, stmts),
            };
            param_usage_from(param, usage)
        })
        .collect()
}

fn param_usage_from(param: &IrParam, usage: Usage) -> ParamUsage {
    if !matches!(param.ty, IrType::Array(_)) {
        return ParamUsage {
            pass: ParamPass::Value,
            mutated: usage.mutated,
        };
    }

    let pass = if matches!(param.ty, IrType::Array(ir::IrArrayKind::Object(_))) {
        if usage.escapes {
            ParamPass::Value
        } else {
            ParamPass::MutRef
        }
    } else if usage.escapes {
        ParamPass::Value
    } else if usage.mutated {
        ParamPass::MutRef
    } else {
        ParamPass::Ref
    };

    ParamUsage {
        pass,
        mutated: usage.mutated,
    }
}

fn usage_in_stmts(name: &str, stmts: &[IrStmt]) -> Usage {
    stmts.iter().fold(Usage::default(), |acc, stmt| {
        acc.combine(usage_in_stmt(name, stmt))
    })
}

fn usage_in_stmt(name: &str, stmt: &IrStmt) -> Usage {
    match stmt {
        IrStmt::Leteral(var) => {
            let mut usage = usage_in_expr_opt(name, var.value.as_ref());
            if var.name == name {
                usage.escapes = true;
            }
            usage
        }
        IrStmt::Expression(expr) => usage_in_expr(name, expr),
        IrStmt::Return(expr) => usage_in_expr_opt(name, expr.as_ref()),
        IrStmt::Block(stmts) => usage_in_stmts(name, stmts),
        IrStmt::If {
            condition,
            then_branch,
            else_branch,
        } => usage_in_expr(name, condition)
            .combine(usage_in_stmts(name, then_branch))
            .combine(usage_in_stmts(name, else_branch.as_deref().unwrap_or(&[]))),
        IrStmt::While(condition, body) => {
            usage_in_expr(name, condition).combine(usage_in_stmts(name, body))
        }
        IrStmt::DoWhile(body, condition) => {
            usage_in_stmts(name, body).combine(usage_in_expr(name, condition))
        }
        IrStmt::For {
            init,
            condition,
            update,
            body,
        } => usage_in_for_init(name, init.as_ref())
            .combine(usage_in_expr_opt(name, condition.as_ref()))
            .combine(usage_in_expr_opt(name, update.as_ref()))
            .combine(usage_in_stmts(name, body)),
        IrStmt::ForIn { left, right, body } => usage_in_for_in_left(name, left)
            .combine(usage_in_expr(name, right))
            .combine(usage_in_stmts(name, body)),
        IrStmt::Switch {
            discriminant,
            cases,
        } => cases
            .iter()
            .fold(usage_in_expr(name, discriminant), |acc, case| {
                acc.combine(usage_in_expr_opt(name, case.test.as_ref()))
                    .combine(usage_in_stmts(name, &case.consequent))
            }),
        IrStmt::Try {
            try_block,
            catch,
            finally,
        } => usage_in_stmts(name, try_block)
            .combine(match catch {
                Some(catch) => usage_in_stmts(name, &catch.body),
                None => Usage::default(),
            })
            .combine(usage_in_stmts(name, finally.as_deref().unwrap_or(&[]))),
        IrStmt::Throw(expr) => usage_in_expr(name, expr),
        IrStmt::VarDecl(vars) => vars.iter().fold(Usage::default(), |acc, var| {
            acc.combine(usage_in_expr_opt(name, var.value.as_ref()))
        }),
        IrStmt::Labeled { body, .. } => usage_in_stmt(name, body.as_ref()),
        IrStmt::TypeAlias(_) => Usage::default(),
        IrStmt::Break(_) | IrStmt::Continue(_) | IrStmt::Empty | IrStmt::Unsupported(_) => {
            Usage::default()
        }
    }
}

fn usage_in_for_init(name: &str, init: Option<&ir::IrForInit>) -> Usage {
    match init {
        Some(ir::IrForInit::VarDecl(vars)) => vars.iter().fold(Usage::default(), |acc, var| {
            acc.combine(usage_in_expr_opt(name, var.value.as_ref()))
        }),
        Some(ir::IrForInit::Expr(expr)) => usage_in_expr(name, expr),
        None => Usage::default(),
    }
}

fn usage_in_for_in_left(name: &str, left: &ir::IrForInLeft) -> Usage {
    match left {
        ir::IrForInLeft::Var(var) => usage_in_expr_opt(name, var.value.as_ref()),
        ir::IrForInLeft::Identifier(ident) => {
            if ident == name {
                Usage {
                    escapes: true,
                    mutated: false,
                }
            } else {
                Usage::default()
            }
        }
        ir::IrForInLeft::Pattern(expr) => usage_in_expr(name, expr),
    }
}

fn usage_in_expr_opt(name: &str, expr: Option<&IrExpression>) -> Usage {
    match expr {
        Some(expr) => usage_in_expr(name, expr),
        None => Usage::default(),
    }
}

fn usage_in_expr(name: &str, expr: &IrExpression) -> Usage {
    match expr {
        IrExpression::Identifier(ident) => {
            if ident == name {
                Usage {
                    escapes: true,
                    mutated: false,
                }
            } else {
                Usage::default()
            }
        }
        IrExpression::Literal(_) => Usage::default(),
        IrExpression::Binary { left, right, .. } => {
            usage_in_expr(name, left).combine(usage_in_expr(name, right))
        }
        IrExpression::Assignment { left, right, .. } => {
            let mut usage = usage_in_expr(name, right);
            if array_index_target_matches(name, left.as_ref()) {
                usage.mutated = true;
            } else if matches!(left.as_ref(), IrExpression::Identifier(ident) if ident == name) {
                usage.escapes = true;
            } else {
                usage = usage.combine(usage_in_expr(name, left));
            }
            usage
        }
        IrExpression::Call { callee, args } => {
            let mut usage = usage_in_expr(name, callee);

            let callee = strip_paren(callee);
            let param_passes = match callee {
                IrExpression::Identifier(callee_name) => {
                    typing::lookup_function_param_passes(callee_name)
                }
                IrExpression::Arrow { params, body } => Some(
                    infer_param_usages_for_arrow(params, body)
                        .iter()
                        .map(|usage| usage.pass)
                        .collect(),
                ),
                IrExpression::Function(func) => Some(
                    infer_param_usages(&func.params, &func.body)
                        .iter()
                        .map(|usage| usage.pass)
                        .collect(),
                ),
                _ => None,
            };

            for (idx, arg) in args.iter().enumerate() {
                let arg = strip_paren(arg);
                let mut arg_usage = usage_in_expr(name, arg);

                if let Some(passes) = param_passes.as_ref() {
                    if matches!(passes.get(idx), Some(ParamPass::MutRef))
                        && (target_is_name(name, arg) || array_index_target_matches(name, arg))
                    {
                        arg_usage.mutated = true;
                        arg_usage.escapes = false;
                    }
                }

                usage = usage.combine(arg_usage);
            }

            usage
        }
        IrExpression::Array(items)
        | IrExpression::ArrayExpr(items)
        | IrExpression::Sequence(items) => items.iter().fold(Usage::default(), |acc, expr| {
            acc.combine(usage_in_expr(name, expr))
        }),
        IrExpression::Arrow { params, body } => {
            if params.iter().any(|param| param.name == name) {
                Usage::default()
            } else {
                match body {
                    IrArrowBody::Expr(expr) => usage_in_expr(name, expr.as_ref()),
                    IrArrowBody::Block(stmts) => usage_in_stmts(name, stmts),
                }
            }
        }
        IrExpression::RuntimeCall(call) => usage_in_runtime_call(name, call),
        IrExpression::Member { object, .. } => usage_in_expr(name, object),
        IrExpression::Delete(target) => match target {
            ir::IrDeleteTarget::Property { object, property } => usage_in_expr(name, object)
                .combine(match property {
                    ir::IrDeleteProperty::Dynamic(expr) => usage_in_expr(name, expr.as_ref()),
                    _ => Usage::default(),
                }),
            ir::IrDeleteTarget::Expr(expr) => usage_in_expr(name, expr.as_ref()),
        },
        IrExpression::Template(parts) => {
            parts.iter().fold(Usage::default(), |acc, part| match part {
                ir::IrTemplatePart::String(_) => acc,
                ir::IrTemplatePart::Expr(expr) => acc.combine(usage_in_expr(name, expr)),
            })
        }
        IrExpression::SuperCall { args } => args.iter().fold(Usage::default(), |acc, expr| {
            acc.combine(usage_in_expr(name, expr))
        }),
        IrExpression::Conditional {
            test,
            consequent,
            alternate,
        } => usage_in_expr(name, test)
            .combine(usage_in_expr(name, consequent))
            .combine(usage_in_expr(name, alternate)),
        IrExpression::Function(func) => {
            if func.params.iter().any(|param| param.name == name) {
                Usage::default()
            } else {
                usage_in_stmts(name, &func.body)
            }
        }
        IrExpression::PostfixUnary { left, .. } => {
            if array_index_target_matches(name, left.as_ref()) {
                Usage {
                    mutated: true,
                    escapes: false,
                }
            } else {
                usage_in_expr(name, left)
            }
        }
        IrExpression::PrefixUnary { arg, .. } => {
            if array_index_target_matches(name, arg.as_ref()) {
                Usage {
                    mutated: true,
                    escapes: false,
                }
            } else {
                usage_in_expr(name, arg)
            }
        }
        IrExpression::Unary { expr, .. } | IrExpression::Paren(expr) => usage_in_expr(name, expr),
        IrExpression::Object(properties) => {
            properties.iter().fold(Usage::default(), |acc, prop| {
                acc.combine(usage_in_expr(name, &prop.value))
            })
        }
    }
}

fn strip_paren(expr: &IrExpression) -> &IrExpression {
    match expr {
        IrExpression::Paren(inner) => strip_paren(inner),
        _ => expr,
    }
}

fn usage_in_runtime_call(name: &str, call: &RuntimeNamespace) -> Usage {
    match call {
        RuntimeNamespace::Array(array_call) => match array_call {
            ArrayCall::Length { target } => usage_in_expr_target(name, target),
            ArrayCall::Index { target, index, .. } => {
                usage_in_expr_target(name, target).combine(usage_in_expr(name, index))
            }
            ArrayCall::Push { target, args } | ArrayCall::Pop { target, args } => {
                let mut usage = usage_in_expr_target(name, target);
                if target_is_name(name, target) {
                    usage.mutated = true;
                }
                args.iter()
                    .fold(usage, |acc, expr| acc.combine(usage_in_expr(name, expr)))
            }
            ArrayCall::Map { target, callback } | ArrayCall::Filter { target, callback } => {
                usage_in_expr_target(name, target).combine(usage_in_expr(name, callback))
            }
            ArrayCall::Join { target, separator } => usage_in_expr_target(name, target)
                .combine(usage_in_expr_opt(name, separator.as_deref())),
        },
        RuntimeNamespace::Console(ConsoleCall::Log(args)) => {
            args.iter().fold(Usage::default(), |acc, expr| {
                acc.combine(usage_in_expr(name, expr))
            })
        }
        RuntimeNamespace::String(string_call) => usage_in_string_call(name, string_call),
        RuntimeNamespace::Math(math_call) => match math_call {
            ir::MathCall::Random => Usage::default(),
            ir::MathCall::Sqrt { arg } => usage_in_expr(name, arg),
        },
        RuntimeNamespace::Value(value_call) => usage_in_value_call(name, value_call),
    }
}

fn usage_in_expr_target(name: &str, target: &IrExpression) -> Usage {
    if target_is_name(name, target) {
        Usage::default()
    } else {
        usage_in_expr(name, target)
    }
}

fn target_is_name(name: &str, target: &IrExpression) -> bool {
    matches!(target, IrExpression::Identifier(ident) if ident == name)
}

fn array_index_target_matches(name: &str, expr: &IrExpression) -> bool {
    match expr {
        IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index { target, .. })) => {
            target_is_name(name, target.as_ref())
        }
        IrExpression::Member { object, .. } => array_index_target_matches(name, object),
        IrExpression::Paren(inner) => array_index_target_matches(name, inner),
        _ => false,
    }
}

fn usage_in_string_call(name: &str, call: &StringCall) -> Usage {
    match call {
        StringCall::Length { target }
        | StringCall::ToUpperCase { target }
        | StringCall::ToLowerCase { target } => usage_in_expr(name, target),
        StringCall::Split {
            target,
            separator,
            limit,
        } => usage_in_expr(name, target)
            .combine(usage_in_expr_opt(name, separator.as_deref()))
            .combine(usage_in_expr_opt(name, limit.as_deref())),
        StringCall::Replace {
            target,
            pattern,
            replacement,
        } => usage_in_expr(name, target)
            .combine(usage_in_expr(name, pattern))
            .combine(usage_in_expr(name, replacement)),
        StringCall::Includes {
            target,
            search,
            position,
        } => usage_in_expr(name, target)
            .combine(usage_in_expr(name, search))
            .combine(usage_in_expr_opt(name, position.as_deref())),
        StringCall::Concat { target, args } => {
            args.iter().fold(usage_in_expr(name, target), |acc, expr| {
                acc.combine(usage_in_expr(name, expr))
            })
        }
        StringCall::Slice { target, start, end } => usage_in_expr(name, target)
            .combine(usage_in_expr_opt(name, start.as_deref()))
            .combine(usage_in_expr_opt(name, end.as_deref())),
        StringCall::Substr {
            target,
            start,
            length,
        } => usage_in_expr(name, target)
            .combine(usage_in_expr_opt(name, start.as_deref()))
            .combine(usage_in_expr_opt(name, length.as_deref())),
    }
}

fn usage_in_value_call(name: &str, call: &ValueCall) -> Usage {
    match call {
        ValueCall::Coerce { expr } => usage_in_expr(name, expr),
        ValueCall::Add { left, right }
        | ValueCall::Sub { left, right }
        | ValueCall::Mul { left, right }
        | ValueCall::Div { left, right }
        | ValueCall::Mod { left, right }
        | ValueCall::Equal { left, right }
        | ValueCall::StrictEqual { left, right }
        | ValueCall::NotEqual { left, right }
        | ValueCall::StrictNotEqual { left, right }
        | ValueCall::LessThan { left, right }
        | ValueCall::LessThanOrEqual { left, right }
        | ValueCall::GreaterThan { left, right }
        | ValueCall::GreaterThanOrEqual { left, right } => {
            usage_in_expr(name, left).combine(usage_in_expr(name, right))
        }
        ValueCall::LogicalNot { expr } => usage_in_expr(name, expr),
        ValueCall::GetProperty { target, .. } => usage_in_expr(name, target),
        ValueCall::GetPropertyDynamic { target, property } => {
            usage_in_expr(name, target).combine(usage_in_expr(name, property))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrArrayKind, IrAssignOp, IrExpression, IrLiteral, IrParam};

    #[test]
    fn marks_array_index_assignment_as_mutable() {
        let params = vec![IrParam {
            name: "b".into(),
            ty: IrType::Array(IrArrayKind::Number),
        }];

        let left = IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index {
            target: Box::new(IrExpression::Identifier("b".into())),
            index: Box::new(IrExpression::Literal(IrLiteral::Number(0.0))),
            element: Some(IrArrayKind::Number),
        }));

        let stmt = IrStmt::Expression(IrExpression::Assignment {
            op: IrAssignOp::SubAssign,
            left: Box::new(left),
            right: Box::new(IrExpression::Literal(IrLiteral::Number(1.0))),
        });

        let usages = infer_param_usages(&params, &[stmt]);
        assert_eq!(usages.len(), 1);
        assert_eq!(usages[0].pass, ParamPass::MutRef);
    }
}
