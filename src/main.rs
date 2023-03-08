#![allow(dead_code)]
#[allow(unreachable_patterns)]
mod ink;
use ink::ink_casper_contract;
fn main(){
    let _res:u64 = ink_casper_contract();
}
