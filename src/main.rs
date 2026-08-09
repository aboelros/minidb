

use clap::Parser;
use minidb::cli::{self, Cli};
use minidb::error;

fn main() -> Result<(), error::MiniDbError> {
    let args = Cli::parse();
    
    println!("Starting MiniDB...");
    
    if let Some(cmd) = args.command {
        match cmd {
            cli::Commands::Run { script_path } => {
                cli::run_script(&script_path)?;
            }
        }
    } else if let Some(db_path) = args.db_path {
        println!("MiniDB v0.1.0");
        println!("Database: {}", db_path.display());
        cli::start_repl(&db_path)?;
    } else {
        println!("Usage: minidb [COMMAND] <db_path>");
        println!("Commands:");
        println!("  run <script_path>    Execute a SQL script file");
    }

    Ok(())
}
