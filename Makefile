# AI Context Generator - Makefile
# Additional configuration for development and deployment

# Build and run in development mode
dev:
	cargo run

# Build optimized release version
release:
	cargo build --release

# Build using build script (recommended)
build:
	./build.sh

# Run tests
test:
	cargo test

# Run tests with verbose output
test-verbose:
	cargo test -- --nocapture

# Clean all build artifacts
clean:
	cargo clean

# Check code formatting
fmt:
	cargo fmt

# Apply automatic formatting
fmt-fix:
	cargo fmt --all

# Check linting
lint:
	cargo clippy

# Apply linting fixes
lint-fix:
	cargo clippy --fix --allow-dirty

# Check everything (format, lint, test)
check: fmt lint test

# Install tool locally to system PATH
install: release
	@echo "📦 Installing ai-context-gen to /usr/local/bin/"
	@if [ -f "target/release/ai-context-gen" ]; then \
		sudo cp target/release/ai-context-gen /usr/local/bin/; \
		echo "✅ Installation completed successfully!"; \
		echo "🚀 You can now use 'ai-context-gen' from anywhere"; \
	else \
		echo "❌ Release binary not found. Run 'make release' first."; \
		exit 1; \
	fi

# Uninstall from system
uninstall:
	@echo "🗑️  Removing ai-context-gen from /usr/local/bin/"
	@sudo rm -f /usr/local/bin/ai-context-gen
	@echo "✅ Uninstallation completed"

# Run with example data (current directory)
demo:
	@echo "🚀 Running AI Context Generator demo..."
	@echo "📁 Analyzing current directory..."
	@echo "Press Ctrl+C to exit"
	cargo run

# Run example usage
example-basic:
	@echo "🧪 Running basic usage example..."
	cargo run --example basic_usage

example-advanced:
	@echo "🧪 Running advanced usage example..."
	cargo run --example advanced_usage

examples: example-basic example-advanced

# Run help command
help:
	cargo run -- --help

# Show project information
info:
	@echo "📋 AI Context Generator Project Information"
	@echo "Name: ai-context-gen"
	@echo "Version: $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"
	@echo "Rust Edition: $(shell grep '^edition' Cargo.toml | cut -d'"' -f2)"
	@echo "Build Type: $(if $(wildcard target/release/ai-context-gen),Release,Debug)"

# Crate publishing commands
publish-check:
	@echo "🔍 Checking crate for publishing..."
	cargo check --all-targets
	cargo clippy --all-targets -- -D warnings
	cargo test
	cargo doc --no-deps

publish-dry-run:
	@echo "🧪 Performing dry-run publish..."
	cargo publish --dry-run

publish:
	@echo "📦 Publishing crate to crates.io..."
	@echo "⚠️  Make sure you have incremented the version in Cargo.toml"
	@read -p "Are you sure you want to publish? [y/N] " confirm && [ "$$confirm" = "y" ]
	cargo publish

# Documentation commands
docs:
	@echo "📚 Building documentation..."
	cargo doc --no-deps --open

docs-private:
	@echo "📚 Building documentation (including private items)..."
	cargo doc --no-deps --document-private-items --open

# Version management
version-patch:
	@echo "🔢 Bumping patch version..."
	@current_version=$$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2); \
	IFS='.' read -ra parts <<< "$$current_version"; \
	new_version="$${parts[0]}.$${parts[1]}.$$((parts[2] + 1))"; \
	sed -i "s/^version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	echo "Version updated from $$current_version to $$new_version"

version-minor:
	@echo "🔢 Bumping minor version..."
	@current_version=$$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2); \
	IFS='.' read -ra parts <<< "$$current_version"; \
	new_version="$${parts[0]}.$$((parts[1] + 1)).0"; \
	sed -i "s/^version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	echo "Version updated from $$current_version to $$new_version"

version-major:
	@echo "🔢 Bumping major version..."
	@current_version=$$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2); \
	IFS='.' read -ra parts <<< "$$current_version"; \
	new_version="$$((parts[0] + 1)).0.0"; \
	sed -i "s/^version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	echo "Version updated from $$current_version to $$new_version"

# Crate information
crate-info:
	@echo "📋 AI Context Generator - Crate Information"
	@echo "============================================"
	@echo "Name: $(shell grep '^name' Cargo.toml | head -1 | cut -d'"' -f2)"
	@echo "Version: $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"
	@echo "Edition: $(shell grep '^edition' Cargo.toml | cut -d'"' -f2)"
	@echo "License: $(shell grep '^license' Cargo.toml | cut -d'"' -f2)"
	@echo "Description: $(shell grep '^description' Cargo.toml | cut -d'"' -f2)"
	@echo ""
	@echo "📦 Targets disponíveis:"
	@echo "  - Binary: ai-context-gen"
	@echo "  - Library: ai-context-gen"

# Show available make targets
help-make:
	@echo "🔧 AI Context Generator - Available Make Targets"
	@echo "================================================="
	@echo ""
	@echo "📋 Development:"
	@echo "  dev          - Build and run in development mode"
	@echo "  demo         - Run demo with current directory"
	@echo "  help         - Show application help"
	@echo "  examples     - Run all examples"
	@echo "  example-basic - Run basic usage example"
	@echo "  example-advanced - Run advanced usage example"
	@echo ""
	@echo "🏗️  Building:"
	@echo "  build        - Build using build script (recommended)"
	@echo "  release      - Build optimized release version"
	@echo ""
	@echo "🧪 Testing & Quality:"
	@echo "  test         - Run tests"
	@echo "  test-verbose - Run tests with verbose output"
	@echo "  check        - Run format, lint and test checks"
	@echo ""
	@echo "🎨 Code Quality:"
	@echo "  fmt          - Check code formatting"
	@echo "  fmt-fix      - Apply automatic formatting"
	@echo "  lint         - Check linting"
	@echo "  lint-fix     - Apply linting fixes"
	@echo ""
	@echo "📦 Installation:"
	@echo "  install      - Install to system PATH"
	@echo "  uninstall    - Remove from system"
	@echo ""
	@echo "📚 Documentation:"
	@echo "  docs         - Build and open documentation"
	@echo "  docs-private - Build docs including private items"
	@echo ""
	@echo "🚀 Publishing:"
	@echo "  publish-check - Check crate for publishing"
	@echo "  publish-dry-run - Perform dry-run publish"
	@echo "  publish      - Publish to crates.io"
	@echo ""
	@echo "🔢 Version Management:"
	@echo "  version-patch - Bump patch version (0.0.X)"
	@echo "  version-minor - Bump minor version (0.X.0)"
	@echo "  version-major - Bump major version (X.0.0)"
	@echo ""
	@echo "🧹 Maintenance:"
	@echo "  clean        - Clean all build artifacts"
	@echo "  info         - Show project information"
	@echo "  crate-info   - Show crate-specific information"
	@echo "  help-make    - Show this help"
	@echo ""

.PHONY: dev release build test test-verbose clean fmt fmt-fix lint lint-fix check install uninstall demo help info help-make publish-check publish-dry-run publish docs docs-private version-patch version-minor version-major crate-info example-basic example-advanced examples
