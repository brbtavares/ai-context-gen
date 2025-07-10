#!/bin/bash

# Installation script for ai-context-gen
# Installs the executable on the system

set -e

echo "🚀 AI Context Generator - Installer"
echo "===================================="

# Check if executable exists
if [ ! -f "target/release/ai-context-gen" ]; then
    echo "❌ Executable not found in target/release/"
    echo "📦 Running release build first..."
    cargo build --release
    
    if [ $? -ne 0 ]; then
        echo "❌ Build failed. Aborting installation."
        exit 1
    fi
fi

# Check permissions for system installation
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    echo "🔐 Administrator permissions required to install in $INSTALL_DIR"
    echo "📁 Installing ai-context-gen..."
    sudo cp target/release/ai-context-gen "$INSTALL_DIR/"
else
    echo "📁 Installing ai-context-gen..."
    cp target/release/ai-context-gen "$INSTALL_DIR/"
fi

# Check if installation was successful
if [ -f "$INSTALL_DIR/ai-context-gen" ]; then
    echo "✅ Installation completed successfully!"
    echo ""
    echo "🎉 ai-context-gen is now available globally!"
    echo "🔧 Test with: ai-context-gen --help"
    echo "📚 Usage: ai-context-gen --path /path/to/project"
    echo ""
    echo "🗑️  To uninstall: make uninstall"
else
    echo "❌ Installation failed."
    exit 1
fi
