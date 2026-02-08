use crate::core::{logger::title::title, worlds::list::get_list::get_list};

pub fn print(){
    let saves = get_list();
    
    // println!("== Hytale Universes/Saves: ==");
    title("Hytale Universes/Saves");
    for save in saves{
        println!("{}",save);
    }
    // println!("{:?}", saves);
}
