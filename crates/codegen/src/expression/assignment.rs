use ir::{ArrayCall, IrArrayKind, IrAssignOp, IrExpression, IrType, IrTypeAliasDef, RuntimeNamespace};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};

use crate::{expression::object_struct_literal_tokens, Codegen, typing};

use super::unsupported::unsupported_assign_op;

pub(crate) fn assignment_tokens(
    op: IrAssignOp,
    left: &IrExpression,
    right: &IrExpression,
) -> TokenStream {
    if let Some((target, index, element)) = array_index_target(left) {
        return array_index_assignment_tokens(op, target, index, element, right);
    }
    if let IrExpression::Member { object, property } = left {
        return member_assignment_tokens(op, object.as_ref(), property, right);
    }

    let left_tokens = left.codegen();
    let left_ty = typing::infer_expression_type(left);
    let right_tokens = match (left_ty, right) {
        (Some(ir::IrType::Object(id)), IrExpression::Object(props)) => {
            object_struct_literal_tokens(id, props)
        }
        _ => right.codegen(),
    };

    let target_ident = format_ident!("ts_2_rs_target", span = Span::mixed_site());
    let value_ident = format_ident!("ts_2_rs_value", span = Span::mixed_site());
    let rhs_ident = format_ident!("ts_2_rs_rhs", span = Span::mixed_site());

    let right_ty = typing::infer_expression_type(right);
    let dynamic = matches!(left_ty, Some(ir::IrType::Any | ir::IrType::Value)) || left_ty.is_none();
    let coerce_rhs = |tokens: TokenStream| {
        typing::coerce_to_type(tokens, &left_ty.unwrap_or(ir::IrType::Value), right_ty)
    };

    let assign_value = |value_ident: &proc_macro2::Ident| {
        if left_ty.as_ref().map(typing::is_copy_type).unwrap_or(false) {
            quote! {
                *#target_ident = #value_ident;
                #value_ident
            }
        } else {
            quote! {
                *#target_ident = (#value_ident).clone();
                #value_ident
            }
        }
    };

    match op {
        IrAssignOp::Assign => {
            if dynamic {
                let assign_tokens = assign_value(&value_ident);
                let rhs_value = if typing::expr_is_copy_type(right) {
                    quote! { #right_tokens }
                } else {
                    quote! { (#right_tokens).clone() }
                };
                quote!({
                    let #value_ident = #rhs_value;
                    let #target_ident = &mut #left_tokens;
                    #assign_tokens
                })
            } else {
                let coerced = coerce_rhs(quote! { (#right_tokens) });
                let assign_tokens = assign_value(&value_ident);
                quote!({
                    let #value_ident = #coerced;
                    let #target_ident = &mut #left_tokens;
                    #assign_tokens
                })
            }
        }
        IrAssignOp::AddAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::add),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                    typing::expr_is_copy_type(right),
                )
            } else if matches!(left_ty, Some(ir::IrType::Str)) {
                quote!({
                    let #rhs_ident = (#right_tokens).to_string();
                    let #target_ident = &mut #left_tokens;
                    #target_ident.push_str(&#rhs_ident);
                    (#target_ident).clone()
                })
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) + (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::SubAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::sub),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                    typing::expr_is_copy_type(right),
                )
            } else if matches!(left_ty, Some(ir::IrType::UInt)) {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident).saturating_sub(#rhs_ident);
                    (*#target_ident)
                })
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) - (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::MulAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::mul),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                    typing::expr_is_copy_type(right),
                )
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) * (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::DivAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::div),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                    typing::expr_is_copy_type(right),
                )
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) / (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::ModAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::modulo),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                    typing::expr_is_copy_type(right),
                )
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) % (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::LeftShiftAssign => bitwise_assignment(
            quote!(<<),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
            typing::expr_is_copy_type(right),
        ),
        IrAssignOp::RightShiftAssign => bitwise_assignment(
            quote!(>>),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
            typing::expr_is_copy_type(right),
        ),
        IrAssignOp::BitwiseOrAssign => bitwise_assignment(
            quote!(|),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
            typing::expr_is_copy_type(right),
        ),
        IrAssignOp::BitwiseXorAssign => bitwise_assignment(
            quote!(^),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
            typing::expr_is_copy_type(right),
        ),
        IrAssignOp::BitwiseAndAssign => bitwise_assignment(
            quote!(&),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
            typing::expr_is_copy_type(right),
        ),
        IrAssignOp::ExpAssign => {
            exponent_assign_tokens(
                &target_ident,
                &rhs_ident,
                &left_tokens,
                &right_tokens,
                typing::expr_is_copy_type(right),
            )
        }
        IrAssignOp::UnsignedRightShiftAssign => unsupported_assign_op("unsigned right shift"),
        IrAssignOp::LogicalOrAssign => unsupported_assign_op("logical or assignment"),
        IrAssignOp::LogicalAndAssign => unsupported_assign_op("logical and assignment"),
        IrAssignOp::NullishCoalesceAssign => unsupported_assign_op("nullish coalesce assignment"),
    }
}

