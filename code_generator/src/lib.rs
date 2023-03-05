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

// solution
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
    //let name_ident = fields.iter().find(|f| f.ident.as_ref().unwrap().to_string() == "args").unwrap();
    for attribute in fields.iter(){
        println!("IDENTIFIER: {:?}", attribute.ident);
        let a = vec![attribute.ident.to_token_stream().to_string(), attribute.ty.to_token_stream().to_string()];
        println!("Found Attribute: {:?}", attribute.ty.to_token_stream().to_string());
        attributes.push(a);
    }
    println!("Attributes for this Entry Point: {:?}", attributes);
    let mut NAMES:Vec<String> = Vec::new();
    let mut TYPES:Vec<String> = Vec::new();

    for attribute in attributes.iter(){
        NAMES.push(attribute[0].clone());
        TYPES.push(attribute[1].clone());
    }
    println!("NAMES:{:?} TYPES:{:?}", NAMES, TYPES);
    let trait_impl = quote! {
        pub fn get_params(&self) -> Vec<Parameter>{
            let mut params: Vec<Parameter> = Vec::new();
            //let attributes = vec![#(#attributes),*];
            let NAMES = vec![#(#NAMES),*];
            let TYPES = vec![#(#TYPES),*];
            for i in 0..NAMES.len(){
                if NAMES[i] != "Name"{
                    if TYPES[i] == "CasperString"{
                        params.push(Parameter::new(NAMES[i], CLType::String));
                    }
                    else if TYPES[i] == "CasperU64"{
                        params.push(Parameter::new(NAMES[i], CLType::U64));
                    }
                    else if TYPES[i] == "CasperKey"{
                        params.push(Parameter::new(NAMES[i], CLType::Key));
                    }
                    else if TYPES[i] == "CasperU512"{
                        params.push(Parameter::new(NAMES[i], CLType::U512));
                    }
                }
            }
            params
        }
    };
    println!("Struct Name:{:?}", struct_name);
    let output = quote! {
        impl #struct_name {
            #trait_impl
        }
    };
    output.into()
}
