use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug)]
struct BragEntry {
    content: String,
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
}

impl BragEntry {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            timestamp: Utc::now(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, content] if content != "view" && content != "--help" => {
            add_entry(BragEntry::new(content)).expect("Failed to add brag entry")
        }
        [_, flag] if flag == "view" => view_entries(false).expect("Failed to view brag entries"),
        [_, flag, raw_flag] if flag == "view" && raw_flag == "--raw" => {
            view_entries(true).expect("Failed to view brag entries in raw format")
        }
        [_, flag] if flag == "--help" => print_help(),
        _ => print_help(),
    }
}

fn print_help() {
    println!("Brag - A CLI tool to maintain a brag list\n");
    println!("Usage:");
    println!("  brag [MESSAGE]      Add a new entry with the message");
    println!("  brag view           View all brag entries");
    println!("  brag view --raw     View all brag entries in raw JSON format");
    println!("  brag --help         Show this help message");
}

fn brag_file_path() -> PathBuf {
    home_dir().expect("Could not find home directory").join(".brag_list.json")
}

fn add_entry(entry: BragEntry) -> Result<(), Box<dyn std::error::Error>> {
    let path = brag_file_path();
    let mut entries = if path.exists() {
        let file = fs::read_to_string(&path)?;
        serde_json::from_str(&file)?
    } else {
        Vec::new()
    };

    entries.push(entry);
    let serialized = serde_json::to_string(&entries)?;
    fs::write(path, serialized)?;
    println!("Brag entry added!");
    Ok(())
}

fn view_entries(raw: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = brag_file_path();
    if path.exists() {
        let file = fs::read_to_string(&path)?;
        if raw {
            println!("{}", file);
        } else {
            let mut entries: Vec<BragEntry> = serde_json::from_str(&file)?;
            entries.sort_by_key(|entry| entry.timestamp);
            if entries.is_empty() {
                println!("Your brag list is currently empty. Add some achievements!");
            } else {
                println!("Your Brag List:");
                for entry in entries {
                    println!("{}: {}", entry.timestamp.format("%Y-%m-%d %H:%M:%S"), entry.content);
                }
            }
        }
    } else {
        println!("Your brag list is currently empty. Start adding some achievements!");
    }
    Ok(())
}
