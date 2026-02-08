use std::fs::{remove_dir_all};

use crate::core::{worlds::list::{get_path::get_path, is_exist::is_exist}};

pub fn delete(save_name: &str)->bool{
    //println!("{}", get_path(save_name).display());
  if is_exist(save_name){let _ = remove_dir_all(get_path(save_name)).is_ok(); return true;}
  false
}
