# Changelog

All notable changes to `Envx for VSCode` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and the project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4] - 2026-06-29

### Changed

- Updated underlying `envx` engine to v0.1.4 to support the latest features (including tags filtering).
- Added explicit mention and link to the main `envx` project repository in the README.

## [0.1.3] - 2026-06-28

### Added

- **Syntax Highlighting:** Support for section headers (`[SectionName]`).
- Updated underlying `envx` engine to support parsing section headers.

## [0.1.2] - 2026-06-28

### Changed

- Updated underlying `envx` engine to v0.1.2.
- The Language Server now correctly supports parsing and validating bare (unquoted) values without generating `UnexpectedToken` errors.
- Improved validation for the expanded list of 26 built-in functions.

## [0.1.0] - 2026-06-28

### Added

- Initial release of `Envx for VSCode`.
- **Syntax Highlighting:** Support for variables, strings, template blocks (`${{ ... }}`), keywords, and built-in functions.
- **Auto-closing Brackets:** Seamless typing for blocks, quotes, and parentheses.
- **Language Server Protocol (LSP) Integration:**
  - Live syntax validation and error diagnostics powered by the native `envx` engine.
  - "Go to Definition" support for following inter-variable and imported variable references.
- Self-contained `envx-lsp` Rust binary built natively into the extension.
