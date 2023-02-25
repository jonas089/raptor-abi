use std::string::ToString;
use casper::EntryPoints;

fn main(){
    let example: EntryPoints = EntryPoints{name:"total_supply".to_string(), args:"some_args".to_string()};
    let res = example.my_trait_function();
    println!("Res: {}", res);
}

#[test]
fn test_ep(){

}
