use clap::{Parser, Subcommand};
use colored::*;

use crate::services::NoteService;
use crate::models::Priority;

#[derive(Parser)]
#[command(name = "ezn")]
#[command(about = "⚡ Zero-friction note taking for developers", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new note
    Add {
        /// Note content
        content: String,
        
        /// Add tags (can specify multiple times)
        #[arg(short, long)]
        tag: Vec<String>,
        
        /// Set priority (low, medium, high, urgent)
        #[arg(short, long, default_value = "medium")]
        priority: String,
    },
    
    /// List notes
    List {
        /// Show only today's notes
        #[arg(long)]
        today: bool,
        
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
        
        /// Limit number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    
    /// Search notes
    Search {
        /// Search query
        query: String,
    },
    
    /// Show note details
    Show {
        /// Note ID
        id: i64,
    },
    
    /// Delete a note
    Delete {
        /// Note ID
        id: i64,
        
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    
    /// Show today's notes
    Today,
    
    /// Show statistics
    Stats,
}

impl Cli {
    pub fn execute(&self, note_service: &NoteService) -> anyhow::Result<()> {
        match &self.command {
            Commands::Add { content, tag, priority } => {
                let priority = Priority::from_str(priority)?;
                let note = note_service.add_note(content, tag.clone(), priority)?;
                
                println!("{} Note added with ID: {}", 
                    "✓".green().bold(), 
                    note.id.unwrap().to_string().cyan().bold()
                );
            }
            
            Commands::List { today, tag, limit } => {
                let notes = if *today {
                    note_service.list_today(*limit)?
                } else if let Some(tag_filter) = tag {
                    note_service.list_by_tag(tag_filter, *limit)?
                } else {
                    note_service.list_recent(*limit)?
                };
                
                if notes.is_empty() {
                    println!("{}", "No notes found".dimmed());
                } else {
                    for note in notes {
                        print_note(&note);
                        println!();
                    }
                }
            }
            
            Commands::Search { query } => {
                let notes = note_service.search(query)?;
                
                if notes.is_empty() {
                    println!("{}", format!("No notes found matching '{}'", query).dimmed());
                } else {
                    println!("Found {} notes:", notes.len());
                    println!();
                    for note in notes {
                        print_note(&note);
                        println!();
                    }
                }
            }
            
            Commands::Show { id } => {
                let note = note_service.get_note(*id)?;
                print_note_detailed(&note);
            }
            
            Commands::Delete { id, force } => {
                // Check if note exists first
                if let Err(_) = note_service.get_note(*id) {
                    return Err(anyhow::anyhow!("Note {} not found", id));
                }
                
                if !force {
                    println!("Delete note {}? (y/N)", id.to_string().cyan());
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    if !input.trim().eq_ignore_ascii_case("y") {
                        println!("Cancelled");
                        return Ok(());
                    }
                }
                
                note_service.delete_note(*id)?;
                println!("{} Note {} deleted", "✓".green().bold(), id);
            }
            
            Commands::Today => {
                let notes = note_service.list_today(100)?;
                
                if notes.is_empty() {
                    println!("{}", "No notes today yet. Add one with: ezn add \"your note\"".dimmed());
                } else {
                    println!("{}", format!("📝 Today's Notes ({})", notes.len()).bold());
                    println!();
                    for note in notes {
                        print_note(&note);
                        println!();
                    }
                }
            }
            
            Commands::Stats => {
                let stats = note_service.get_stats()?;
                
                println!("{}", "📊 Statistics".bold());
                println!();
                println!("Total notes:     {}", stats.total.to_string().cyan());
                println!("Today:           {}", stats.today.to_string().cyan());
                println!("This week:       {}", stats.week.to_string().cyan());
                println!("This month:      {}", stats.month.to_string().cyan());
                println!();
                println!("By Priority:");
                println!("  Urgent:        {}", stats.urgent.to_string().red());
                println!("  High:          {}", stats.high.to_string().yellow());
                println!("  Medium:        {}", stats.medium.to_string().normal());
                println!("  Low:           {}", stats.low.to_string().dimmed());
            }
        }
        
        Ok(())
    }
}

fn print_note(note: &crate::models::Note) {
    let id_str = format!("[{}]", note.id.unwrap()).bright_blue().bold();
    let priority_str = format_priority(&note.priority);
    let time_str = note.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed();
    
    println!("{} {} {}", id_str, priority_str, time_str);
    println!("  {}", note.content);
    
    if !note.tags.is_empty() {
        let tags_str = note.tags.iter()
            .map(|t| format!("#{}", t).cyan().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("  {}", tags_str);
    }
}

fn print_note_detailed(note: &crate::models::Note) {
    println!();
    println!("{}", format!("Note #{}", note.id.unwrap()).bold());
    println!("{}", "─".repeat(50).dimmed());
    println!();
    println!("{}", note.content);
    println!();
    println!("{}", "─".repeat(50).dimmed());
    println!("Priority:  {}", format_priority(&note.priority));
    println!("Created:   {}", note.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("Updated:   {}", note.updated_at.format("%Y-%m-%d %H:%M:%S"));
    
    if !note.tags.is_empty() {
        println!("Tags:      {}", note.tags.iter()
            .map(|t| format!("#{}", t).cyan().to_string())
            .collect::<Vec<_>>()
            .join(", "));
    }
    println!();
}

fn format_priority(priority: &Priority) -> ColoredString {
    match priority {
        Priority::Urgent => "URGENT".red().bold(),
        Priority::High => "HIGH".yellow(),
        Priority::Medium => "MEDIUM".normal(),
        Priority::Low => "LOW".dimmed(),
    }
}