fn array_index_target(left: &IrExpression) -> Option<(&IrExpression, &IrExpression, Option<IrArrayKind>)> {
    match left {
        IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index {
            target,
            index,
            element,
        })) => Some((target.as_ref(), index.as_ref(), *element)),
        IrExpression::Paren(inner) => array_index_target(inner.as_ref()),
        _ => None,
    }
}

fn array_index_assignment_tokens(
    op: IrAssignOp,
    target: &IrExpression,
    index: &IrExpression,
    element: Option<IrArrayKind>,
    right: &IrExpression,
) -> TokenStream {
    let target_tokens = target.codegen();
    let index_tokens = index.codegen();
    let right_tokens = right.codegen();

    let target_ty = typing::infer_expression_type(target).or_else(|| element.map(IrType::Array));
    let element_ty = match target_ty {
        Some(IrType::Array(kind)) => array_element_type(kind),
        _ => IrType::Value,
    };
    let right_ty = typing::infer_expression_type(right);

    let index_ty = typing::infer_expression_type(index);
    let idx_tokens = if matches!(index_ty, Some(IrType::UInt)) {
        quote! { #index_tokens }
    } else if matches!(index_ty, Some(IrType::Number)) {
        quote! { (#index_tokens) as usize }
    } else {
        quote! { runtime::value::into_value(#index_tokens).to_number() as usize }
    };

    let idx_ident = format_ident!("ts_2_rs_idx", span = Span::mixed_site());
    let vec_ident = format_ident!("ts_2_rs_vec", span = Span::mixed_site());
    let target_ident = format_ident!("ts_2_rs_target", span = Span::mixed_site());
    let rhs_ident = format_ident!("ts_2_rs_rhs", span = Span::mixed_site());
    let left_ident = format_ident!("ts_2_rs_left", span = Span::mixed_site());
    let value_ident = format_ident!("ts_2_rs_value", span = Span::mixed_site());
    let default_value = array_element_default(&element_ty);
    let vec_tokens = match target {
        IrExpression::Identifier(name)
            if matches!(typing::lookup_binding_pass(name), Some(typing::ParamPass::MutRef)) =>
        {
            quote! { &mut *#target_tokens }
        }
        _ => quote! { &mut #target_tokens },
    };

    let number_op = |op_tokens: TokenStream| {
        let coerced_rhs = typing::coerce_to_type(quote! { (#right_tokens) }, &IrType::Number, right_ty);
        quote!({
            let #idx_ident = #idx_tokens;
            let #rhs_ident = #coerced_rhs;
            let #vec_ident = #vec_tokens;
            if #idx_ident >= #vec_ident.len() {
                #vec_ident.resize(#idx_ident + 1, #default_value);
            }
            let #target_ident = &mut #vec_ident[#idx_ident];
            *#target_ident = (*#target_ident) #op_tokens (#rhs_ident);
            (*#target_ident)
        })
    };

    let value_op = |op_fn: TokenStream| {
        let new_value = quote! { #op_fn(#left_ident, (#rhs_ident).clone()) };
        let coerced = if matches!(element_ty, IrType::Value | IrType::Any) {
            new_value
        } else {
            typing::coerce_to_type(new_value, &element_ty, Some(IrType::Value))
        };
        let assign_value = if typing::is_copy_type(&element_ty) {
            quote! { *#target_ident = #value_ident; }
        } else {
            quote! { *#target_ident = (#value_ident).clone(); }
        };
        quote!({
            let #idx_ident = #idx_tokens;
            let #rhs_ident = runtime::value::into_value(#right_tokens);
            let #vec_ident = #vec_tokens;
            if #idx_ident >= #vec_ident.len() {
                #vec_ident.resize(#idx_ident + 1, #default_value);
            }
            let #target_ident = &mut #vec_ident[#idx_ident];
            let #left_ident = runtime::value::into_value((*#target_ident).clone());
            let #value_ident = #coerced;
            #assign_value
            #value_ident
        })
    };

    match op {
        IrAssignOp::Assign => {
            let coerced_rhs = typing::coerce_to_type(quote! { (#right_tokens) }, &element_ty, right_ty);
            let assign_value = if typing::is_copy_type(&element_ty) {
                quote! { *#target_ident = #value_ident; }
            } else {
                quote! { *#target_ident = (#value_ident).clone(); }
            };
            quote!({
                let #idx_ident = #idx_tokens;
                let #value_ident = #coerced_rhs;
                let #vec_ident = #vec_tokens;
                if #idx_ident >= #vec_ident.len() {
                    #vec_ident.resize(#idx_ident + 1, #default_value);
                }
                let #target_ident = &mut #vec_ident[#idx_ident];
                #assign_value
                #value_ident
            })
        }
        IrAssignOp::AddAssign => {
            if matches!(element_ty, IrType::Number) {
                number_op(quote!(+))
            } else if matches!(element_ty, IrType::Str) {
                let rhs = typing::coerce_to_type(quote! { (#right_tokens) }, &IrType::Str, right_ty);
                quote!({
                    let #idx_ident = #idx_tokens;
                    let #rhs_ident = #rhs;
                    let #vec_ident = #vec_tokens;
                    if #idx_ident >= #vec_ident.len() {
                        #vec_ident.resize(#idx_ident + 1, #default_value);
                    }
                    let #target_ident = &mut #vec_ident[#idx_ident];
                    #target_ident.push_str(&#rhs_ident);
                    (#target_ident).clone()
                })
            } else {
                value_op(quote!(runtime::value::ops::add))
            }
        }
        IrAssignOp::SubAssign => {
            if matches!(element_ty, IrType::Number) {
                number_op(quote!(-))
            } else {
                value_op(quote!(runtime::value::ops::sub))
            }
        }
        IrAssignOp::MulAssign => {
            if matches!(element_ty, IrType::Number) {
                number_op(quote!(*))
            } else {
                value_op(quote!(runtime::value::ops::mul))
            }
        }
        IrAssignOp::DivAssign => {
            if matches!(element_ty, IrType::Number) {
                number_op(quote!(/))
            } else {
                value_op(quote!(runtime::value::ops::div))
            }
        }
        IrAssignOp::ModAssign => {
            if matches!(element_ty, IrType::Number) {
                number_op(quote!(%))
            } else {
                value_op(quote!(runtime::value::ops::modulo))
            }
        }
        IrAssignOp::ExpAssign => {
            if matches!(element_ty, IrType::Number) {
                let coerced_rhs = typing::coerce_to_type(quote! { (#right_tokens) }, &IrType::Number, right_ty);
                quote!({
                    let #idx_ident = #idx_tokens;
                    let #rhs_ident = #coerced_rhs;
                    let #vec_ident = #vec_tokens;
                    if #idx_ident >= #vec_ident.len() {
                        #vec_ident.resize(#idx_ident + 1, #default_value);
                    }
                    let #target_ident = &mut #vec_ident[#idx_ident];
                    *#target_ident = (*#target_ident).powf(#rhs_ident);
                    (*#target_ident)
                })
            } else {
                unsupported_assign_op("exponentiation")
            }
        }
        IrAssignOp::LeftShiftAssign
        | IrAssignOp::RightShiftAssign
        | IrAssignOp::UnsignedRightShiftAssign
        | IrAssignOp::BitwiseOrAssign
        | IrAssignOp::BitwiseXorAssign
        | IrAssignOp::BitwiseAndAssign
        | IrAssignOp::LogicalOrAssign
        | IrAssignOp::LogicalAndAssign
        | IrAssignOp::NullishCoalesceAssign => unsupported_assign_op("unsupported array assignment"),
    }
}

fn array_element_type(kind: IrArrayKind) -> IrType {
    match kind {
        IrArrayKind::Number => IrType::Number,
        IrArrayKind::Str => IrType::Str,
        IrArrayKind::Bool => IrType::Bool,
        IrArrayKind::Object(id) => IrType::Object(id),
        IrArrayKind::Value | IrArrayKind::Any | IrArrayKind::Unknown => IrType::Value,
    }
}

fn array_element_default(ty: &IrType) -> TokenStream {
    match ty {
        IrType::Number => quote! { 0.0f64 },
        IrType::UInt => quote! { 0usize },
        IrType::Str => quote! { ::std::string::String::new() },
        IrType::Bool => quote! { false },
        IrType::Unit => quote! { () },
        IrType::Any | IrType::Value => quote! { runtime::value::Value::Undefined },
        IrType::Array(_) => quote! { ::std::vec::Vec::new() },
        IrType::Object(_) => quote! { ::std::default::Default::default() },
    }
}

fn member_assignment_tokens(
    op: IrAssignOp,
    object: &IrExpression,
    property: &str,
    right: &IrExpression,
) -> TokenStream {
    if matches!(op, IrAssignOp::Assign) {
        if let Some(IrType::Object(id)) = typing::infer_expression_type(object) {
            if let Some(alias) = typing::lookup_type_alias(id) {
                if let IrTypeAliasDef::Object(fields) = alias.def {
                    if let Some(field) = fields.iter().find(|field| field.name == property) {
                        let object_tokens = object_tokens_for_member(object);
                        let field_ident = format_ident!("{}", property);
                        let right_tokens = right.codegen();
                        let right_ty = typing::infer_expression_type(right);
                        let coerced = typing::coerce_to_type(right_tokens, &field.ty, right_ty);
                        let assign_value = if typing::is_copy_type(&field.ty) {
                            quote! { ts_2_rs_value }
                        } else {
                            quote! { ts_2_rs_value.clone() }
                        };
                        return quote!({
                            let ts_2_rs_value = #coerced;
                            (#object_tokens).#field_ident = #assign_value;
                            ts_2_rs_value
                        });
                    }
                }
            }
        }
    }

    if let Some(IrType::Object(id)) = typing::infer_expression_type(object) {
        if let Some(alias) = typing::lookup_type_alias(id) {
            if let IrTypeAliasDef::Object(fields) = alias.def {
                if let Some(field) = fields.iter().find(|field| field.name == property) {
                    let numeric_field = matches!(field.ty, IrType::Number | IrType::UInt);
                    if numeric_field {
                        let object_tokens = object_tokens_for_member(object);
                        let field_ident = format_ident!("{}", property);
                        let right_tokens = right.codegen();
                        let right_ty = typing::infer_expression_type(right);
                        let rhs = typing::coerce_to_type(right_tokens, &field.ty, right_ty);
                        return match op {
                            IrAssignOp::AddAssign => quote!({
                                let ts_2_rs_rhs = #rhs;
                                let ts_2_rs_target = &mut #object_tokens;
                                ts_2_rs_target.#field_ident = ts_2_rs_target.#field_ident + ts_2_rs_rhs;
                                ts_2_rs_target.#field_ident
                            }),
                            IrAssignOp::SubAssign => quote!({
                                let ts_2_rs_rhs = #rhs;
                                let ts_2_rs_target = &mut #object_tokens;
                                ts_2_rs_target.#field_ident = ts_2_rs_target.#field_ident - ts_2_rs_rhs;
                                ts_2_rs_target.#field_ident
                            }),
                            IrAssignOp::MulAssign => quote!({
                                let ts_2_rs_rhs = #rhs;
                                let ts_2_rs_target = &mut #object_tokens;
                                ts_2_rs_target.#field_ident = ts_2_rs_target.#field_ident * ts_2_rs_rhs;
                                ts_2_rs_target.#field_ident
                            }),
                            IrAssignOp::DivAssign => quote!({
                                let ts_2_rs_rhs = #rhs;
                                let ts_2_rs_target = &mut #object_tokens;
                                ts_2_rs_target.#field_ident = ts_2_rs_target.#field_ident / ts_2_rs_rhs;
                                ts_2_rs_target.#field_ident
                            }),
                            IrAssignOp::ModAssign => quote!({
                                let ts_2_rs_rhs = #rhs;
                                let ts_2_rs_target = &mut #object_tokens;
                                ts_2_rs_target.#field_ident = ts_2_rs_target.#field_ident % ts_2_rs_rhs;
                                ts_2_rs_target.#field_ident
                            }),
                            IrAssignOp::ExpAssign => quote!({
                                let ts_2_rs_rhs = #rhs;
                                let ts_2_rs_target = &mut #object_tokens;
                                ts_2_rs_target.#field_ident = ts_2_rs_target.#field_ident.powf(ts_2_rs_rhs);
                                ts_2_rs_target.#field_ident
                            }),
                            _ => unsupported_assign_op("unsupported member assignment"),
                        };
                    }
                }
            }
        }
    }

    match op {
        IrAssignOp::Assign => member_simple_assign(object, property, right),
        IrAssignOp::AddAssign => member_value_op(quote!(runtime::value::ops::add), object, property, right),
        IrAssignOp::SubAssign => member_value_op(quote!(runtime::value::ops::sub), object, property, right),
        IrAssignOp::MulAssign => member_value_op(quote!(runtime::value::ops::mul), object, property, right),
        IrAssignOp::DivAssign => member_value_op(quote!(runtime::value::ops::div), object, property, right),
        IrAssignOp::ModAssign => member_value_op(quote!(runtime::value::ops::modulo), object, property, right),
        IrAssignOp::ExpAssign => member_exponent_assign(object, property, right),
        IrAssignOp::LeftShiftAssign => member_bitwise_assign(quote!(<<), object, property, right),
        IrAssignOp::RightShiftAssign => member_bitwise_assign(quote!(>>), object, property, right),
        IrAssignOp::BitwiseOrAssign => member_bitwise_assign(quote!(|), object, property, right),
        IrAssignOp::BitwiseXorAssign => member_bitwise_assign(quote!(^), object, property, right),
        IrAssignOp::BitwiseAndAssign => member_bitwise_assign(quote!(&), object, property, right),
        IrAssignOp::UnsignedRightShiftAssign
        | IrAssignOp::LogicalOrAssign
        | IrAssignOp::LogicalAndAssign
        | IrAssignOp::NullishCoalesceAssign => unsupported_assign_op("unsupported member assignment"),
    }
}

fn object_tokens_for_member(object: &IrExpression) -> TokenStream {
    if let IrExpression::Identifier(name) = object {
        if let Some(alias) = typing::lookup_object_alias(name) {
            return object_index_tokens(&alias.target, &alias.index);
        }
    }

    if let IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index { target, index, element })) = object {
        if matches!(element, Some(IrArrayKind::Object(_))) {
            return object_index_tokens(target.as_ref(), index.as_ref());
        }
    }

    object.codegen()
}

fn object_index_tokens(target: &IrExpression, index: &IrExpression) -> TokenStream {
    let target_tokens = target.codegen();
    let index_tokens = index.codegen();
    match typing::infer_expression_type(index) {
        Some(IrType::UInt) => quote! { #target_tokens[#index_tokens] },
        _ => quote! { #target_tokens[(#index_tokens) as usize] },
    }
}

fn member_simple_assign(object: &IrExpression, property: &str, right: &IrExpression) -> TokenStream {
    let object_tokens = object_tokens_for_member(object);
    let right_tokens = right.codegen();
    let property_literal = Literal::string(property);

    let bound_value = if typing::expr_is_copy_type(right) {
        quote! { #right_tokens }
    } else {
        quote! { (#right_tokens).clone() }
    };

    quote!({
        let ts_2_rs_value = #bound_value;
        let ts_2_rs_target = &mut #object_tokens;
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal, ts_2_rs_value.clone());
        ts_2_rs_value
    })
}

fn member_value_op(
    op_fn: TokenStream,
    object: &IrExpression,
    property: &str,
    right: &IrExpression,
) -> TokenStream {
    let object_tokens = object_tokens_for_member(object);
    let right_tokens = right.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    let bound_rhs = if typing::expr_is_copy_type(right) {
        quote! { #right_tokens }
    } else {
        quote! { (#right_tokens).clone() }
    };

    quote!({
        let ts_2_rs_rhs = #bound_rhs;
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_current = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal);
        let ts_2_rs_new = #op_fn(ts_2_rs_current, (ts_2_rs_rhs).clone());
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new.clone());
        ts_2_rs_new
    })
}

fn member_exponent_assign(
    object: &IrExpression,
    property: &str,
    right: &IrExpression,
) -> TokenStream {
    let object_tokens = object_tokens_for_member(object);
    let right_tokens = right.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    let bound_rhs = if typing::expr_is_copy_type(right) {
        quote! { #right_tokens }
    } else {
        quote! { (#right_tokens).clone() }
    };

    quote!({
        let ts_2_rs_rhs = #bound_rhs;
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_base = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal).into_number();
        let ts_2_rs_exp = (ts_2_rs_rhs).clone().into_number();
        let ts_2_rs_new = runtime::value::Value::Number(ts_2_rs_base.powf(ts_2_rs_exp));
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new.clone());
        ts_2_rs_new
    })
}

fn member_bitwise_assign(
    operator: TokenStream,
    object: &IrExpression,
    property: &str,
    right: &IrExpression,
) -> TokenStream {
    let object_tokens = object_tokens_for_member(object);
    let right_tokens = right.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    let bound_rhs = if typing::expr_is_copy_type(right) {
        quote! { #right_tokens }
    } else {
        quote! { (#right_tokens).clone() }
    };

    quote!({
        let ts_2_rs_rhs = #bound_rhs;
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_lhs = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal).into_number() as i64;
        let ts_2_rs_rhs_num = (ts_2_rs_rhs).clone().into_number() as i64;
        let ts_2_rs_new = runtime::value::Value::Number(((ts_2_rs_lhs) #operator (ts_2_rs_rhs_num)) as f64);
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new.clone());
        ts_2_rs_new
    })
}

fn value_compound_assignment(
    op_fn: TokenStream,
    target_ident: &proc_macro2::Ident,
    rhs_ident: &proc_macro2::Ident,
    left_tokens: &TokenStream,
    right_tokens: &TokenStream,
    rhs_is_copy: bool,
) -> TokenStream {
    let rhs_value = if rhs_is_copy {
        quote! { #right_tokens }
    } else {
        quote! { (#right_tokens).clone() }
    };
    quote!({
        let #rhs_ident = #rhs_value;
        let #target_ident = &mut #left_tokens;
        let ts_2_rs_new = #op_fn((*#target_ident).clone(), (#rhs_ident).clone());
        *#target_ident = ts_2_rs_new.clone();
        ts_2_rs_new
    })
}

fn exponent_assign_tokens(
    target_ident: &proc_macro2::Ident,
    rhs_ident: &proc_macro2::Ident,
    left_tokens: &TokenStream,
    right_tokens: &TokenStream,
    rhs_is_copy: bool,
) -> TokenStream {
    let rhs_value = if rhs_is_copy {
        quote! { #right_tokens }
    } else {
        quote! { (#right_tokens).clone() }
    };
    quote!({
        let #rhs_ident = #rhs_value;
        let #target_ident = &mut #left_tokens;
        let ts_2_rs_base = (*#target_ident).clone().into_number();
        let ts_2_rs_exp = (#rhs_ident).clone().into_number();
        let ts_2_rs_new = runtime::value::Value::Number(ts_2_rs_base.powf(ts_2_rs_exp));
        *#target_ident = ts_2_rs_new.clone();
        ts_2_rs_new
    })
}

fn bitwise_assignment(
    operator: TokenStream,
    target_ident: &proc_macro2::Ident,
    rhs_ident: &proc_macro2::Ident,
    left_tokens: &TokenStream,
    right_tokens: &TokenStream,
    rhs_is_copy: bool,
) -> TokenStream {
    let rhs_value = if rhs_is_copy {
        quote! { #right_tokens }
    } else {
        quote! { (#right_tokens).clone() }
    };
    quote!({
        let #rhs_ident = #rhs_value;
        let #target_ident = &mut #left_tokens;
        let ts_2_rs_lhs = (*#target_ident).clone().into_number() as i64;
        let ts_2_rs_rhs = (#rhs_ident).clone().into_number() as i64;
        let ts_2_rs_new = runtime::value::Value::Number(((ts_2_rs_lhs) #operator (ts_2_rs_rhs)) as f64);
        *#target_ident = ts_2_rs_new.clone();
        ts_2_rs_new
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral};
    use quote::quote;

    #[test]
    fn simple_assignment_returns_assigned_value() {
        let tokens = assignment_tokens(
            IrAssignOp::Assign,
            &IrExpression::Identifier("value".into()),
            &IrExpression::Literal(IrLiteral::Number(5.0)),
        );

        let expected = quote!({
            let ts_2_rs_value = 5;
            let ts_2_rs_target = &mut value;
            *ts_2_rs_target = (ts_2_rs_value).clone();
            ts_2_rs_value
        });

        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn add_assign_updates_and_returns_new_value() {
        let tokens = assignment_tokens(
            IrAssignOp::AddAssign,
            &IrExpression::Identifier("counter".into()),
            &IrExpression::Literal(IrLiteral::Number(2.0)),
        );

        let expected = quote!({
            let ts_2_rs_rhs = 2;
            let ts_2_rs_target = &mut counter;
            let ts_2_rs_new =
                runtime::value::ops::add((*ts_2_rs_target).clone(), (ts_2_rs_rhs).clone());
            *ts_2_rs_target = ts_2_rs_new.clone();
            ts_2_rs_new
        });

        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn exponent_assign_is_translated_using_powf() {
        let tokens = assignment_tokens(
            IrAssignOp::ExpAssign,
            &IrExpression::Identifier("base".into()),
            &IrExpression::Literal(IrLiteral::Number(3.0)),
        );

        let expected = quote!({
            let ts_2_rs_rhs = 3;
            let ts_2_rs_target = &mut base;
            let ts_2_rs_base = (*ts_2_rs_target).clone().into_number();
            let ts_2_rs_exp = (ts_2_rs_rhs).clone().into_number();
            let ts_2_rs_new = runtime::value::Value::Number(ts_2_rs_base.powf(ts_2_rs_exp));
            *ts_2_rs_target = ts_2_rs_new.clone();
            ts_2_rs_new
        });

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
