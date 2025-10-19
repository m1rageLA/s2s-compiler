use ir::{IrExpression, IrPostfixOp};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use crate::Codegen;

pub(crate) fn postfixunary_tokens(left: Box<IrExpression>, op: IrPostfixOp) -> TokenStream {
    let left = left.codegen();
    let temp = format_ident!("ts_2_rs", span = Span::mixed_site());

    match op {
        IrPostfixOp::Increment => {
            // { let _ts2r_tmp = #left; #left += 1; _ts2r_tmp }
            quote! ({
                let #temp = #left;
                #left += 1.0;
                #temp
            })
        }
        IrPostfixOp::Decrement => {
            // { let _ts2r_tmp = #left; #left -= 1; _ts2r_tmp }
            quote! ({
                let #temp = #left;
                #left -= 1.0;
                #temp
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn postfix_increment_preserves_original_value() {
        let tokens = postfixunary_tokens(
            Box::new(IrExpression::Identifier("counter".into())),
            IrPostfixOp::Increment,
        );

        assert_eq!(
            tokens.to_string(),
            quote!({
                let ts_2_rs = counter;
                counter += 1.0;
                ts_2_rs
            })
            .to_string()
        );
    }

    #[test]
    fn postfix_decrement_updates_value_after_snapshot() {
        let tokens = postfixunary_tokens(
            Box::new(IrExpression::Identifier("index".into())),
            IrPostfixOp::Decrement,
        );

        assert_eq!(
            tokens.to_string(),
            quote!({
                let ts_2_rs = index;
                index -= 1.0;
                ts_2_rs
            })
            .to_string()
        );
    }
}
