# MiniGrep

A simple, fast, and lightweight CLI tool for searching text in files — written in Rust.

## Features

- Search for a string inside a file
- Case-sensitive and case-insensitive modes
- Cross-platform support (Windows, Linux, macOS)
- Simple usage and installation

## Installation (Windows)

You can install `minigrep` with the provided installer script:

```sh
curl -O https://fxnbins.faizeenhoque.xyz/minigrep/install_minigrep.bat
install_minigrep.bat
```
Or run manually:
```sh
powershell -Command "Invoke-WebRequest -Uri 'https://fxnbins.faizeenhoque.xyz/minigrep/v1.0.0/minigrep.exe' -OutFile '$env:USERPROFILE\fxnbin\minigrep.exe'"
```
Make sure to add C:\Users\YourName\fxnbins to your system PATH if it's not already.

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

