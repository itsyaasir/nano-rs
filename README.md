
---

# Nano-rs

Nano-rs is a simple text editor written in Rust, inspired by the simplicity and ease-of-use of the Nano editor. It supports basic text editing features such as insertion, deletion, cursor navigation, and more. It also includes syntax highlighting thanks to the `syntect` crate.

## Installation

First, ensure you have Rust installed on your system. If not, download it from the official Rust website.

Then, clone the project and build it using `cargo`:

```bash
git clone https://github.com/itsyaasir/nano-rs.git
cd nano-rs
cargo build --release
```

This will create an executable in the `target/release` directory.

## Usage

To start the editor, run:

```bash
./target/release/nano-rs
```

To open a specific file, pass the file name as an argument:

```bash
./target/release/nano-rs myfile.txt
```

## Features

- [ ] Basic text editing (insertion, deletion, etc.)
- [ ] Cursor navigation
- [ ] Syntax highlighting for multiple languages
- [ ] Reading and writing to files

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an issue.

---
