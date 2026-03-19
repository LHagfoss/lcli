<img src="img.png">

# lcli

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Latest Release](https://img.shields.io/github/v/release/lhagfoss/lcli?style=for-the-badge&logo=github)](https://github.com/lhagfoss/lcli/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/lhagfoss/lcli/rust.yml?branch=main&style=for-the-badge)](https://github.com/lhagfoss/lcli/actions)

A command-line tool built with Rust made by awesome sauce lagos (me).

## Installation

*coming soon...*
For now git clone the repo then do the next step.

```bash
cargo install --path .
```

This will put the `lcli` binary in your `~/.cargo/bin` folder, which should already be in your `$PATH`.

## Usage

- `lcli ls [path]` - List files and directories in a given path (defaults to current directory).
- `lcli time` - Display the current local time in a pretty format.
- `lcli about` - Displays information about lcli and its creator.
- `lcli calc <COMMAND>` - Simple calculator with subcommands:
  - `add <A> <B>` - Add two numbers.
  - `subtract <A> <B>` - Subtract two numbers.
  - `multiply <A> <B>` - Multiply two numbers.
  - `divide <A> <B>` - Divide two numbers.
- `lcli quote <COMMAND>` - Fetch and manage quotes:
  - `random` - Fetch and print a random quote.
  - `create <QUOTE> <AUTHOR>` - Create and store a custom quote.
- `lcli spam <COMMAND>` - Spam print logs to terminal:
  - `counter <CONTENT> <COUNT>` - Spam print content by a counter.
  - `duration <CONTENT> <SECONDS>` - Spam print content for a duration (in seconds).
- `lcli file <COMMAND>` - File operations:
  - `new <NAME> [PATH]` - Create a new empty file (defaults to current directory).
  - `delete <PATH>` - Delete a file.
  - `move <OLD_PATH> <NEW_PATH>` - Move a file to a new directory.
  - `rename <OLD_PATH> <NEW_NAME>` - Rename a file.
- `lcli --json ...` - Get output in JSON format for scripting (works with most commands).

## Development

Built with:
- `clap` for the CLI interface.
- `owo-colors` for beautiful terminal output.
- `tabled` for pretty-printing data.
