extern crate serde_json;
use std::fs::{self, OpenOptions, File};
use std::io::{Write, Read};
use std::path::PathBuf;

pub struct MetaWriter{
    pub env_path_to_output: PathBuf
}

impl MetaWriter{
    pub fn dump_json(&self, data: &Vec<Vec<String>>) -> std::io::Result<()> {
        let json_data = serde_json::to_string(data)?;
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.env_path_to_output.clone())?;
        let res = file.write_all(json_data.as_bytes())?;
        Ok(res)
    }
    
    pub fn load_json(&self) -> std::io::Result<Vec<Vec<String>>> {
        let mut file = File::open(self.env_path_to_output.clone())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Vec<Vec<String>> = serde_json::from_str(&contents)?;
        Ok(data)
    }
    
    pub fn create_json_file(&self) -> std::io::Result<File> {
        OpenOptions::new()
            .write(true)
            .create(true) // create the file if it doesn't exist
            .open(&self.env_path_to_output)
    }
    
    pub fn file_exists(&self) -> bool {
        fs::metadata(&self.env_path_to_output).is_ok() && fs::metadata(&self.env_path_to_output).unwrap().is_file()
    }
    
}