use ir::IrBinOp;
use proc_macro2::TokenStream;
use quote::quote;

use super::unsupported::unsupported_bin_op;

pub(crate) fn binary_op_tokens(op: IrBinOp, left: TokenStream, right: TokenStream) -> TokenStream {
    match op {
        // Arithmetic operators are emitted directly here for numeric cases.
        // Dynamic/Any/Value operands are lowered into `IrExpression::RuntimeCall`
        // by the lowering step, so codegen only needs to handle plain
        // arithmetic tokens for `IrExpression::Binary`.
        IrBinOp::Add => quote! { (#left) + (#right) },
        IrBinOp::Sub => quote! { (#left) - (#right) },
        IrBinOp::Mul => quote! { (#left) * (#right) },
        IrBinOp::Div => quote! { (#left) / (#right) },
        IrBinOp::Mod => quote! { (#left) % (#right) },

        //not supported runtime
        IrBinOp::Equal | IrBinOp::StrictEqual => quote! { (#left) == (#right) },
        IrBinOp::NotEqual | IrBinOp::StrictNotEqual => quote! { (#left) != (#right) },
        IrBinOp::LessThan => quote! { (#left) < (#right) },
        IrBinOp::LessThanOrEqual => quote! { (#left) <= (#right) },
        IrBinOp::GreaterThan => quote! { (#left) > (#right) },
        IrBinOp::GreaterThanOrEqual => quote! { (#left) >= (#right) },
        IrBinOp::LeftShift => quote! { (#left) << (#right) },
        IrBinOp::RightShift => quote! { (#left) >> (#right) },
        IrBinOp::BitwiseOr => quote! { (#left) | (#right) },
        IrBinOp::BitwiseXor => quote! { (#left) ^ (#right) },
        IrBinOp::BitwiseAnd => quote! { (#left) & (#right) },
        IrBinOp::LogicalOr => quote! { (#left) || (#right) },
        IrBinOp::LogicalAnd => quote! { (#left) && (#right) },
        IrBinOp::UnsignedRightShift => unsupported_bin_op("unsigned right shift"),
        IrBinOp::In => unsupported_bin_op("in"),
        IrBinOp::InstanceOf => unsupported_bin_op("instanceof"),
        IrBinOp::Exp => unsupported_bin_op("exponentiation"),
        IrBinOp::Unsupported => unsupported_bin_op("unsupported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream as Ts;
    use quote::quote;
    use syn::{parse2, Expr};

    /// Удобняшка: нормализуем строковое представление токенов.
    /// to_string у TokenStream детерминирован, но может иметь разные пробелы в зависимости от версии.
    fn norm(ts: &Ts) -> String {
        ts.to_string().replace([' ', '\n', '\t'], "")
    }

    /// Проверяет, что токены парсятся как корректное выражение Rust
    fn assert_parses(expr: &Ts) {
        parse2::<Expr>(expr.clone()).expect("generated tokens must be a valid Rust expression");
    }

    #[test]
    fn supported_binary_ops_emit_expected_tokens_on_simple_idents() {
        let left = quote!(lhs);
        let right = quote!(rhs);

        let cases: Vec<(IrBinOp, Ts)> = vec![
            (IrBinOp::Add, quote! { (lhs) + (rhs) }),
            (IrBinOp::Sub, quote! { (lhs) - (rhs) }),
            (IrBinOp::Mul, quote! { (lhs) * (rhs) }),
            (IrBinOp::Div, quote! { (lhs) / (rhs) }),
            (IrBinOp::Mod, quote! { (lhs) % (rhs) }),
            (IrBinOp::Equal, quote! { (lhs) == (rhs) }),
            (IrBinOp::StrictEqual, quote! { (lhs) == (rhs) }),
            (IrBinOp::NotEqual, quote! { (lhs) != (rhs) }),
            (IrBinOp::StrictNotEqual, quote! { (lhs) != (rhs) }),
            (IrBinOp::LessThan, quote! { (lhs) < (rhs) }),
            (IrBinOp::LessThanOrEqual, quote! { (lhs) <= (rhs) }),
            (IrBinOp::GreaterThan, quote! { (lhs) > (rhs) }),
            (IrBinOp::GreaterThanOrEqual, quote! { (lhs) >= (rhs) }),
            (IrBinOp::LeftShift, quote! { (lhs) << (rhs) }),
            (IrBinOp::RightShift, quote! { (lhs) >> (rhs) }),
            (IrBinOp::BitwiseOr, quote! { (lhs) | (rhs) }),
            (IrBinOp::BitwiseXor, quote! { (lhs) ^ (rhs) }),
            (IrBinOp::BitwiseAnd, quote! { (lhs) & (rhs) }),
            (IrBinOp::LogicalOr, quote! { (lhs) || (rhs) }),
            (IrBinOp::LogicalAnd, quote! { (lhs) && (rhs) }),
        ];

        for (op, expected) in cases {
            let got = binary_op_tokens(op, left.clone(), right.clone());
            assert_eq!(
                norm(&got),
                norm(&expected),
                "mismatch for {op:?}: got `{}` expected `{}`",
                got,
                expected
            );
            assert_parses(&got);
        }
    }

    #[test]
    fn parentheses_preserve_precedence_for_complex_operands() {
        // Важно: левый/правый могут быть уже составными выражениями — мы обязаны поставить скобки.
        let complex_left = quote! { a + b * c };
        let complex_right = quote! { d || e && f };

        // Проверим на нескольких бинарных операторах разных приоритетов.
        let cases = [
            (IrBinOp::Mul, quote! { (a + b * c) * (d || e && f) }),
            (IrBinOp::Add, quote! { (a + b * c) + (d || e && f) }),
            (IrBinOp::LogicalAnd, quote! { (a + b * c) && (d || e && f) }),
            (IrBinOp::BitwiseOr, quote! { (a + b * c) | (d || e && f) }),
            (IrBinOp::RightShift, quote! { (a + b * c) >> (d || e && f) }),
            (IrBinOp::LessThanOrEqual, quote! { (a + b * c) <= (d || e && f) }),
        ];

        for (op, expected) in cases {
            let got = binary_op_tokens(op, complex_left.clone(), complex_right.clone());
            assert_eq!(norm(&got), norm(&expected), "parentheses lost for {op:?}");
            assert_parses(&got);
        }
    }

    #[test]
    fn works_with_weird_idents_and_paths() {
        // Проверяем, что не ломаемся на путях, generic'ах и raw идентификаторах
        let left = quote! { ::core::mem::size_of::<r#match>() };
        let right = quote! { some::module::r#type::<Vec<u8>>() };
        let cases = [
            (IrBinOp::Sub, quote! { (::core::mem::size_of::<r#match>()) - (some::module::r#type::<Vec<u8>>()) }),
            (IrBinOp::Equal, quote! { (::core::mem::size_of::<r#match>()) == (some::module::r#type::<Vec<u8>>()) }),
        ];
        for (op, expected) in cases {
            let got = binary_op_tokens(op, left.clone(), right.clone());
            assert_eq!(norm(&got), norm(&expected), "mismatch with raw idents/paths for {op:?}");
            assert_parses(&got);
        }
    }

    #[test]
    fn generation_is_deterministic_for_same_inputs() {
        let left = quote!(foo.bar().baz(1, 2 + 3));
        let right = quote!((x << 2) | 7);
        let op = IrBinOp::BitwiseAnd;

        let a = binary_op_tokens(op, left.clone(), right.clone());
        let b = binary_op_tokens(op, left.clone(), right.clone());
        assert_eq!(norm(&a), norm(&b), "TokenStream must be deterministic");
    }

    #[test]
    fn unsupported_binary_ops_emit_clean_panic_without_operands() {
        let left = quote!(lhs + 42);     // намеренно не тривиальные
        let right = quote!(rhs >> 1);

        let cases = vec![
            (IrBinOp::UnsignedRightShift, "codegen for binary op `unsigned right shift` not implemented"),
            (IrBinOp::In, "codegen for binary op `in` not implemented"),
            (IrBinOp::InstanceOf, "codegen for binary op `instanceof` not implemented"),
            (IrBinOp::Exp, "codegen for binary op `exponentiation` not implemented"),
            (IrBinOp::Unsupported, "codegen for binary op `unsupported` not implemented"),
        ];

        for (op, message) in cases {
            let got = binary_op_tokens(op, left.clone(), right.clone());
            let expected = quote! { panic!(#message) };

            // 1) точное «золотое» сравнение
            assert_eq!(norm(&got), norm(&expected), "unexpected tokens for {op:?}");

            // 2) синтаксис валиден
            assert_parses(&got);

            // 3) в сгенерированном коде не должно быть следов lhs/rhs
            let got_str = got.to_string();
            assert!(
                !got_str.contains("lhs") && !got_str.contains("rhs"),
                "unsupported op must not embed operands; got: {got_str}"
            );

            // 4) сообщение должно быть именно строковым литералом (а не форматированием)
            // На уровне токенов это просто проверка наличия кавычек вокруг текста.
            assert!(
                got_str.contains("codegen for binary op")
                    && got_str.contains("not implemented"),
                "panic message content changed; got: {got_str}"
            );
        }
    }

    #[test]
    fn boolean_vs_bitwise_are_not_confused() {
        let left = quote!(p());
        let right = quote!(q());

        let logical_or = binary_op_tokens(IrBinOp::LogicalOr, left.clone(), right.clone());
        let bit_or = binary_op_tokens(IrBinOp::BitwiseOr, left.clone(), right.clone());
        assert_ne!(norm(&logical_or), norm(&bit_or), "|| must not degrade to |");
        assert_eq!(norm(&logical_or), norm(&quote! { (p()) || (q()) }));
        assert_eq!(norm(&bit_or), norm(&quote! { (p()) | (q()) }));
    }
}
