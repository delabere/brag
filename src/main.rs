use clap::{App, Arg, SubCommand, ArgMatches, AppSettings};
use chrono::{DateTime, Utc, serde::ts_seconds};
use console::Term;
use dialoguer::{Select, theme::ColorfulTheme};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{env, fs, io::Read, path::PathBuf, process::Command};
use tempfile::NamedTempFile;
use std::io::Write; // Add this at the top where other `use` statements are

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

fn main() {
    let matches = App::new("Brag")
        .version("1.0")
        .author("Jack Rickards jackrickards@hotmail.co.uk")
        .about("Maintains a brag list")
        .setting(AppSettings::ArgRequiredElseHelp) // This line enforces showing help if no args
        .subcommand(SubCommand::with_name("add")
            .about("Adds a new brag to your list or opens an editor to write the brag")
            .arg(Arg::with_name("MESSAGE")
                .help("The brag message to add")
                .index(1)))
        .subcommand(SubCommand::with_name("view")
            .about("Views your brag list")
            .arg(Arg::with_name("raw")
                .long("raw")
                .help("Displays the raw JSON data")))
        .subcommand(SubCommand::with_name("edit")
            .about("Edits an existing brag entry"))
        .subcommand(SubCommand::with_name("remove")
            .about("Removes an entry from your brag list"))
        .get_matches();

    // Corrected match statements with Some wrapping
    if let Some(add_matches) = matches.subcommand_matches("add") {
        handle_add(add_matches).expect("Failed to add brag entry");
    } else if let Some(view_matches) = matches.subcommand_matches("view") {
        let raw = view_matches.is_present("raw");
        view_entries(raw).expect("Failed to view brag entries");
    } else if matches.subcommand_matches("edit").is_some() {
        edit_entry().expect("Failed to edit brag entry");
    } else if matches.subcommand_matches("remove").is_some() {
        remove_entry().expect("Failed to remove brag entry");
    }
}

fn handle_add(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(message) = matches.value_of("MESSAGE") {
        save_entry(BragEntry::new(message.to_string()))?;
        println!("Brag entry added!");
    } else {
        add_entry_from_editor()?;
    }
    Ok(())
}

// Implement add_entry_from_editor, save_entry, view_entries, edit_entry, remove_entry, and utility functions here...

fn brag_file_path() -> PathBuf {
    home_dir().expect("Could not find home directory").join(".brag_list.json")
}

fn add_entry_from_editor() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_owned();

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    Command::new(editor)
        .arg(&temp_path)
        .status()?;

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

    // Optionally delete the temp file here if you want to clean up
    // std::fs::remove_file(temp_path)?;

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

fn view_entries(raw: bool) -> Result<(), Box<dyn std::error::Error>> {
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
                    println!("{}. {}: {}", index + 1, entry.timestamp.format("%Y-%m-%d %H:%M:%S"), entry.content);
                }
            }
        }
    } else {
        println!("Your brag list is currently empty.");
    }
    Ok(())
}

fn edit_entry() -> Result<(), Box<dyn std::error::Error>> {
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
        .items(&entries.iter().map(|e| e.content.as_str()).collect::<Vec<&str>>())
        .interact_on_opt(&Term::stderr())?;

    if let Some(index) = selection {
        let mut temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_owned();
        write!(temp_file.as_file_mut(), "{}", entries[index].content)?;

        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        Command::new(editor)
            .arg(&temp_path)
            .status()?;

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
fn remove_entry() -> Result<(), Box<dyn std::error::Error>> {
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
        .items(&entries.iter().map(|e| e.content.as_str()).collect::<Vec<&str>>())
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
