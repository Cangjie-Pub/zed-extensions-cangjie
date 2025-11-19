# Cangjie Extension for Zed

[中文说明 (Chinese Documentation)](README_zh.md)

This extension adds comprehensive language support for the Cangjie programming language within the Zed editor.

## Features

- Syntax Highlighting
- Code Completion
- Diagnostics (Error and Warning reporting)
- Go to Definition
- Find All References
- Hover Information
- Document Symbols Outline

## Requirements

- **Zed Editor**: Version 0.135.0 or higher.
- **Rust Toolchain**: Required to build the extension from source.
- **Cangjie Language Server (LSP)**: A pre-built binary of the Cangjie SDK containing the `LSPServer`.

## Installation

### Option 1: Install via Zed Marketplace (Recommended - Future)

*(Note: This extension is pending submission/approval. If available, installing from the marketplace will be the simplest method.)*

### Option 2: Manual Installation (Load Dev Extension)

1.  **Clone the Repository**:
    Clone this repository to a local directory on your machine.
    ```bash
    git clone https://github.com/Cangjie-Pub/zed-extensions-cangjie.git # Or wherever you forked it
    cd zed-extensions-cangjie/cangjie # Adjust path if needed
    ```
2.  **Build the Extension**:
    Use Cargo to compile the extension in release mode.
    ```bash
    cargo build --release
    ```
3.  **Load the Extension in Zed**:
    -   Open Zed.
    -   Go to `Zed > Settings` (or `Ctrl/Cmd + ,`).
    -   Navigate to the `Extensions` tab.
    -   Click the **`Install Dev Extension`** button.
    -   In the file dialog, navigate to and select the **root folder** of this extension (the folder containing `Cargo.toml`). For example, select the `zed-extensions/cangjie` directory.
    -   Zed will automatically detect and load the extension from that directory.
    -   If prompted about trusting the extension, review the source and confirm trust.
4.  **(Optional) Enable the Extension**:
    The extension should be loaded and typically enabled by default. You can verify its status in the `Extensions` settings list. Toggle it on if necessary.

*(Note: You do not need to manually copy binaries or create folders in the extension directory anymore. Zed handles loading directly from your source folder.)*

## Setup Cangjie Language Server (LSP)

This extension requires the Cangjie Language Server (`LSPServer`) to function. You need to install the Cangjie SDK manually and tell the extension where to find the `LSPServer` executable.

### Steps:

1.  **Download the Cangjie SDK**:
    Visit the official Cangjie website or its GitHub releases page to download the appropriate SDK package for your operating system (e.g., `cangjie-sdk-mac-aarch64-1.0.4.tar.gz`).

2.  **Extract the SDK**:
    Extract the downloaded archive to a location of your choice. For example:
    ```bash
    tar -xzf cangjie-sdk-mac-aarch64-1.0.4.tar.gz -C ~/
    ```
    This might create a directory like `~/cangjie-sdk/cangjie/`.

3.  **Configure Zed to Locate the LSP Server**:
    You need to provide the absolute path to the `LSPServer` executable in your Zed settings.
    -   Open Zed Settings (`Cmd/Ctrl + ,`).
    -   Switch to the `JSON` view.
    -   Add or modify the configuration block for `cangjie-lsp`. Replace `/absolute/path/to/...` with the actual path on your system.

    ```json
     {
       "lsp": {
         "cangjie-lsp": {
           "settings": {
             "serverPath": "/Users/yourusername/cangjie-sdk/cangjie/tools/bin/LSPServer" // <-- Update this path
           }
         }
       },
       "file_types": {
         "Cangjie": ["cj"]
       },
       "languages": {
         "Cangjie": {
           "language_servers": ["cangjie-lsp"]
         }
       }
     }
    ```

    *   **`lsp.cangjie-lsp.settings.serverPath`**: Set this to the full, absolute path to the `LSPServer` executable within your extracted SDK.

4.  **Restart Zed**:
    Save your settings and completely restart the Zed editor to ensure all changes take effect.

Once configured correctly, opening a `.cj` file should activate the Cangjie language features provided by this extension.

## Contributing

Contributions are very welcome! Feel free to submit issues or pull requests to improve this extension.

## Building the Extension (For Developers)

Navigate to the extension's directory and run the standard Cargo build command:

```bash
cd /path/to/zed-extensions/cangjie # Or wherever you cloned it
cargo build --release
