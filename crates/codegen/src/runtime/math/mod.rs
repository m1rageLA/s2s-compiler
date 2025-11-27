use ir::MathCall;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn math_call_tokens(call: &MathCall) -> TokenStream {
    match call {
        MathCall::Random => quote! { runtime::math::random() },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_random_call() {
        let tokens = math_call_tokens(&MathCall::Random);

        assert_eq!(
            tokens.to_string(),
            quote::quote! { runtime::math::random() }.to_string()
        );
    }
}
