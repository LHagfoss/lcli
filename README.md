# lcli

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Latest Release](https://img.shields.io/github/v/release/lhagfoss/lcli?style=for-the-badge&logo=github)](https://github.com/lhagfoss/lcli/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/lhagfoss/lcli/rust.yml?branch=main&style=for-the-badge)](https://github.com/lhagfoss/lcli/actions)

A command-line tool built with Rust made by awesome sauce lagos (me).

<img src="img.png">

## Installation

*coming soon...*
For now git clone the repo then do the next step.

```bash
cargo install --path .
```

This will put the `lcli` binary in your `~/.cargo/bin` folder, which should already be in your `$PATH`.

## Usage

- `lcli list [path]` - List files and directories.
- `lcli time` - Display the current local time.
- `lcli about` - Learn more about the tool.
- `lcli --json ...` - Get output in JSON format for scripting.

## Development

Built with:
- `clap` for the CLI interface.
- `owo-colors` for beautiful terminal output.
- `tabled` for pretty-printing data.
