use serde_json::{Value, json};

pub fn default_json() -> Value{
    json!({
        "hytale_saves_path": "~/.hytalesaves/",
    })
}
