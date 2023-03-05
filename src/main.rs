//mod ep2_test;
mod ink;
//use ep2_test::test_ep_integration_EP2;
use ink::ink_casper_contract;
fn main(){
    /*
    let res: u64 = test_ep_integration_EP2();
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
    */
    let res:u64 = ink_casper_contract();
}
