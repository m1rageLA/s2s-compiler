use proc_macro2::TokenStream;
use quote::quote;

fn get_token_stream() -> TokenStream {
    let var = "peremennaja";
    let znaczenie = 10;
    let tokens = quote! {
        let #var = #znaczenie;
    };
    tokens
}

#[cfg(test)]
mod tests {
    use std::println;

use super::*;

    #[test]
    fn generate_token_stream() {
        let answ = get_token_stream();
        println!("{}", answ);
    }
}