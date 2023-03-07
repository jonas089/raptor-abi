use std::vec::Vec;
use syn::{parse_macro_input, DeriveInput, Data, Ident};
use proc_macro::{TokenStream};
use proc_macro2::{TokenTree};
use casper_types::{
    CLType::U512, contracts::NamedKeys, runtime_args, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs
};
use quote::{quote, ToTokens, TokenStreamExt, quote_spanned};
use std::{string};
extern crate serde_json;
use helpers::meta::{dump_json, load_json, create_json_file};

// proc macro for Entry Point ABI generation.
#[proc_macro_derive(InkCasperMacro)]
pub fn ink_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = match &input.data{
        Data::Struct(data_struct) => {
            data_struct
        },
        _ => {
            panic!("Input is not of type struct!");
        }
    };
    let fields = &data.fields;
    let struct_name = &input.ident;
    let mut attributes: Vec<Vec<String>> = Vec::new();
    attributes.push(vec!["Name".to_string(), struct_name.to_string()]);
    for attribute in fields.iter(){
        let a = vec![attribute.ident.to_token_stream().to_string(), attribute.ty.to_token_stream().to_string()];
        attributes.push(a);
    }

    let mut names:Vec<String> = Vec::new();
    let mut types:Vec<String> = Vec::new();
    for attribute in attributes.iter(){
        names.push(attribute[0].clone());
        types.push(attribute[1].clone());
    }
    match load_json() {
        Ok(mut meta) => {
            meta.extend(attributes);
            dump_json(&meta);
        },
        Err(_) => {
            dump_json(&attributes);
        }
    }
    let trait_impl = quote! {
        pub fn get_params(&self) -> Vec<Parameter>{
            let mut params: Vec<Parameter> = Vec::new();
            let names = vec![#(#names),*];
            let types = vec![#(#types),*];
            let mut params = Vec::new();
            for (name, ty) in names.iter().zip(types.iter()).skip(1) {
                let param = match *ty {
                    "CasperString" => Parameter::new(name.clone(), CLType::String),
                    "CasperU64" => Parameter::new(name.clone(), CLType::U64),
                    "CasperKey" => Parameter::new(name.clone(), CLType::Key),
                    "CasperU512" => Parameter::new(name.clone(), CLType::U512),
                    _ => panic!("Unsupported type: {}", ty),
                };
                params.push(param);
            }
            println!("PARAMETERS: {:?}", params);
            params
        }
    };
    let output = quote! {
        impl #struct_name {
            #trait_impl
        }
    };
    output.into()
}
