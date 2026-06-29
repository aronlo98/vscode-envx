.PHONY: build test install package clean

# Compiles the Rust LSP server and the TypeScript client
build:
	@echo "Building envx-lsp (Rust)..."
	cd envx-lsp && cargo build --release
	@mkdir -p bin
	@cp envx-lsp/target/release/envx-lsp bin/ || cp envx-lsp/target/release/envx-lsp.exe bin/ || true
	@echo "Installing NPM dependencies and compiling TypeScript..."
	npm install
	npm run compile

# Runs tests for the LSP server (and client if available)
test:
	@echo "Running envx-lsp tests (Rust)..."
	cd envx-lsp && cargo test
	# @echo "Running client tests (TypeScript)..."
	# npm run test

# Packages the extension into a .vsix file
package: build
	@echo "Packaging extension..."
	npx @vscode/vsce package

# Builds, packages, and installs the extension into your local VSCode
install: package
	@echo "Installing extension into VSCode..."
	code --install-extension $$(ls -t envx-*.vsix | head -1) --force

# Cleans up generated binaries and installed modules
clean:
	@echo "Cleaning generated files..."
	rm -rf out/
	rm -rf bin/
	rm -rf node_modules/
	rm -f *.vsix
	cd envx-lsp && cargo clean
