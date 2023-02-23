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
    let example = EntryPoints{name:"jonas".to_string(), args:"test_args".to_string()};

    println!("{}", new_variable);
}

#[test]
fn test_entry_point_struct(){
    let example: EntryPoints = EntryPoints{name:"total_supply".to_string(), args:"vec![]".to_string()};
}
