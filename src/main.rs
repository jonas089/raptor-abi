use std::string::ToString;
use casper::EntryPoints;

fn main(){
    let example: EntryPoints = EntryPoints{name:"total_supply".to_string(), args:"some_args".to_string()};
    // what it will look like

    /*
        let example: EntryPoint = EntryPoint(...);
        example.into => macro impl that creates a new EntryPoint form the struct and returns it as a TokenStream
        EntryPoints::add_entry_point(example.into())

        note that EntryPoint class is occupied, so maybe choose something else or not import it from casper_types

    */
    let res = example.my_trait_function();
    println!("Res: {}", res);
}

#[test]
fn test_ep(){

}
