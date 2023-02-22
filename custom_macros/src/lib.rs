use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
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

/*
#[proc_macro_derive(Entries)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    /*
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(ref data) => match &data.fields {
            syn::Fields::Named(fields) => fields.named.iter(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    let field_replacements = fields.map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote! { stringify!(#name): format!("{:?}", #name), }
    }).collect::<Vec<_>>();
    */

    let ast: DeriveInput = syn::parse(input).unwrap();

    let fields = match ast.data {
        syn::Data::Struct(ref data) => match &data.fields {
            syn::Fields::Named(fields) => fields.named.iter(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let name_field = fields.clone().find(|f| f.ident.as_ref().unwrap() == "name").unwrap();
    let args_field = fields.clone().find(|f| f.ident.as_ref().unwrap() == "args").unwrap();

    let name = quote! { #name_field }.to_string();
    let args = quote! { #args_field }.to_string();

    let gen = quote! {
        let name = #name;
        let args = #args;
    };
    gen.into()
}
*/
