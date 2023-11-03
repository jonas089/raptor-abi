//#![allow(dead_code)]
//#![allow(unreachable_patterns)]
#![no_std]
#![no_main]

extern crate alloc;
use alloc::{vec, vec::Vec};
use helpers::types::{String, U64, U256, U512, Key};
use casper_macros::{InkCasperMacro};
use casper_types::{
    CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter
};
extern crate serde_json;

pub fn init(){
    #[derive(Default)]
    #[derive(InkCasperMacro)]
    struct NewEntryPointArgs1{
        sender: Key,
        recipient: Key,
        amount: U64,
        id_first:U256,
        text: String
    }

    #[derive(Default)]
    #[derive(InkCasperMacro)]
    struct NewEntryPointArgs2{
        sender: Key,
        recipient: Key,
        amount: U64,
        id_first: U256,
        text: String
    }

    let ep = NewEntryPointArgs1::default();
    let ep2 = NewEntryPointArgs2::default();
    let result = ep.get_params();
    let _result2 = ep2.get_params();

    let mut eps = EntryPoints::new();
    let entry_point = EntryPoint::new("test", result.clone(), CLType::Unit, EntryPointAccess::Public, EntryPointType::Contract);
    eps.add_entry_point(entry_point);
}
