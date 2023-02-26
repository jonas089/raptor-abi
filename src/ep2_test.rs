#![no_std]
extern crate alloc;
use alloc::{vec, string::{String, ToString}};
use casper::{EntryPoint2};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs
};

pub fn test_ep_integration_EP2() -> u64{
    let example: EntryPoint2 = EntryPoint2{
        name:"total_supply".to_string(),
        args:vec![Parameter::new("string parameter", CLType::String), Parameter::new("integer parameter", CLType::U512)],
        ret:CLType::Unit,
        access:EntryPointAccess::Public,
        tp:EntryPointType::Contract
    };
    let ep: EntryPoint = example.forge();

    /*
    let eps: EntryPoints = EntryPoints::new();
    eps.add_entry_point(ep);
    */

    match ep {
        EntryPoint => {
            0 as u64
        },
        _ => {
            1 as u64
        }
    }
}

#[test]
fn test_ep(){
    panic!("No tests implemented");
}