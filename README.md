# MiniGrep

A simple, fast, and lightweight CLI tool for searching text in files — written in Rust.

## Features

- Search for a string inside a file
- Case-sensitive and case-insensitive modes
- Cross-platform support (Windows, Linux, macOS)
- Simple usage and installation

## Installation (Windows)

## Usage
```sh
minigrep <query> <filename> [--case-insensitive]
```

## Examples
```sh
minigrep "hello" notes.txt
minigrep "trust" poem.txt --case-insensitive
```

## Build from Source
```sh
git clone https://github.com/yourusername/minigrep.git
cd minigrep
cargo build --release
```

The executable will be in ```target/release/minigrep.exe```.

## License

This project is licensed under the **Creative Commons Attribution-NonCommercial 4.0 International License (CC BY-NC 4.0)**.  
You are free to use, modify, and share the software — but **not for commercial purposes**.

See [LICENSE](./LICENSE) for full terms.

