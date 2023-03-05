#![no_std]
extern crate alloc;
use alloc::{vec, string::{String, ToString}};

use code_generator::{InkCasperMacro};
use casper_types::{
    CLType::U512, contracts::NamedKeys, runtime_args, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs
};

pub fn ink_casper_contract() -> u64{
    #[derive(InkCasperMacro)]
    struct NewEntryPointArgs1{
        sender:String,
        recipient:String,
        amount:u64,
        id_first:u64
    }

    #[derive(InkCasperMacro)]
    struct NewEntryPointArgs2{
        sender:String,
        recipient:String,
        amount:u64,
        id_second:u64
    }


/*
    let ep: EntryPoint = example.forge();
    match ep {
        EntryPoint => {
            0 as u64
        },
        _ => {
            1 as u64
        }
    }
*/
    99 as u64
}
