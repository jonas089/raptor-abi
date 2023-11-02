use std::{vec::Vec, path::PathBuf, fmt::write};
use syn::{parse_macro_input, DeriveInput, Data};
use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use std::env;
extern crate serde_json;
use helpers::meta::MetaWriter;

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

    let env_path_to_output: PathBuf = PathBuf::from(env::var("OUTPUT_PATH").expect("Missing required environment variable: OUTPUT_PATH!"));

    let writer: MetaWriter = MetaWriter{
        env_path_to_output: env_path_to_output
    };
    // dump metadata at build time
    if !writer.file_exists(){
        match writer.create_json_file(){
            Ok(_file) => {

                match writer.dump_json(&attributes){
                    Ok(_r) => {

                    },
                    Err(_) => {
                        panic!("Failed to dump json!");
                    }
                }
            },
            Err(error) => {
                panic!("Failed to create File!: {:?}", error);
            }
        };
    }
    else{
        match writer.load_json(){
            Ok(_contents) => {
                //contents.extend(attributes.clone());
                match writer.dump_json(&attributes){
                    Ok(_r) => {

                    },
                    Err(_) => {
                        panic!("Failed to dump json!");
                    }
                }
            },
            Err(error) => {
                panic!("Failed to load json!: {:?}", error);
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
                    "CasperKey" => Parameter::new(name.clone(), CLType::Key),
                    "CasperU64" => Parameter::new(name.clone(), CLType::U64),
                    "CasperU256" => Parameter::new(name.clone(), CLType::U256),
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
