mod common;
use common::parse_ts_module;

use ir::*;
use lowering::ast_to_ir;

fn op_of_var_init(ir: &IrModule) -> &IrBinOp {
    let v = match &ir.items[0] {
        IrItem::Variable(v) => v,
        _ => panic!("expected variable"),
    };
    let IrExpression::Binary { op, .. } = v.value.as_ref().expect("init").clone() else {
        panic!("expected binary expression");
    };
    // возвращаем ссылку из временной нельзя, так что сравним в месте вызова
    Box::leak(Box::new(op))
}

#[test]
fn all_binary_ops_mapped() {
    let cases = vec![
        ("const r = 1 + 2;", IrBinOp::Add),
        ("const r = 1 - 2;", IrBinOp::Sub),
        ("const r = 2 * 3;", IrBinOp::Mul),
        ("const r = 6 / 2;", IrBinOp::Div),
        ("const r = 5 % 2;", IrBinOp::Mod),
        ("const r = 2 ** 3;", IrBinOp::Exp),
        ("const r = 1 == 2;", IrBinOp::Equal),
        ("const r = 1 === 2;", IrBinOp::StrictEqual),
        ("const r = 1 != 2;", IrBinOp::NotEqual),
        ("const r = 1 !== 2;", IrBinOp::StrictNotEqual),
        ("const r = 1 < 2;", IrBinOp::LessThan),
        ("const r = 1 <= 2;", IrBinOp::LessThanOrEqual),
        ("const r = 2 > 1;", IrBinOp::GreaterThan),
        ("const r = 2 >= 1;", IrBinOp::GreaterThanOrEqual),
        ("const r = 1 << 2;", IrBinOp::LeftShift),
        ("const r = 4 >> 1;", IrBinOp::RightShift),
        ("const r = 4 >>> 1;", IrBinOp::UnsignedRightShift),
        ("const r = 1 | 2;", IrBinOp::BitwiseOr),
        ("const r = 1 ^ 2;", IrBinOp::BitwiseXor),
        ("const r = 1 & 2;", IrBinOp::BitwiseAnd),
        ("const r = true || false;", IrBinOp::LogicalOr),
        ("const r = true && false;", IrBinOp::LogicalAnd),
        ("const r = 'x' in obj;", IrBinOp::In),
        ("const r = a instanceof B;", IrBinOp::InstanceOf),
    ];

    for (src, expected) in cases {
        let m = parse_ts_module(src);
        let ir = ast_to_ir(&m);
        let got = op_of_var_init(&ir);
        assert_eq!(*got, expected, "failed on: {}", src);
    }
}

#[test]
fn binary_subtrees_are_recursively_converted() {
    // Проверим, что left/right идут через expr_to_ir()
    let m = parse_ts_module("const z = (1 + 2) * (3 - 4);");
    let ir = ast_to_ir(&m);

    let v = match &ir.items[0] {
        IrItem::Variable(v) => v,
        _ => unreachable!(),
    };
    let IrExpression::Binary { op, left, right } = v.value.as_ref().unwrap() else {
        unreachable!()
    };

    assert_eq!(*op, IrBinOp::Mul);

    // левый: (1 + 2)
    match &**left {
        IrExpression::Binary { op, .. } => assert_eq!(*op, IrBinOp::Add),
        _ => panic!("left should be binary +"),
    }
    // правый: (3 - 4)
    match &**right {
        IrExpression::Binary { op, .. } => assert_eq!(*op, IrBinOp::Sub),
        _ => panic!("right should be binary -"),
    }
}
