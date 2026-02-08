use serde_json::{Value, json};

use crate::core::worlds::list::get_list::get_list;


pub fn raw() -> Value{
    json!({
        "saves": get_list()
    })
}
