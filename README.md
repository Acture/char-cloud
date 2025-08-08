# Char Cloud

[![Crates.io](https://img.shields.io/crates/v/char-cloud)](https://crates.io/crates/char-cloud)
[![Release Build](https://github.com/Acture/char-cloud/actions/workflows/release.yml/badge.svg)](https://github.com/Acture/char-cloud/actions/workflows/release.yml)
[![License](https://img.shields.io/crates/l/char-cloud)](LICENSE)

Char Cloud is a command-line tool written in Rust that generates word-cloud images in custom shapes. It lets you use a piece of text as the outline and fill that shape with the words you provide, producing an SVG file as output.

## Features

- Custom shape defined by text
- Automatic font-size fitting
- Configurable output image size
- Custom font support
- Multiple color support
- SVG output
- Built-in Chinese fonts

## Installation
```bash
cargo install char-cloud
```

## Usage

Basic example:
```bash
char-cloud --text "Target Text" --words "word1,word2,word3" --output output.svg
```

For more options, run:
```bash
char-cloud --help
```

## License
This project is licensed under the Affero GPL v3. Please read the [LICENSE](LICENSE) file before use.
