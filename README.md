# lcli - A Modern Command-Line Utility

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Latest Release](https://img.shields.io/github/v/release/lhagfoss/lcli?style=for-the-badge&logo=github)](https://github.com/lhagfoss/lcli/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/lhagfoss/lcli/rust.yml?branch=main&style=for-the-badge)](https://github.com/lhagfoss/lcli/actions)

lcli is a modern, colorful, and user-friendly command-line utility written in Rust, designed to be a powerful and visually appealing replacement for common shell commands.

![lcli Screenshot](https://i.imgur.com/8c3KjYw.png)

*(The image above is a representative screenshot of the output you can expect)*

## ✨ Features

* 🎨 **Colorful, Modern Output:** Uses colors and modern fonts to make terminal output readable and pleasant.
* 📦 **Boxed & Tabled Data:** Displays information in clean, rounded boxes and tables for clarity.
* 💾 **JSON Output:** Provides a `--json` flag on commands for easy integration with scripts and other tools.
* 🚀 **Simple Installation:** Comes with a dedicated installer and uninstaller for easy management on Windows.

## 🚀 Installation (Windows)

1.  **Download the Latest Release**
    Go to the [**Releases Page**](https://github.com/lhagfoss/lcli/releases) and download the latest `.zip` file (e.g., `lcli-v0.1.0-windows-x64.zip`).

2.  **Unzip the Files**
    Extract the contents of the `.zip` file to a folder. You will find `lcli.exe`, `lcli-installer.exe`, and `lcli-uninstaller.exe`.

3.  **Run the Installer**
    Double-click and run **`lcli-installer.exe`**. It will automatically copy `lcli.exe` to a permanent location and add it to your user's `PATH`.

4.  ⚠️ **Open a New Terminal**
    You must **close your current terminal and open a new one** for the `lcli` command to become available.

## 💻 Usage

Once installed, you can run `lcli` from any new terminal window.

```bash
# Get help and see all available commands
lcli --help

# List files and folders in the current directory
lcli list

# Get information about the lcli tool
lcli about

# Get the current time
lcli time

# Use the JSON flag for scriptable output
lcli list --json