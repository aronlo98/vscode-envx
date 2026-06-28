# envx for VSCode

A Visual Studio Code extension that provides first-class support for `.envx` files.

`.envx` is a modern, safe replacement for `dotenv` that supports dynamic variables, pipe-based text manipulation, conditional logic, inter-variable dependencies, and file imports.

---

## Features

- **Syntax Highlighting:** Rich syntax coloring for variables, strings, template blocks (`${{ ... }}`), keywords (`if`, `then`, `else`, `@import`), and built-in functions.
- **Auto-closing Brackets:** Seamless typing experience with auto-closing for `${{ }}` blocks, quotes, and parentheses.
- **Language Server Protocol (LSP) Integration:** 
  - **Live Validation:** Get instant feedback on syntax errors, unclosed strings, or invalid template blocks as you type, powered by the native `envx` parser.
  - **Error Diagnostics:** Visually identifies issues like circular dependencies, duplicate variables, or undefined functions directly in the editor.

---

## Installation

You can install the extension using the packaged `.vsix` file either manually through the VSCode interface or via the command line.

### Option 1: Command Line
```sh
code --install-extension envx-0.1.0.vsix
```

### Option 2: Manual (VSCode UI)
1. Open the **Extensions** view in VSCode (`Cmd + Shift + X` or `Ctrl + Shift + X`).
2. Click on the `...` menu in the top right corner of the Extensions view.
3. Select **Install from VSIX...**
4. Locate and select the `envx-0.1.0.vsix` file.

---

## Requirements & Configuration

The syntax highlighting works out of the box. However, to use the advanced **Language Server** features (live validation, error diagnostics, and Go to Definition), you must build and configure the `envx-lsp` server included in this repository.

1. Build the LSP server:
```sh
cd envx-lsp
cargo build --release
```

2. Configure VSCode to point to the built executable. Set this in your `settings.json`:
```json
{
    "envx.executablePath": "/absolute/path/to/vscode-envx/envx-lsp/target/release/envx-lsp"
}
```

---

## Contributing

This extension is part of the broader `envx` project ecosystem. Contributions, bug reports, and feature requests are welcome!

1. Clone the repository.
2. Run `npm install` inside the `vscode-envx` directory.
3. Open the folder in VSCode and press `F5` to launch a new Extension Development Host window.

---

## License

MIT
