use std::fs::exists;

use crate::core::worlds::list::get_path::get_path;


pub fn is_exist(save_name: &str) -> bool{
    if !exists(get_path(save_name)).unwrap() {return false;}
    return true;
}
