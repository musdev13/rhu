use std::fs::{read_to_string, write};
use std::path::PathBuf;
use serde_json::{Value, from_str};

pub fn read_json(json_file_path: PathBuf, default: &str) -> Value {
    if !json_file_path.exists() {
        let _ = write(&json_file_path, &default);
        return from_str(default).unwrap_or(Value::Null);
    }

    let text = match read_to_string(&json_file_path) {
        Ok(t) => t,
        Err(_) => return Value::Null,
    };

    serde_json::from_str(&text).unwrap_or(Value::Null)
}
