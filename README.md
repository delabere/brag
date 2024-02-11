![Raw GitHub Image](https://raw.githubusercontent.com/delabere/brag/main/brag_logo.png)

Brag is a user-friendly command-line tool designed to help you to maintain a personalized list of accomplishments, milestones, or any brag-worthy moments. It's written in Rust so it's blazingly fast 🚀🚀🚀

When you get to a performance review, you can use your brag list as your source of inspiration. You could even throw it at a LLM like ChatGPT and it can even do the bulk of the writing for you.

Your brag-list is stored as `JSON` so it's easy to store wherever you would like or edit manually if you so wish. The structure is very simple:
```json
[
  {
    "content": "I wrote my first brag... cool 🤩",
    "timestamp": 1707682722
  },
  {
    "content": "Another one within a minute of the first, you can't stop me!",
    "timestamp": 1707682740
  },
  {
    "content": "This isn't where I parked my car?",
    "timestamp": 1707682755
  }
]

```

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
```sh
❯ brag edit
? Select an entry to edit ›
  I wrote my first brag... cool 🤩
  Another one within a minute of the first, you can't stop me!
❯ This isn't where I parked my car?
```

Follow the interactive prompt to select an entry to edit in your default editor.


### Viewing Entries

To view your list of achievements:

```sh
brag view
```

```sh
❯ brag view
1. 2024-02-11 20:18:42: I wrote my first brag... cool 🤩
2. 2024-02-11 20:19:00: Another one within a minute of the first, you can't stop me!
3. 2024-02-11 20:19:15: This isn't where I parked my car?
```

To view your list in raw JSON format so you can filter it down with a tool like `jq`:

```sh
brag view --raw
```

```
❯ brag view --raw
[{"content":"I wrote my first brag... cool 🤩","timestamp":1707682722},{"content":"Another one within a minute of the first, you can't stop me!","timestamp":1707682740},{"content":"This isn't where I parked my car?","timestamp":1707682755}]
```

### Removing an Entry

To remove an entry from your list:

```sh
brag remove
```

```sh
❯ brag remove
? Select an entry to remove ›
  I wrote my first brag... cool 🤩
❯ Another one within a minute of the first, you can't stop me!
  This isn't where I parked my car?
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

