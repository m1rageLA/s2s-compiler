use crate::ast_to_ir;
use ir::{IrExpression, IrFunction, IrItem, IrLiteral, IrParam, IrType};
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

#[test]
fn lowers_typed_function_declaration_into_ir() {
    let module = ast("function add(a: number, b: number): number {}");
    let ir_module = ast_to_ir(&module);

    assert_eq!(ir_module.items.len(), 1, "expected a single IR item");
    assert_eq!(ir_module.items[0], IrItem::Function(IrFunction {
        name: "add".into(),
        params: vec![IrParam { name: "a".into(), ty: IrType::Number }, IrParam { name: "b".into(), ty: IrType::Number }],
        ret: IrType::Number,
        body: vec![],
    }), "expected IR function");

    match &ir_module.items[0] {
        IrItem::Function(funct) => {
            assert_eq!(funct.name, "add");
            assert_eq!(funct.params.len(), 2);
            assert_eq!(funct.params[0].ty, IrType::Number);
            assert_eq!(funct.params[1].ty, IrType::Number);
            assert_eq!(funct.ret, IrType::Number);
        }
        other => panic!("expected IR function, got {other:?}"),
    } 
    
}