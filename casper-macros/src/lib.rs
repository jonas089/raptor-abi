use std::{vec::Vec, path::PathBuf, fmt::{write, format}, f32::consts::E};
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
            panic!("Macro misuse - Input not of type 'Struct'!");
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

    let env_path_to_output: PathBuf = PathBuf::from(env::var("OUTPUT_PATH").expect("Missing required environment variable: OUTPUT_PATH"));

    let writer: MetaWriter = MetaWriter{
        env_path_to_output: env_path_to_output
    };
    // dump metadata at build time
    if !writer.file_exists(){
        writer.create_json_file().map_err(|e| format!("Failed to create file: {:?}", e));
        writer.dump_json(&vec![attributes]).map_err(|e| format!("Failed to dump json: {:?}", e));
    }
    else{
        let mut metadata = writer.load_json().map_err(|e| format!("Failed to load json: {:?}", e)).unwrap();
        metadata.push(attributes);
        writer.dump_json(&metadata).map_err(|e| format!("Failed to dump json: {:?}", e));
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
                    "Bool" => Parameter::new(name.clone(), CLType::Bool),
                    "I32" => Parameter::new(name.clone(), CLType::I32),
                    "I64" => Parameter::new(name.clone(), CLType::I64),
                    "U8" => Parameter::new(name.clone(), CLType::U8),
                    "U32" => Parameter::new(name.clone(), CLType::U32),
                    "U64" => Parameter::new(name.clone(), CLType::U64),
                    "U128" => Parameter::new(name.clone(), CLType::U128),
                    "U256" => Parameter::new(name.clone(), CLType::U256),
                    "U512" => Parameter::new(name.clone(), CLType::U512),
                    "Unit" => Parameter::new(name.clone(), CLType::Unit),
                    "String" => Parameter::new(name.clone(), CLType::String),
                    "Key" => Parameter::new(name.clone(), CLType::Key),
                    "URef" => Parameter::new(name.clone(), CLType::URef),
                    "PublicKey" => Parameter::new(name.clone(), CLType::PublicKey),
                    
                    "Any" => Parameter::new(name.clone(), CLType::Any),
                    _ => panic!("Unsupported type detected!"),
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
