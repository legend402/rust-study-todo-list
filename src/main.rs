mod cli;
mod command;
mod database;
mod utils;

use clap::Parser;
use cli::{ Cli, Commands };
use database::Database;

fn main() {
    // 获取环境变量中的参数
    let args = Cli::parse();
    
    let mut db = Database::open();
    // 判断参数是否为add
    let result = match args.command {
        Commands::Info => command::info(),
        Commands::Add { content } => command::add(&mut db, content),
        Commands::Remove { id } => command::remove(&mut db, id),
        Commands::List => command::list(&mut db),
    };

    if let Err(err) = result {
        eprintln!("\x1b[31merror:\x1b[39m {}", err);
        std::process::exit(1);
    }
}