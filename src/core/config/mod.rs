use serde_json::{Value, to_string_pretty};

use crate::core::config::default_json::default_json;
use crate::core::config::get_config_dir::get_config_dir;
use crate::core::fs;
use crate::core::fs::json::read_json::read_json;

pub mod get_base_dir;
pub mod get_config_dir;
pub mod default_json;

use std::sync::LazyLock;
use std::path::PathBuf;

pub static RHU_CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| get_config_dir().join("config.json"));
pub static RHU_CONFIG: LazyLock<Value> = LazyLock::new(|| read_json(RHU_CONFIG_PATH.clone(), &to_string_pretty(&default_json()).unwrap()));

pub static HYTALE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path_str = RHU_CONFIG
        .get("hytale_saves_path")
        .and_then(|v| v.as_str())
        .expect("hytale_path missing in config");
    fs::path_expand::path_expand(path_str)
});
pub static HYTALE_SAVES_PATH: LazyLock<PathBuf> = LazyLock::new(|| HYTALE_PATH.join("Saves"));

