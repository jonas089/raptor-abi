#![warn(unused_variables)]
#![warn(dead_code)]
mod ink;
use ink::ink_casper_contract;
fn main(){
    let _res:u64 = ink_casper_contract();
}
