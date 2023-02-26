mod example_contract;
use example_contract::test_ep_integration;
fn main(){
    let res: u64 = test_ep_integration();
    match res {
        0 => {
            println!("Success! [Entry Point] type match.");
        }
        1 => {
            panic!("Error! [Entry Point] type mistmatch.");
        }
        _ => {
            panic!("Error! Unknown.");
        }
    }
}
