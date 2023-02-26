use syn::{parse_macro_input, DeriveInput, Data};
use proc_macro::TokenStream;

use std::{string};

use casper_types::{EntryPoint};
use quote::{quote, ToTokens};

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
