mod example_contract;
use example_contract::test_ep_integration;
fn main(){
    let res: String = test_ep_integration();
    println!("Example compilation result: {}", res);
}
