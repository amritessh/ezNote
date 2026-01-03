use clap::Parser;
use colored::*;

mod cli;
mod db;
mod models;
mod services;

use cli::Cli;
use db::Database;
use services::NoteService;

fn main() -> anyhow::Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Initialize database
    let db = Database::new()?;
    let note_service = NoteService::new(db);
    
    // Execute command
    match cli.execute(&note_service) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}
