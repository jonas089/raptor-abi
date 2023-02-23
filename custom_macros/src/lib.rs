//use proc_macro::TokenStream;
//use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Expr, Lit, parse_quote};
use std::string;


use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

#[proc_macro]
pub fn generate_ep(input: TokenStream) -> TokenStream{
    let i2 = input.clone();
    let args = input.to_string();
    let tokens = quote! {
        let res = #args;
        let newly_created_var:String = "hello".to_string();
        println!("Args: {}", #args);
    };
    tokens.into()
}
