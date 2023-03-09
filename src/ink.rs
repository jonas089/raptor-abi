/*
 Artificial Smart contract with only Entry Points used for testing.
*/
extern crate alloc;
use alloc::{vec, vec::Vec};

use helpers::types::{CasperKey, CasperU64, CasperString};
use code_generator::{InkCasperMacro};
use casper_types::{
    CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter
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
    let _result2 = ep2.get_params();

    let mut eps = EntryPoints::new();
    let entry_point = EntryPoint::new("test", result.clone(), CLType::Unit, EntryPointAccess::Public, EntryPointType::Contract);
    eps.add_entry_point(entry_point);

    99 as u64
}
