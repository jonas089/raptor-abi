extern crate serde_json;
use std::fs;

pub fn dump_json(data: &Vec<Vec<String>>) -> std::io::Result<()> {
    let json_data = serde_json::to_string(data)?;
    std::fs::write("output.json", json_data)
}
pub fn load_json() -> std::io::Result<Vec<Vec<String>>> {
    let file_contents = std::fs::read_to_string("output.json")?;
    let data: Vec<Vec<String>> = serde_json::from_str(&file_contents)?;
    Ok(data)
}
pub fn create_json_file(path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let res = metadata.is_file();
        if res == false{
            fs::File::create(path);
        }
        metadata.is_file()
    } else {
        fs::File::create(path);
        false
    }
}
