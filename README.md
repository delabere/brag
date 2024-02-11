# Brag CLI Application

## Overview

The Brag CLI application is a fast, efficient, and user-friendly tool designed to help users maintain a personalized list of accomplishments, milestones, or any brag-worthy moments. Blazingly fast, thanks to Rust's performance and safety features, Brag lets users swiftly add, view, and manage their achievements with ease.

Whether you're preparing for a performance review, tracking personal milestones, or simply want to keep a record of your proud moments, Brag provides a streamlined command-line interface to store your accomplishments with timestamps for historical ordering.

## Features

- **Add Entries**: Quickly add new achievements to your brag list with a simple command.
- **View Entries**: Display your achievements in a nicely formatted list or raw JSON format for easy parsing or integration with other tools.
- **Remove Entries**: Interactively select and remove entries you no longer want to keep.
- **Blazingly Fast**: Written in Rust, Brag offers exceptional performance and efficiency.

## Usage

### Adding an Entry

To add a new achievement to your brag list:

```sh
brag add "Completed my first marathon"
```

### Viewing Entries

To view your list of achievements:

```sh
brag view
```

To view your list in raw JSON format:

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

Currently, you can compile Brag from source. Make sure you have Rust installed on your system, and then clone this repository and build the project:

```sh
git clone https://github.com/delabere/brag.git
cd brag
cargo build --release
```

The executable will be available in `target/release/`.

We are working on making Brag available through cargo install and other package managers soon.

## Contributing

Contributions to Brag are welcome! Whether it's reporting bugs, suggesting new features, or contributing to the code, we appreciate your help in making Brag better.

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

Brag is released under the MIT License. See [LICENSE](LICENSE) for more information.
