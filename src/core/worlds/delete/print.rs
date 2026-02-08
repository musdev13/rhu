pub fn print(is_deleted: bool, save_name: &str){
   if is_deleted {println!("\"{}\" deleted",save_name);return;}
   println!("can't delete \"{}\"",save_name)
}

