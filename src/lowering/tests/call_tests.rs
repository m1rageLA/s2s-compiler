mod helpers;
use helpers::assert_number_literal;
use ir::IrExpression;
use lowering::expressions::call_to_ir;
use swc_common::DUMMY_SP;
use swc_ecma_ast as swc_ast;

#[test]
fn handles_super_calls() {
    let call_expr = swc_ast::CallExpr {
        span: DUMMY_SP,
        ctxt: Default::default(),
        type_args: None,
        callee: swc_ast::Callee::Super(swc_ast::Super { span: DUMMY_SP }),
        args: vec![swc_ast::ExprOrSpread {
            spread: None,
            expr: Box::new(swc_ast::Expr::Lit(swc_ast::Lit::Num(swc_ast::Number {
                span: DUMMY_SP,
                value: 1.0,
                raw: None,
            }))),
        }],
    };

    match call_to_ir(&call_expr) {
        IrExpression::SuperCall { args } => {
            assert_eq!(args.len(), 1);
            assert_number_literal(Some(&args[0]), 1.0);
        }
        other => panic!("expected super call expression, got {other:?}"),
    }
}
