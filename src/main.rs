use anyhow::{anyhow, bail, ensure, Context, Result};
use chrono::{serde::ts_seconds, DateTime, Utc};
use clap::{Parser, Subcommand};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::{Seek, SeekFrom, Write};
use std::process::Command;
use std::{env, fs, io::Read, path::PathBuf};
use tempfile::NamedTempFile;

#[derive(Debug, Parser)]
struct Add {
    /// The brag message to add
    #[arg(short, long)]
    text: Option<String>,
}

#[derive(Debug, Parser)]
struct View {
    /// Display raw JSON data
    #[arg(short, long)]
    raw: bool,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Adds a new brag to your list or opens an editor to write the brag
    Add(Add),
    /// Views your brag list
    View(View),
    Edit,
    Remove,
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

fn main() -> Result<()> {
    let command = Cli::parse();
    let mut brag = Brag::read()?;
    match command.action {
        Action::Add(args) => brag.add(&args),
        Action::View(args) => brag.view(&args),
        Action::Edit => brag.edit(),
        Action::Remove => brag.remove(),
    }?;
    brag.write()
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    content: String,
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
}

impl Entry {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            timestamp: Utc::now(),
        }
    }

    fn edit(&mut self) -> Result<()> {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

        let mut tmp = NamedTempFile::new()?;
        tmp.write_all(self.content.as_bytes())?;
        Command::new(editor).arg(tmp.path()).status()?;

        let mut res = String::new();
        tmp.seek(SeekFrom::Start(0))?;
        tmp.read_to_string(&mut res)?;

        self.content = res.trim().to_string();
        if self.content.is_empty() {
            bail!("No content provided!");
        }
        Ok(())
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.content
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Brag {
    entries: Vec<Entry>,
}

impl Brag {
    fn path() -> PathBuf {
        home_dir()
            .expect("Could not find home directory")
            .join(".brag_list.json")
    }

    fn read() -> Result<Self> {
        match std::fs::File::open(Brag::path()) {
            Ok(reader) => {
                let brag =
                    serde_json::from_reader(reader).context("Failed to parse .brag_list.json")?;
                Ok(brag)
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::NotFound {
                    bail!(e)
                }
                Ok(Brag { entries: vec![] })
            }
        }
    }

    fn write(&self) -> Result<()> {
        let writer = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(Brag::path())?;
        serde_json::to_writer_pretty(writer, self).context("Failed to write .brag_list.json")
    }

    fn select_entry(&self, prompt: &str) -> Result<usize> {
        ensure!(
            !self.entries.is_empty(),
            "Your brag list is currently empty."
        );

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(0)
            .items(&self.entries)
            .interact_on_opt(&Term::stderr())?;

        selection.ok_or(anyhow!("No entry selected"))
    }

    fn add(&mut self, args: &Add) -> Result<()> {
        let entry = match &args.text {
            Some(message) => Entry::new(message),
            None => {
                let mut entry = Entry::new("");
                entry.edit()?;
                entry
            }
        };
        self.entries.push(entry);
        Ok(())
    }

    fn view(&self, args: &View) -> Result<()> {
        if args.raw {
            println!("{}", fs::read_to_string(Brag::path())?);
            return Ok(());
        }

        for entry in self.entries.iter() {
            println!("{}", entry);
        }
        Ok(())
    }

    fn edit(&mut self) -> Result<()> {
        let index = self.select_entry("Select entry to edit")?;
        self.entries[index].edit()?;
        println!("Entry updated.");
        Ok(())
    }

    fn remove(&mut self) -> Result<()> {
        let index = self.select_entry("Select entry to remove")?;
        self.entries.remove(index);
        println!("Entry removed.");
        Ok(())
    }
}
