use ir::{
    IrDeleteProperty, IrDeleteTarget, IrExpression, IrPostfixOp, IrPrefixOp, IrUnaryOp, IrType,
};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};

use crate::{Codegen, typing};

pub(crate) fn postfixunary_tokens(left: Box<IrExpression>, op: IrPostfixOp) -> TokenStream {
    if let IrExpression::Member { object, property } = left.as_ref() {
        return member_postfix_tokens(object.as_ref(), property, op);
    }

    let inferred = typing::infer_expression_type(left.as_ref());
    let left_tokens = left.codegen();
    let temp = format_ident!("ts_2_rs", span = Span::mixed_site());

    match op {
        IrPostfixOp::Increment => {
            if matches!(inferred, Some(ir::IrType::Number)) {
                quote! ({
                    let ts_2_rs_target = &mut #left_tokens;
                    let #temp = (*ts_2_rs_target);
                    *ts_2_rs_target = #temp + 1.0;
                    #temp
                })
            } else {
                quote! ({
                    let ts_2_rs_target = &mut #left_tokens;
                    let #temp = (*ts_2_rs_target).clone();
                    let ts_2_rs_new = runtime::value::ops::add(
                        #temp.clone(),
                        runtime::value::Value::Number(1.0)
                    );
                    *ts_2_rs_target = ts_2_rs_new;
                    #temp
                })
            }
        }
        IrPostfixOp::Decrement => {
            if matches!(inferred, Some(ir::IrType::Number)) {
                quote! ({
                    let ts_2_rs_target = &mut #left_tokens;
                    let #temp = (*ts_2_rs_target);
                    *ts_2_rs_target = #temp - 1.0;
                    #temp
                })
            } else {
                quote! ({
                    let ts_2_rs_target = &mut #left_tokens;
                    let #temp = (*ts_2_rs_target).clone();
                    let ts_2_rs_new = runtime::value::ops::sub(
                        #temp.clone(),
                        runtime::value::Value::Number(1.0)
                    );
                    *ts_2_rs_target = ts_2_rs_new;
                    #temp
                })
            }
        }
    }
}

fn member_postfix_tokens(object: &IrExpression, property: &str, op: IrPostfixOp) -> TokenStream {
    let object_tokens = object.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    let op_fn = match op {
        IrPostfixOp::Increment => quote!(runtime::value::ops::add),
        IrPostfixOp::Decrement => quote!(runtime::value::ops::sub),
    };

    quote!({
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_current = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal);
        let ts_2_rs_new = #op_fn(ts_2_rs_current.clone(), runtime::value::Value::Number(1.0));
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new);
        ts_2_rs_current
    })
}

pub(crate) fn prefixunary_tokens(arg: &IrExpression, op: IrPrefixOp) -> TokenStream {
    if let IrExpression::Member { object, property } = arg {
        return member_prefix_tokens(object.as_ref(), property, op);
    }

    let inferred = typing::infer_expression_type(arg);
    let target_tokens = arg.codegen();

    match op {
        IrPrefixOp::Increment => {
            if matches!(inferred, Some(IrType::Number)) {
                quote!({
                    let ts_2_rs_target = &mut #target_tokens;
                    *ts_2_rs_target = *ts_2_rs_target + 1.0;
                    *ts_2_rs_target
                })
            } else {
                quote!({
                    let ts_2_rs_target = &mut #target_tokens;
                    let ts_2_rs_new = runtime::value::ops::add(
                        (*ts_2_rs_target).clone(),
                        runtime::value::Value::Number(1.0)
                    );
                    *ts_2_rs_target = ts_2_rs_new.clone();
                    ts_2_rs_new
                })
            }
        }
        IrPrefixOp::Decrement => {
            if matches!(inferred, Some(IrType::Number)) {
                quote!({
                    let ts_2_rs_target = &mut #target_tokens;
                    *ts_2_rs_target = *ts_2_rs_target - 1.0;
                    *ts_2_rs_target
                })
            } else {
                quote!({
                    let ts_2_rs_target = &mut #target_tokens;
                    let ts_2_rs_new = runtime::value::ops::sub(
                        (*ts_2_rs_target).clone(),
                        runtime::value::Value::Number(1.0)
                    );
                    *ts_2_rs_target = ts_2_rs_new.clone();
                    ts_2_rs_new
                })
            }
        }
    }
}

