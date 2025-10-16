# Cangjie Extension for Zed

This extension provides language support for the Cangjie programming language in the Zed editor.

## Features
- Syntax highlighting
- Code completion
- Diagnostics
- Go to definition
- Hover information

## Requirements
- Zed editor version 0.100.0 or higher
- Rust toolchain
- Cangjie language server (cangjie-lsp)

## Installation
1. Clone this repository into your Zed extensions directory
2. Run `cargo build --release` to build the extension
3. The compiled extension will be in `target/release/libcangjie_zed_extension.dylib`
4. Enable the extension in Zed's settings

## Configuration


Add the following to your Zed settings:
```json
{
  "file_types": { "Cangjie": ["cj"] },
  "languages": {
    "Cangjie": {
      "language_servers": ["cangjie-lsp"]
    }
  }
}
```

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue.

## Building the Extension
```bash
cd zed-extensions/cangjie
cargo build --release
```

