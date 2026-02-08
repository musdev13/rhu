use crate::{cli::args::worlds::delete::DeleteArgs, core::{logger::print_raw::print_raw, worlds::delete::{delete::delete as delete_world, print::print, raw::raw}}};

pub fn delete(args: DeleteArgs){
    let delete_status = delete_world(&args.save_name);
    if args.raw {print_raw(raw(delete_status, &args.save_name)); return;}
   print(delete_status, &args.save_name); 
}
