use std::{fs::create_dir, path::PathBuf};

pub fn mkdir(path: &PathBuf){
    if !path.exists() {
        create_dir(&path).unwrap();
    }
}
