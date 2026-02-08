use std::fs;
use crate::core::config::HYTALE_SAVES_PATH;

pub fn get_list() -> Vec<String>{
    let mut saves = Vec::new();
    let saves_dir = &*HYTALE_SAVES_PATH;

    let entries = match fs::read_dir(saves_dir){
        Ok(e) => e,
        Err(_) => return saves,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name() {
                if let Some(name) = name.to_str(){
                    saves.push(name.to_string());
                }
            }
        }
    }
    saves
}

