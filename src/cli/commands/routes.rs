use crate::cli::commands;


pub fn routes(command: commands::Commands){
    match command {
        commands::Commands::Worlds {command} => match command {
            commands::worlds::Commands::List(args) => commands::worlds::list::list(args),
            commands::worlds::Commands::Import => println!("Import"),
            commands::worlds::Commands::Export => println!("Export"),
            commands::worlds::Commands::Delete(args) => commands::worlds::delete::delete(args),
        }
    }
}
