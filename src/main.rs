use chrono::{serde::ts_seconds, DateTime, Utc};
use clap::{Parser, Subcommand};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{env, fs, io::Read, path::PathBuf, process};
use tempfile::NamedTempFile; // Add this at the top where other `use` statements are

#[derive(Serialize, Deserialize, Debug)]
struct BragEntry {
    content: String,
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
}

impl BragEntry {
    fn new(content: String) -> Self {
        Self {
            content,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Parser)]
struct Add {
    /// The brag message to add
    #[arg(short, long)]
    text: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Adds a new brag to your list or opens an editor to write the brag
    Add(Add),
    /// Views your brag list
    View {
        /// Display the raw JSON data
        #[arg(short, long)]
        raw: bool,
    },
    Edit,
    Remove,
}

#[derive(Debug, Parser)]
struct Brag {
    #[command(subcommand)]
    action: Action,
}

fn main() {
    let command = Brag::parse();
    println!("{:?}", command);

    match command.action {
        Action::Add(args) => handle_add(&args).unwrap(),
        Action::View { raw } => handle_view(raw).unwrap(),
        Action::Edit => handle_edit().unwrap(),
        Action::Remove => handle_remove().unwrap(),
    }
}

fn handle_add(args: &Add) -> Result<(), Box<dyn std::error::Error>> {
    match &args.text {
        Some(message) => save_entry(BragEntry::new(message.clone())),
        None => add_entry_from_editor(),
    }
}

fn brag_file_path() -> PathBuf {
    home_dir()
        .expect("Could not find home directory")
        .join(".brag_list.json")
}

fn add_entry_from_editor() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_owned();

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    process::Command::new(editor).arg(&temp_path).status()?;

    let mut file = std::fs::File::open(&temp_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if !content.trim().is_empty() {
        let entry = BragEntry::new(content.trim().to_string());
        save_entry(entry)?;
        println!("Brag entry added!");
    } else {
        println!("No content provided. No entry added.");
    }

    std::fs::remove_file(temp_path)?;

    Ok(())
}

fn save_entry(entry: BragEntry) -> Result<(), Box<dyn std::error::Error>> {
    let path = brag_file_path();
    let mut entries = if path.exists() {
        let file = fs::read_to_string(&path)?;
        serde_json::from_str(&file)?
    } else {
        Vec::new()
    };

    entries.push(entry);
    fs::write(path, serde_json::to_string(&entries)?)?;
    Ok(())
}

fn handle_view(raw: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = brag_file_path();
    if path.exists() {
        let file = fs::read_to_string(&path)?;
        if raw {
            println!("{}", file);
        } else {
            let entries: Vec<BragEntry> = serde_json::from_str(&file)?;
            if entries.is_empty() {
                println!("Your brag list is currently empty.");
            } else {
                for (index, entry) in entries.iter().enumerate() {
                    println!(
                        "{}. {}: {}",
                        index + 1,
                        entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        entry.content
                    );
                }
            }
        }
    } else {
        println!("Your brag list is currently empty.");
    }
    Ok(())
}

fn handle_edit() -> Result<(), Box<dyn std::error::Error>> {
    let path = brag_file_path();
    if !path.exists() {
        println!("Your brag list is currently empty.");
        return Ok(());
    }

    let file = fs::read_to_string(&path)?;
    let mut entries: Vec<BragEntry> = serde_json::from_str(&file)?;
    if entries.is_empty() {
        println!("Your brag list is currently empty.");
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an entry to edit")
        .default(0)
        .items(
            &entries
                .iter()
                .map(|e| e.content.as_str())
                .collect::<Vec<&str>>(),
        )
        .interact_on_opt(&Term::stderr())?;

    if let Some(index) = selection {
        let mut temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_owned();
        write!(temp_file.as_file_mut(), "{}", entries[index].content)?;

        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        process::Command::new(editor).arg(&temp_path).status()?;

        let mut updated_content = String::new();
        std::fs::File::open(&temp_path)?.read_to_string(&mut updated_content)?;

        entries[index].content = updated_content.trim().to_string();
        fs::write(path, serde_json::to_string(&entries)?)?;
        println!("Entry updated.");
    } else {
        println!("No entry selected.");
    }

    Ok(())
}
fn handle_remove() -> Result<(), Box<dyn std::error::Error>> {
    let path = brag_file_path();
    if !path.exists() {
        println!("Your brag list is currently empty.");
        return Ok(());
    }

    let file = fs::read_to_string(&path)?;
    let mut entries: Vec<BragEntry> = serde_json::from_str(&file)?;
    if entries.is_empty() {
        println!("Your brag list is currently empty.");
        return Ok(());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an entry to remove")
        .default(0)
        .items(
            &entries
                .iter()
                .map(|e| e.content.as_str())
                .collect::<Vec<&str>>(),
        )
        .interact_on_opt(&Term::stderr())?;

    if let Some(index) = selection {
        entries.remove(index);
        fs::write(path, serde_json::to_string(&entries)?)?;
        println!("Entry removed.");
    } else {
        println!("No entry removed.");
    }

    Ok(())
}
// Note: You'll need to re-implement view_entries, edit_entry, and remove_entry functions similar to previous examples.
// These functions will largely remain unchanged from their earlier implementations.
