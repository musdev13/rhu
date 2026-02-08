use serde_json::{Value, json};

pub fn raw(is_deleted: bool, save_name: &str) -> Value{
  json!({"save_name": save_name, "is_deleted": is_deleted}) 
} 
