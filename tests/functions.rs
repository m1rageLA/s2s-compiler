mod common;
use common::parse_ts_module;

use ir::*;
use lowering::ast_to_ir;

fn only_fn(ir: &IrModule) -> &IrFunction {
    match &ir.items[0] {
        IrItem::Function(f) => f,
        other => panic!("expected function, got {:?}", other),
    }
}

#[test]
fn fn_with_typed_params_and_return() {
    let m = parse_ts_module(
        r#"
        function add(x: number, y: number): number { return x + y; }
    "#,
    );
    let ir = ast_to_ir(&m);
    assert_eq!(ir.items.len(), 1);
    let f = only_fn(&ir);

    assert_eq!(f.name, "add");
    assert_eq!(f.params.len(), 2);
    assert_eq!(f.params[0].name, "x");
    assert_eq!(f.params[0].ty, IrType::Int);
    assert_eq!(f.params[1].name, "y");
    assert_eq!(f.params[1].ty, IrType::Int);
    assert_eq!(f.ret, IrType::Int);

    // body пока пустой в твоей реализации
    assert!(f.body.is_empty());
}

#[test]
fn fn_without_return_type_becomes_any() {
    let m = parse_ts_module("function f(x: string) { }");
    let ir = ast_to_ir(&m);
    let f = only_fn(&ir);
    assert_eq!(f.ret, IrType::Any);
}

#[test]
fn fn_with_unsupported_param_pattern_is_ignored() {
    // Такой fn должен быть пропущен (возвращается None в fn_decl_to_ir)
    let m = parse_ts_module("function g({a}: {a: number}) { return a; }");
    let ir = ast_to_ir(&m);
    assert!(
        ir.items.is_empty(),
        "function with pattern param should be ignored"
    );
}

#[test]
fn module_mixed_items_keeps_only_supported() {
    let m = parse_ts_module(
        r#"
        function ok(x: number): number { return x; }
        function bad({a}: any) { return a; } // игнор
        let n: number = 1;
        const {p} = obj; // игнор
    "#,
    );
    let ir = ast_to_ir(&m);
    assert_eq!(ir.items.len(), 2);

    match (&ir.items[0], &ir.items[1]) {
        (IrItem::Function(f), IrItem::Variable(v)) => {
            assert_eq!(f.name, "ok");
            assert_eq!(v.name, "n");
        }
        _ => panic!("unexpected order or items"),
    }
}
