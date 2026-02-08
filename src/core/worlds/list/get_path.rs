use std::path::PathBuf;

use crate::core::config::HYTALE_SAVES_PATH;

pub fn get_path(save_name: &str)->PathBuf{
    return (&*HYTALE_SAVES_PATH.join(save_name)).to_path_buf();
}
