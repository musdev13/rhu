use std::{fs::write, path::PathBuf};
use serde_json::{Value, to_string_pretty};

#[allow(dead_code)]
pub fn write_json(json_file_path: PathBuf, value: Value){
    write(json_file_path, to_string_pretty(&value).unwrap()).unwrap();    
}
