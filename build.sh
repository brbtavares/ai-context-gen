#!/bin/bash

# Build script for ai-context-gen project
# Generates an optimized executable for Linux

echo "🚀 Starting ai-context-gen build..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Install Rust at https://rustup.rs/"
    exit 1
fi

echo "🔧 Checking Rust version..."
rustc --version

echo "📦 Downloading dependencies..."
cargo fetch

echo "🏗️  Compiling in release mode..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build completed successfully!"
    echo "📁 Executable available at: target/release/ai-context-gen"
    echo ""
    echo "🔧 To use:"
    echo "  ./target/release/ai-context-gen --help"
    echo "  ./target/release/ai-context-gen --path /path/to/project"
    echo ""
    echo "📦 To install on system: make install"
else
    echo "❌ Build failed"
    exit 1
fi
