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

    let mut NAMES:Vec<String> = Vec::new();
    let mut TYPES:Vec<String> = Vec::new();
    for attribute in attributes.iter(){
        NAMES.push(attribute[0].clone());
        TYPES.push(attribute[1].clone());
    }
    let _meta = load_json();
    match _meta{
        Ok(m) => {
            let mut _m = m.clone();
            for a in attributes.clone().iter(){
                println!("attribute to be pushed: {:?}", a);
                _m.push((*a.clone()).to_vec());
            }
            dump_json(&_m);
        },
        Err(_) => {
            dump_json(&attributes);
        }
    }
    let trait_impl = quote! {
        pub fn get_params(&self) -> Vec<Parameter>{
            let mut params: Vec<Parameter> = Vec::new();
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
    let output = quote! {
        impl #struct_name {
            #trait_impl
        }
    };
    output.into()
}
