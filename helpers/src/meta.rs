extern crate serde_json;
use std::fs;
pub fn dump_json(metadata:String){
    fs::write("output.json", metadata);
}
