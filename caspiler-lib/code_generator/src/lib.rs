use syn::{parse_macro_input, DeriveInput, Data};
use proc_macro::TokenStream;
use casper_types::{EntryPoint};
use quote::{quote, ToTokens};

use std::{string};

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
    let trait_impl = quote! {
        pub fn meta_data(&self) -> String{
            self.name.clone().to_string()
        }
        pub fn forge(&self) -> EntryPoint{
            EntryPoint::new(self.name.clone(), self.args.clone(), self.ret.clone(), self.access.clone(), self.tp.clone())
            // TBD: dump EP as json
            // .. then, combine json and .wasm
            // => write a script that compiles to wasm and then combines .wasm and .json into .contract
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

// use this macro for code that is supposed to run only at compiletime.
#[proc_macro_derive(Cmp)]
pub fn compile_derive(input: TokenStream) -> TokenStream{

    TokenStream::new()
}

#[proc_macro_derive(FCmp)]
pub fn compile_fn_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let output = quote! {
        fn #name() {}
    };
    output.into()
}
