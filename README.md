![Raw GitHub Image](https://raw.githubusercontent.com/delabere/brag/main/brag_logo.png)

Brag is a user-friendly command-line tool designed to help you to maintain a personalized list of accomplishments, milestones, or any brag-worthy moments. It's written in Rust so it's blazingly fast 🚀🚀🚀

When you get to a performance review, you can use your brag list to hep you write it up. You could even throw it at a LLM like ChatGPT and it can even do the bulk of the writing for you.

Happy bragging!

## Features

- **Add Entries**: Quickly add new achievements to your brag list with a simple command.
- **View Entries**: Display your achievements in a nicely formatted list or raw JSON format for easy parsing or integration with other tools.
- **Edit Entries**: Interactively select and edit entries.
- **Remove Entries**: Interactively select and remove entries you no longer want to keep.
- **Dotfile Storage**: Your brag list is stored as a dotfile in your home directory, each brag is also timestamped.
- **Blazingly Fast**: Written in Rust, Brag offers exceptional performance and efficiency.

## Usage

### Adding an Entry

To add a new achievement to your brag list:

```sh
brag add  # opens in your default editor
# or to add more quickly
brag add "I just wrote my first rust-based project 🦀!!"
```

### Editing an Entry

To remove an entry from your list:

```sh
brag edit
```

Follow the interactive prompt to select an entry to edit in your default editor.


### Viewing Entries

To view your list of achievements:

```sh
brag view
```

To view your list in raw JSON format so you can filter it down with a tool like `jq`:

```sh
brag view --raw
```

### Removing an Entry

To remove an entry from your list:

```sh
brag remove
```

Follow the interactive prompt to select and remove an entry.

## Installation

### Source

Currently, you can compile Brag from source. Make sure you have Rust installed on your system, and then clone this repository and build the project:

```sh
git clone https://github.com/delabere/brag.git
cd brag
cargo build --release
```

The executable will be available in `target/release/`.

### Nix

This project is a nix flake, so you can add it to your own flakes too

