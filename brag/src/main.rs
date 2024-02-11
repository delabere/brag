use clap::{App, Arg, SubCommand};
use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
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
    let matches = App::new("Brag")
        .version("1.0")
        .author("Your Name")
        .about("Maintains a brag list")
        .subcommand(SubCommand::with_name("add")
            .about("Adds a new brag to your list")
            .arg(Arg::with_name("MESSAGE")
                .help("The brag message to add")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("view")
            .about("Views your brag list")
            .arg(Arg::with_name("raw")
                .long("raw")
                .help("Displays the raw JSON data")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(message) = matches.value_of("MESSAGE") {
            add_entry(BragEntry::new(message)).expect("Failed to add brag entry");
        }
    } else if let Some(matches) = matches.subcommand_matches("view") {
        let raw = matches.is_present("raw");
        view_entries(raw).expect("Failed to view brag entries");
    }
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
