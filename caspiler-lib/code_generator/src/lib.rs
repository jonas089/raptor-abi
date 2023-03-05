use syn::{parse_macro_input, DeriveInput, Data};
use proc_macro::TokenStream;
use casper_types::{
    CLType::U512, contracts::NamedKeys, runtime_args, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs
};
use quote::{quote, ToTokens};

use std::{string};

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
    // println!("ARGUMENT IDENT:{:?}", name_ident.ty.to_token_stream().to_string());
    let trait_impl = quote! {
        pub fn forge(&self) -> EntryPoint{
            EntryPoint::new()
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

// for research
#[proc_macro_derive(EpMacro)]
pub fn ep_derive(input: TokenStream) -> TokenStream {
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
    let name_ident = fields.iter().find(|f| f.ident.as_ref().unwrap().to_string() == "args").unwrap(); // find the field named "name" and get its identifier
    //println!("args:{:?}", name_ident);
/*
args:Field { attrs: [], vis: Public(VisPublic { pub_token: Pub }), ident: Some(Ident { ident: "args", span: #0 bytes(364..368) }),
colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None,
segments: [PathSegment { ident: Ident { ident: "Vec", span: #0 bytes(370..373) },
arguments: AngleBracketed(AngleBracketedGenericArguments { colon2_token: None, lt_token: Lt,
args: [Type(Path(TypePath { qself: None, path: Path { leading_colon: None,
segments: [PathSegment { ident: Ident { ident: "Parameter", span: #0 bytes(374..383) }, arguments: None }] } }))],
gt_token: Gt }) }] } }) }
*/
    println!("ARGUMENT IDENT:{:?}", name_ident.ty.to_token_stream().to_string());
    let trait_impl = quote! {
        pub fn meta_data(&self) -> String{
            self.name.clone().to_string()
        }
        pub fn forge(&self) -> EntryPoint{
            EntryPoint::new(self.name.clone(), self.args.clone(), self.ret.clone(), self.access.clone(), self.tp.clone())
        }
    };

    let struct_name = &input.ident;
    let output = quote! {
        impl #struct_name {
            #trait_impl
        }
    };
    output.into()
}
