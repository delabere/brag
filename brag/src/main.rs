use serde::{Serialize, Deserialize};
use std::{fs, env, path::PathBuf};
use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug)]
struct BragEntry {
    content: String,
}

impl BragEntry {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, content] if content != "view" => add_entry(BragEntry::new(content)).expect("Failed to add brag entry"),
        [_, flag] if flag == "view" => view_entries(false).expect("Failed to view brag entries"),
        [_, flag, raw_flag] if flag == "view" && raw_flag == "--raw" => view_entries(true).expect("Failed to view brag entries in raw format"),
        _ => eprintln!("Usage:\n  brag \"I did a thing that I am proud of\"\n  brag view\n  brag view --raw"),
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
            let entries: Vec<BragEntry> = serde_json::from_str(&file)?;
            if entries.is_empty() {
                println!("Your brag list is currently empty. Add some achievements!");
            } else {
                println!("Your Brag List:");
                for (index, entry) in entries.iter().enumerate() {
                    println!("{}. {}", index + 1, entry.content);
                }
            }
        }
    } else {
        println!("Your brag list is currently empty. Start adding some achievements!");
    }
    Ok(())
}
