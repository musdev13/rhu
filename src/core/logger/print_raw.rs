use serde_json::{Value, to_string_pretty};

pub fn print_raw(json: Value){
    println!("{}", to_string_pretty(&json).unwrap());
}
