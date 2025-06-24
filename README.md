# 🔍 Search Text CLI

A simple command-line tool written in Rust to search for a pattern inside a text file — kind of like a minimal `grep`.

This is a learning project built to explore CLI development in Rust using popular crates like `clap`, `anyhow`, and `indicatif`.

---

## ⚙️ Features

- Pattern matching on each line of a file
- Error handling with clear messages
- Progress bar while scanning lines
- Written entirely in safe, modern Rust

---

## 📦 Installation

### Requirements

- Rust and Cargo installed: https://www.rust-lang.org/tools/install

### Install from GitHub

```bash
cargo install --git https://github.com/Dvalfonso/Search-text-cli
```

This will install the binary as search-text-cli into your ~/.cargo/bin.

Make sure that directory is in your $PATH to run it globally.

🧪 Usage

```bash
search-text-cli <PATTERN> <FILE>
```
