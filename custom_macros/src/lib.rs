use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Expr, Lit, parse_quote};
use std::string;
#[proc_macro]
pub fn generate_main(input: TokenStream) -> TokenStream {
    let args = input.to_string();
    let tokens = quote! {
        let args = #args;
        let newly_created_var:String = "hello".to_string();
        println!("Args: {}", #args);
    };
    tokens.into()
}

#[proc_macro]
pub fn my_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);


    let mut metadata: Vec<Vec<String>> = Vec::new();
    // Check if the input is a struct
    if let Data::Struct(data) = input.data {
        // Iterate over the fields of the struct
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                // Get the name and type of the field
                let name = field.ident.unwrap();
                let ty = field.ty;
                let d = field.attrs;
                println!("{}", quote!(d).to_string());
                let ty_str = quote!(ty).to_string();
                // Do something with the name and type of the field
                let key = Vec::from([name.to_string(), ty_str]);
                metadata.push(key);
            }
        }
    }
    println!("{:?}", metadata);
    let _stream = quote! {
        let new_variable:String = "test".to_string();
    };
    // Return a dummy output
    _stream.into()
}
