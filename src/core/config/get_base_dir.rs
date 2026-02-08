use std::{env::current_exe, path::PathBuf};

pub fn get_base_dir() -> PathBuf{
    current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())).unwrap()
}
