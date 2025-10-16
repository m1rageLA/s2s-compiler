use crate::ast_to_ir;
use ir::{IrExpression, IrItem, IrLiteral, IrType};
use parser::ast;

#[test]
fn lowers_typed_variable_declaration_into_ir() {
    let module = ast("let answer: number = 42;");

    let ir_module = ast_to_ir(&module);

    assert_eq!(ir_module.items.len(), 1, "expected a single IR item");

    match &ir_module.items[0] {
        IrItem::Variable(var) => {
            assert_eq!(var.name, "answer");
            assert_eq!(var.ty, IrType::Number);

            let value = var.value.as_ref().expect("variable should have an initializer");
            match value {
                IrExpression::Literal(IrLiteral::Number(n)) => assert!((*n - 42.0).abs() < f64::EPSILON),
                other => panic!("expected numeric literal, got {other:?}"),
            }
        }
        other => panic!("expected IR variable, got {other:?}"),
    }
}

