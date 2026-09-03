# Rust "Grep" Command

![Rust](https://img.shields.io/badge/rust-stable-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

This is my attempt to recreate the Linux's command "grep" in Rust.

## Installation

1. Clone the repository
   ```bash
   git clone https://github.com/DafexDV/rust-grep.git
   cd rust-grep
   cargo install --path .
   ```
2. Compile the project with cargo
   ```bash
   cargo build
   ```
3. Run the cli
   ```
   cargo build release
   # In Linux/MacOS
   ./target/release/grep
   # In Windows
   .\target/release/grep
   ```

## Use

Syntax:
```bash
grep [OPTIONS] <PATTERN> <FILE>
```

Example:
```bash
grep hello test.txt
```

Arguments:

| # | Name        | Description                                    |
|---|-------------|------------------------------------------------|
| 1 | `<PATTERN>` | Search ignoring uppercase and lowercase        |
| 2 | `<FILE>`    | The file in which the pattern will be searched |

Options:

| Name              | Description                                                                 |
|-------------------|-----------------------------------------------------------------------------|
| `-i`              | Search ignoring uppercase and lowercase                                     |
| `-v`              | Show lines that do not match                                                |
| `-c`              | Instead of showing the lines, it just tells you how many matches were found |
| `-w`              | Only matches the exact word, not if it is part of a larger                  |
| `-h`, `--help`    | Print help                                                                  |
| `-V`, `--version` | Print version                                                               |

## Special thanks

- [clap](https://docs.rs/clap/latest/clap/): Command line argument parser
