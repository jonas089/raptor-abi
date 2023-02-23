use std::string::ToString;
use std::fmt;
use casper::{EntryPoint, build_entry_point};
use custom_macros::MyMacro;

#[derive(MyMacro)]
struct EntryPoints{
    name:String,
    args:String
}

impl fmt::Display for EntryPoints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Args: {}", self.name, self.args)
    }
}

fn main(){
    let example: EntryPoints = EntryPoints{name:"total_supply".to_string(), args:"vec![]".to_string()};
    let res = example.my_trait_function();
    println!("Res: {}", res);
}

#[test]
fn test_ep(){

}
