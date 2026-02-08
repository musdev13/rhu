use crate::{cli::args::worlds::list::ListArgs, core::{logger::print_raw::print_raw, worlds::list::{print::print, raw::raw}}};

pub fn list(args: ListArgs){
    if args.raw {print_raw(raw()); return;}
    print();
}
