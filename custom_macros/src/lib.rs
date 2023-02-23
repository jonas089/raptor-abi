//use proc_macro::TokenStream;
//use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Expr, Lit, parse_quote};
use std::{self, string};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
/*
#[proc_macro]
pub fn generate_ep(input: TokenStream) -> TokenStream{
    let args = input.to_string();
    let tokens = quote! {
        let res = #args;
        let newly_created_var:String = "hello".to_string();
        println!("Args: {}", #args);
    };
    tokens.into()
}
*/
#[proc_macro_derive(MyMacro)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate code to implement the MyTrait trait
    let trait_impl = quote! {

        fn my_trait_function(&self) -> String {
            "This code was generated at compile time!".to_string()
            // Implementation goes here
        }


    };

    // Get the name of the struct
    let struct_name = &input.ident;

    // Generate code to add the trait implementation to the struct
    let output = quote! {
        // Add the trait implementation to the struct
        impl #struct_name {
            #trait_impl
        }
    };

    // Return the generated code as a TokenStream
    output.into()
}
