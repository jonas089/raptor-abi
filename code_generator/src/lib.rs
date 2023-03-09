use std::vec::Vec;
use syn::{parse_macro_input, DeriveInput, Data};
use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
extern crate serde_json;
use helpers::meta::{dump_json, load_json, create_json_file, file_exists};

// proc macro for Entry Point ABI generation.
#[proc_macro_derive(InkCasperMacro)]
pub fn ink_derive(input: TokenStream) -> TokenStream {
    // Parse deadcode struct as TokenStream
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
    // Collect Type information from deadcode struct
    let mut attributes: Vec<Vec<String>> = Vec::new();
    attributes.push(vec!["Name".to_string(), struct_name.to_string()]);
    for attribute in fields.iter(){
        let a = vec![attribute.ident.to_token_stream().to_string(), attribute.ty.to_token_stream().to_string()];
        attributes.push(a);
    }
    // Split struct into name and type vector for easier use in quote! block
    let mut names:Vec<String> = Vec::new();
    let mut types:Vec<String> = Vec::new();
    for attribute in attributes.iter(){
        names.push(attribute[0].clone());
        types.push(attribute[1].clone());
    }
    // dump metadata at build time
    if file_exists("output.json") == false{
        match create_json_file("output.json"){
            Ok(_file) => {

                match dump_json(&attributes){
                    Ok(_r) => {

                    },
                    Err(_) => {
                        panic!("Failed to dump json.");
                    }
                }
            },
            Err(_) => {
                panic!("Failed to create File!");
            }
        };
    }
    else{
        match load_json(){
            Ok(_contents) => {
                //contents.extend(attributes.clone());
                match dump_json(&attributes){
                    Ok(_r) => {

                    },
                    Err(_) => {
                        panic!("Failed to dump json!");
                    }
                }
            },
            Err(_) => {
                panic!("Failed to load json!");
            }
        }
    }
    // add a trait to the deadcode struct that returns native Parameters for EPs
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
                    _ => Parameter::new("error", CLType::String),
                };
                params.push(param);
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
