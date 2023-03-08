extern crate serde_json;
use std::fs::{self, OpenOptions, File};
use std::io::{Write, Read};

pub fn dump_json(data: &Vec<Vec<String>>) -> std::io::Result<()> {
    let json_data = serde_json::to_string(data)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.json")?;
    let res = file.write_all(json_data.as_bytes())?;
    file.flush();
    Ok(res)
}

pub fn load_json() -> std::io::Result<Vec<Vec<String>>> {
    let mut file = File::open("output.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: Vec<Vec<String>> = serde_json::from_str(&contents)?;
    file.flush();
    Ok(data)
}

pub fn create_json_file(path: &str) -> std::io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true) // create the file if it doesn't exist
        .open(path)
}

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok() && fs::metadata(file_path).unwrap().is_file()
}
