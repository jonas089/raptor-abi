use std::string::ToString;
use std::fmt;
#[macro_use]
extern crate custom_macros;

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
    generate_main!("total_supply", "vec![]");
    println!("{}", args);
    //let example: EntryPoints = EntryPoints{name:"total_supply".to_string(), args:"vec![]".to_string()};
    //println!("Example: {}", example);
}

#[test]
fn test_entry_point_struct(){
    let example: EntryPoints = EntryPoints{name:"total_supply".to_string(), args:"vec![]".to_string()};
}
