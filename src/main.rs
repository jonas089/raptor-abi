mod ink;
use ink::ink_casper_contract;
use helpers::meta::create_json_file;
fn main(){
    create_json_file("output.json");
    let res:u64 = ink_casper_contract();
}