fn member_prefix_tokens(object: &IrExpression, property: &str, op: IrPrefixOp) -> TokenStream {
    let object_tokens = object.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    let op_fn = match op {
        IrPrefixOp::Increment => quote!(runtime::value::ops::add),
        IrPrefixOp::Decrement => quote!(runtime::value::ops::sub),
    };

    quote!({
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_current = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal);
        let ts_2_rs_new = #op_fn(ts_2_rs_current.clone(), runtime::value::Value::Number(1.0));
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new.clone());
        ts_2_rs_new
    })
}

pub(crate) fn unary_tokens(op: &IrUnaryOp, expr: &IrExpression) -> TokenStream {
    match op {
        IrUnaryOp::TypeOf => {
            match typing::infer_expression_type(expr) {
                Some(IrType::Number) => quote! { "number".to_string() },
                Some(IrType::Str) => quote! { "string".to_string() },
                Some(IrType::Bool) => quote! { "boolean".to_string() },
                Some(IrType::Array(_)) => quote! { "object".to_string() },
                _ => {
                    let expr_tokens = expr.codegen();
                    quote! {
                        runtime::value::ops::type_of(runtime::value::into_value((#expr_tokens).clone()))
                    }
                }
            }
        }
        IrUnaryOp::Void => {
            let expr_tokens = expr.codegen();
            quote!({
                #expr_tokens;
                runtime::value::Value::Undefined
            })
        }
        IrUnaryOp::BitwiseNot => {
            let expr_tokens = expr.codegen();
            quote!({
                let ts_2_rs_val = runtime::value::into_value((#expr_tokens).clone()).to_number() as i32;
                (!(ts_2_rs_val)) as f64
            })
        }
    }
}

pub(crate) fn delete_tokens(target: &IrDeleteTarget) -> TokenStream {
    match target {
        IrDeleteTarget::Property { object, property } => {
            let object_tokens = object.codegen();
            match property {
                IrDeleteProperty::Static(name) => {
                    let lit = Literal::string(name);
                    quote!({
                        let ts_2_rs_target = &mut #object_tokens;
                        runtime::value::ops::delete_property_str(ts_2_rs_target, #lit)
                    })
                }
                IrDeleteProperty::Dynamic(expr) => {
                    let prop_tokens = expr.codegen();
                    quote!({
                        let ts_2_rs_target = &mut #object_tokens;
                        let ts_2_rs_prop = runtime::value::into_value((#prop_tokens).clone());
                        runtime::value::ops::delete_property(ts_2_rs_target, ts_2_rs_prop)
                    })
                }
            }
        }
        IrDeleteTarget::Expr(expr) => {
            let expr_tokens = expr.codegen();
            quote!({
                let _ = (#expr_tokens);
                true
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
                let ts_2_rs_target = &mut counter;
                let ts_2_rs = (*ts_2_rs_target).clone();
                let ts_2_rs_new =
                    runtime::value::ops::add(ts_2_rs.clone(), runtime::value::Value::Number(1.0));
                *ts_2_rs_target = ts_2_rs_new;
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
                let ts_2_rs_target = &mut index;
                let ts_2_rs = (*ts_2_rs_target).clone();
                let ts_2_rs_new =
                    runtime::value::ops::sub(ts_2_rs.clone(), runtime::value::Value::Number(1.0));
                *ts_2_rs_target = ts_2_rs_new;
                ts_2_rs
            })
            .to_string()
        );
    }
}
