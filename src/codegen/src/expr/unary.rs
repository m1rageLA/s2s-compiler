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
                *#left -= 1.0;  
                #temp
            })
        }
    }
}
