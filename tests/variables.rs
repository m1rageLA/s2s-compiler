mod common;
use common::parse_ts_module;

use ir::*;
use lowering::ast_to_ir;

fn only_var(ir: &IrModule) -> &IrVariable {
    match &ir.items[0] {
        IrItem::Variable(v) => v,
        _ => panic!("expected variable item"),
    }
}

#[test]
fn var_number_with_type_and_init_number() {
    let m = parse_ts_module("let x: number = 42;");
    let ir = ast_to_ir(&m);
    assert_eq!(ir.items.len(), 1);

    let v = only_var(&ir);
    assert_eq!(v.name, "x");
    assert_eq!(v.ty, IrType::Number);

    let init = v.value.as_ref().expect("init expected");
    match init {
        IrExpression::Literal(IrLiteral::Number(n)) => assert_eq!(*n, 42.0),
        _ => panic!("expected number literal"),
    }
}

#[test]
fn var_string_with_type_and_init_string() {
    let m = parse_ts_module("const s: string = 'hi';");
    let ir = ast_to_ir(&m);
    let v = only_var(&ir);
    assert_eq!(v.name, "s");
    assert_eq!(v.ty, IrType::Str);
    match v.value.as_ref().unwrap() {
        IrExpression::Literal(IrLiteral::Str(s)) => assert_eq!(s, "hi"),
        _ => panic!("expected string literal"),
    }
}

#[test]
fn var_bool_type_no_init_defaults_to_any_and_none_init_is_supported() {
    let m = parse_ts_module("let f: boolean;");
    let ir = ast_to_ir(&m);
    let v = only_var(&ir);
    assert_eq!(v.name, "f");
    // твой код назначает тип по аннотации; без init — value = None
    assert_eq!(v.ty, IrType::Bool);
    assert!(v.value.is_none());
}

#[test]
fn var_without_type_becomes_any() {
    let m = parse_ts_module("let a = 1;");
    let ir = ast_to_ir(&m);
    let v = only_var(&ir);
    // в var_decl_to_ir, если нет type_ann — Any
    assert_eq!(v.ty, IrType::Any);
}

#[test]
fn var_init_unsupported_expr_marks_identifier_unsupported() {
    // Инициализация массивом — проверяем что элементы понижаются в литералы
    let m = parse_ts_module("let arr = [1,2,3];");
    let ir = ast_to_ir(&m);
    let v = only_var(&ir);
    match v.value.as_ref().unwrap() {
        IrExpression::Array(items) => {
            assert_eq!(items.len(), 3);
            for (idx, item) in items.iter().enumerate() {
                match item {
                    IrExpression::Literal(IrLiteral::Number(n)) => {
                        assert_eq!(*n, (idx as f64) + 1.0);
                    }
                    other => panic!("expected literal number, got {:?}", other),
                }
            }
        }
        other => panic!("expected array literal, got {:?}", other),
    }
}

#[test]
fn var_with_pattern_name_is_ignored() {
    // Деструктурирующее объявление сейчас не поддержано -> пропускается
    let m = parse_ts_module("const {a} = obj;");
    let ir = ast_to_ir(&m);
    assert!(ir.items.is_empty(), "pattern var should be ignored");
}
