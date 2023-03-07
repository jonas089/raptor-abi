#![no_std]
extern crate alloc;
use alloc::{vec, string::{String, ToString}};

use helpers::types::{CasperKey, CasperU64, CasperU512, CasperString};
use code_generator::{InkCasperMacro};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs
};
extern crate serde_json;

pub fn ink_casper_contract() -> u64{
    #[derive(Default)]
    #[derive(InkCasperMacro)]
    struct NewEntryPointArgs1{
        sender:CasperString,
        recipient:CasperString,
        amount:CasperU64,
        id_first:CasperU64,
        key:CasperKey
    }

    #[derive(Default)]
    #[derive(InkCasperMacro)]
    struct NewEntryPointArgs2{
        sender:CasperString,
        recipient:CasperString,
        amount:CasperU64,
        id_first:CasperU64,
        key:CasperKey
    }

    let ep = NewEntryPointArgs1::default();
    let ep2 = NewEntryPointArgs2::default();
    let result = ep.get_params();
    let result2 = ep2.get_params();

    println!("Parameters of Entry Point #1: {:?}", result);
    for _ep in result.iter(){
        match _ep{
            Parameter => {println!("Success!")},
            _ => {panic!("Invalid type!")}
        }
    }
    99 as u64
}
