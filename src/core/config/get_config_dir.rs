use std::{path::PathBuf};

use crate::core::{config::get_base_dir::get_base_dir, fs::mkdir::mkdir};

pub fn get_config_dir() -> PathBuf{
    let config_dir_path = get_base_dir().join("config");
    mkdir(&config_dir_path);
    config_dir_path
}
