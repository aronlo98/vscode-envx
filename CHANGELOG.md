# Changelog

All notable changes to `Envx for VSCode` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and the project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-06-28

### Added

- Initial release of `Envx for VSCode`.
- **Syntax Highlighting:** Support for variables, strings, template blocks (`${{ ... }}`), keywords, and built-in functions.
- **Auto-closing Brackets:** Seamless typing for blocks, quotes, and parentheses.
- **Language Server Protocol (LSP) Integration:**
  - Live syntax validation and error diagnostics powered by the native `envx` engine.
  - "Go to Definition" support for following inter-variable and imported variable references.
- Self-contained `envx-lsp` Rust binary built natively into the extension.
