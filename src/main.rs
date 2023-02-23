use std::string::ToString;
use std::fmt;
use casper::{EntryPoint, build_entry_point};
#[macro_use]
use custom_macros::generate_ep;
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
    let example = EntryPoints{name:"jonas".to_string(), args:"test_args".to_string()};
    let ep = EntryPoint{
        ident: "test".to_string(),
        args: "vec![]".to_string(),
        ret: "Unit".to_string(),
        ty: "Public".to_string()
    };
    generate_ep!(build_entry_point(ep));
    println!("{}", res);
}

#[test]
fn test_entry_point_struct(){
    let example: EntryPoints = EntryPoints{name:"total_supply".to_string(), args:"vec![]".to_string()};
}
