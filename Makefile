# Additional configuration for development

# Compile for development only
dev:
	cargo run

# Compile for release
release:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build files
clean:
	cargo clean

# Check code formatting
fmt:
	cargo fmt

# Check linting
lint:
	cargo clippy

# Install tool locally
install:
	./install.sh

# Run with example data
demo:
	@echo "Example application usage will start..."
	@echo "Press Ctrl+C to exit"
	cargo run

.PHONY: dev release test clean fmt lint install demo
