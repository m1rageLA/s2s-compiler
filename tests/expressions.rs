mod common;
use common::parse_ts_module;

use ir::*;
use lowering::ast_to_ir;

#[test]
fn identifier_expr_in_var_init() {
    let m = parse_ts_module("let a = b;");
    let ir = ast_to_ir(&m);
    let v = match &ir.items[0] {
        IrItem::Variable(v) => v,
        _ => unreachable!(),
    };
    match v.value.as_ref().unwrap() {
        IrExpression::Identifier(s) => assert_eq!(s, "b"),
        _ => panic!("expected identifier"),
    }
}

#[test]
fn string_literal_expr_in_var_init() {
    let m = parse_ts_module("let s = \"ok\";");
    let ir = ast_to_ir(&m);
    let v = match &ir.items[0] {
        IrItem::Variable(v) => v,
        _ => unreachable!(),
    };
    match v.value.as_ref().unwrap() {
        IrExpression::Literal(IrLiteral::Str(s)) => assert_eq!(s, "ok"),
        _ => panic!("expected string literal"),
    }
}

#[test]
fn number_literal_expr_in_var_init() {
    let m = parse_ts_module("let n = 123;");
    let ir = ast_to_ir(&m);
    let v = match &ir.items[0] {
        IrItem::Variable(v) => v,
        _ => unreachable!(),
    };
    match v.value.as_ref().unwrap() {
        IrExpression::Literal(IrLiteral::Number(n)) => assert_eq!(*n, 123.0),
        _ => panic!("expected number literal"),
    }
}

#[test]
fn float_literal_expr_in_var_init() {
    let m = parse_ts_module("let n = 3.14;");
    let ir = ast_to_ir(&m);
    let v = match &ir.items[0] {
        IrItem::Variable(v) => v,
        _ => unreachable!(),
    };
    match v.value.as_ref().unwrap() {
        IrExpression::Literal(IrLiteral::Number(n)) => assert!((*n - 3.14).abs() < f64::EPSILON),
        other => panic!("expected number literal, got {:?}", other),
    }
}

#[test]
fn negative_number_literal_expr_in_var_init() {
    let m = parse_ts_module("let n = -5;");
    let ir = ast_to_ir(&m);
    let v = match &ir.items[0] {
        IrItem::Variable(v) => v,
        _ => unreachable!(),
    };
    match v.value.as_ref().unwrap() {
        IrExpression::Literal(IrLiteral::Number(n)) => assert_eq!(*n, -5.0),
        other => panic!("expected negative number literal, got {:?}", other),
    }
}
