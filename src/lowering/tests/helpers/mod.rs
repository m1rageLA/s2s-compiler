#![allow(dead_code)]

use ir::{IrExpression, IrItem, IrLiteral, IrModule, IrVariable};
use lowering::ast_to_ir;
use parser::ast as parse_ts_module;

pub fn lower(source: &str) -> IrModule {
    let module = parse_ts_module(source);
    ast_to_ir(&module)
}

pub fn expect_variable<'a>(item: &'a IrItem, name: &str) -> &'a IrVariable {
    match item {
        IrItem::Variable(var) if var.name == name => var,
        other => panic!("expected variable {name}, got {other:?}"),
    }
}

pub fn assert_identifier(expr: &IrExpression, expected: &str) {
    match expr {
        IrExpression::Identifier(name) => assert_eq!(name, expected),
        other => panic!("expected identifier {expected}, got {other:?}"),
    }
}

pub fn assert_number_literal(expr: Option<&IrExpression>, expected: f64) {
    let expr = expr.expect("expected number literal expression");
    match expr {
        IrExpression::Literal(IrLiteral::Number(value)) => {
            assert!(
                (value - expected).abs() < f64::EPSILON,
                "expected {expected}, got {value}"
            );
        }
        other => panic!("expected numeric literal {expected}, got {other:?}"),
    }
}

pub fn assert_string_literal(expr: Option<&IrExpression>, expected: &str) {
    let expr = expr.expect("expected string literal expression");
    match expr {
        IrExpression::Literal(IrLiteral::Str(value)) => assert_eq!(value, expected),
        other => panic!("expected string literal {expected}, got {other:?}"),
    }
}

pub fn assert_bool_literal(expr: Option<&IrExpression>, expected: bool) {
    let expr = expr.expect("expected bool literal expression");
    match expr {
        IrExpression::Literal(IrLiteral::Bool(value)) => assert_eq!(*value, expected),
        other => panic!("expected bool literal {expected}, got {other:?}"),
    }
}
