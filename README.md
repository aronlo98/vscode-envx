# envx for VSCode

A Visual Studio Code extension that provides first-class support for `.envx` files.

`.envx` is a modern, safe replacement for `dotenv` that supports dynamic variables, pipe-based text manipulation, conditional logic, inter-variable dependencies, and file imports.

---

## Features

- **Syntax Highlighting:** Rich syntax coloring for variables, strings, bare values, template blocks (`${{ ... }}`), section headers (`[SectionName]`), keywords (`if`, `then`, `else`, `@import`), and built-in functions.
- **Auto-closing Brackets:** Seamless typing experience with auto-closing for `${{ }}` blocks, quotes, and parentheses.
- **Language Server Protocol (LSP) Integration:** 
  - **Live Validation:** Get instant feedback on syntax errors, unclosed strings, or invalid template blocks as you type, powered by the native `envx` parser.
  - **Error Diagnostics:** Visually identifies issues like circular dependencies, duplicate variables, or undefined functions directly in the editor.

---

## Installation

You can easily build, package, and install the extension straight from the source using the provided `Makefile`. Make sure you have `Node.js`, `npm`, and `Rust` installed on your machine.

To compile the language server, package the extension, and install it into VSCode automatically, simply run:

```sh
make install
```

### Other Commands

- `make build`: Compiles the Rust Language Server and the TypeScript extension.
- `make package`: Builds everything and generates a `.vsix` installer.
- `make test`: Runs the test suite for the language server.
- `make clean`: Removes all compiled artifacts and dependencies.

---

## Configuration

If you installed the extension via the Marketplace or built it using `make build`, the extension will automatically locate and use the bundled `envx-lsp` binary. **No manual configuration is needed!**

However, if you wish to use a custom version of the language server, you can override the path in your VSCode `settings.json`:

```json
{
    "envx.executablePath": "/absolute/path/to/your/custom/envx-lsp"
}
```

---

## Contributing

This extension is part of the broader [envx](https://github.com/aronlo98/envx) project ecosystem. You can find the core language engine, CLI, and documentation in the main repository:
**[https://github.com/aronlo98/envx](https://github.com/aronlo98/envx)**

Contributions, bug reports, and feature requests are welcome!

1. Clone the repository.
2. Run `make build` to install dependencies and compile the server.
3. Open the folder in VSCode and press `F5` to launch a new Extension Development Host window.

---

## License

MIT
