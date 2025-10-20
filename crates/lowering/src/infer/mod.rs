use ir::{IrExpression, IrType};

mod expression_binary;
mod expression_conditional;
mod expression_identifier;
mod expression_literal;
mod expression_runtime;
mod expression_template;
mod expression_trivial;
mod function;
mod statements;
mod unify;

pub(crate) use function::infer_function_return_type;
pub(crate) use statements::collect_return_types;
pub(crate) use unify::unify_type;

pub(crate) fn infer_expression_type(expr: &IrExpression) -> Option<IrType> {
    match expr {
        IrExpression::Literal(literal) => expression_literal::infer_literal(literal),
        IrExpression::Identifier(name) => expression_identifier::infer_identifier(name),
        IrExpression::Binary { op, left, right } => {
            expression_binary::infer_binary(*op, left, right)
        }
        IrExpression::Conditional {
            consequent,
            alternate,
            ..
        } => expression_conditional::infer_conditional(consequent, alternate),
        IrExpression::Template(parts) => expression_template::infer_template(parts),
        IrExpression::RuntimeCall(call) => expression_runtime::infer_runtime(call),
        _ => expression_trivial::infer_default(expr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{
        ConsoleCall, IrBinOp, IrExpression, IrForInit, IrLiteral, IrStmt, IrTemplatePart, IrType,
        IrVariable, RuntimeNamespace,
    };

    fn number(value: f64) -> IrExpression {
        IrExpression::Literal(IrLiteral::Number(value))
    }

    fn string(value: &str) -> IrExpression {
        IrExpression::Literal(IrLiteral::Str(value.into()))
    }

    fn bool_lit(value: bool) -> IrExpression {
        IrExpression::Literal(IrLiteral::Bool(value))
    }

    fn ident(name: &str) -> IrExpression {
        IrExpression::Identifier(name.into())
    }

    #[test]
    fn infers_expression_types_across_variants() {
        assert_eq!(infer_expression_type(&number(1.0)), Some(IrType::Number));
        assert_eq!(infer_expression_type(&string("hi")), Some(IrType::Str));
        assert_eq!(infer_expression_type(&bool_lit(true)), Some(IrType::Bool));
        assert_eq!(
            infer_expression_type(&ident("undefined")),
            Some(IrType::Unit)
        );
        assert_eq!(infer_expression_type(&ident("value")), None);

        let str_add = IrExpression::Binary {
            op: IrBinOp::Add,
            left: Box::new(string("prefix")),
            right: Box::new(number(1.0)),
        };
        assert_eq!(infer_expression_type(&str_add), Some(IrType::Str));

        let numeric_add = IrExpression::Binary {
            op: IrBinOp::Add,
            left: Box::new(number(1.0)),
            right: Box::new(number(2.0)),
        };
        assert_eq!(infer_expression_type(&numeric_add), Some(IrType::Number));

        let bool_add = IrExpression::Binary {
            op: IrBinOp::Add,
            left: Box::new(bool_lit(true)),
            right: Box::new(number(1.0)),
        };
        assert!(infer_expression_type(&bool_add).is_none());

        let comparison = IrExpression::Binary {
            op: IrBinOp::GreaterThan,
            left: Box::new(number(1.0)),
            right: Box::new(number(0.0)),
        };
        assert_eq!(infer_expression_type(&comparison), Some(IrType::Bool));

        let conditional_same = IrExpression::Conditional {
            test: Box::new(ident("cond")),
            consequent: Box::new(number(1.0)),
            alternate: Box::new(number(2.0)),
        };
        assert_eq!(
            infer_expression_type(&conditional_same),
            Some(IrType::Number)
        );

        let conditional_diff = IrExpression::Conditional {
            test: Box::new(ident("cond")),
            consequent: Box::new(number(1.0)),
            alternate: Box::new(string("fallback")),
        };
        assert!(infer_expression_type(&conditional_diff).is_none());

        let logical = IrExpression::Binary {
            op: IrBinOp::LogicalAnd,
            left: Box::new(ident("left")),
            right: Box::new(ident("right")),
        };
        assert!(infer_expression_type(&logical).is_none());

        let template = IrExpression::Template(vec![
            IrTemplatePart::String("Hello ".into()),
            IrTemplatePart::Expr(Box::new(ident("name"))),
        ]);
        assert_eq!(infer_expression_type(&template), Some(IrType::Str));

        let runtime =
            IrExpression::RuntimeCall(RuntimeNamespace::Console(ConsoleCall::Log(vec![number(
                1.0,
            )])));
        assert_eq!(infer_expression_type(&runtime), Some(IrType::Unit));

        let call = IrExpression::Call {
            callee: Box::new(ident("fn")),
            args: vec![],
        };
        assert!(infer_expression_type(&call).is_none());
    }

    #[test]
    fn unify_type_respects_existing_inference() {
        let mut inferred = None;
        assert!(unify_type(&mut inferred, IrType::Number));
        assert_eq!(inferred, Some(IrType::Number));

        assert!(unify_type(&mut inferred, IrType::Number));
        assert_eq!(inferred, Some(IrType::Number));

        assert!(!unify_type(&mut inferred, IrType::Str));
        assert_eq!(inferred, Some(IrType::Number));
    }

    fn make_var(name: &str) -> IrVariable {
        IrVariable {
            name: name.into(),
            mutable: false,
            ty: IrType::Any,
            value: None,
        }
    }

    #[test]
    fn infers_function_return_type_through_control_flow() {
        let body = vec![
            IrStmt::VarDecl(vec![make_var("value")]),
            IrStmt::Block(vec![IrStmt::Return(Some(number(1.0)))]),
            IrStmt::If {
                condition: ident("flag"),
                then_branch: vec![IrStmt::Return(Some(number(2.0)))],
                else_branch: Some(vec![IrStmt::Return(Some(number(3.0)))]),
            },
            IrStmt::While(ident("cond"), vec![IrStmt::Return(Some(number(4.0)))]),
            IrStmt::DoWhile(vec![IrStmt::Return(Some(number(5.0)))], ident("cond")),
            IrStmt::For {
                init: Some(IrForInit::VarDecl(vec![make_var("i")])),
                condition: None,
                update: None,
                body: vec![IrStmt::Return(Some(number(6.0)))],
            },
        ];

        assert_eq!(
            infer_function_return_type(&body),
            Some(IrType::Number),
            "expected number type inference across control-flow constructs"
        );
    }

    #[test]
    fn infers_unit_for_bodies_without_returns() {
        let body = vec![IrStmt::VarDecl(vec![make_var("value")])];
        assert_eq!(infer_function_return_type(&body), Some(IrType::Unit));
    }

    #[test]
    fn fails_inference_on_conflicting_return_types() {
        let body = vec![
            IrStmt::Return(Some(number(1.0))),
            IrStmt::Return(Some(string("oops"))),
        ];
        assert_eq!(infer_function_return_type(&body), None);
    }

    #[test]
    fn fails_inference_when_return_types_conflict_with_unit() {
        let body = vec![IrStmt::Return(Some(number(1.0))), IrStmt::Return(None)];
        assert_eq!(infer_function_return_type(&body), None);
    }

    #[test]
    fn fails_when_expression_type_cannot_be_determined() {
        let body = vec![IrStmt::Return(Some(IrExpression::Call {
            callee: Box::new(ident("compute")),
            args: vec![],
        }))];
        assert_eq!(infer_function_return_type(&body), None);
    }
}
