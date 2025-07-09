#!/bin/bash

# Installation script for AI Context Generator

echo "🚀 Installing AI Context Generator..."

# Compile in release mode
echo "📦 Compiling application..."
cargo build --release

# Check if compilation was successful
if [ $? -eq 0 ]; then
    echo "✅ Compilation completed successfully!"
    
    # Create bin directory in home if it doesn't exist
    mkdir -p ~/.local/bin
    
    # Copy the executable
    cp target/release/ai-context-gen ~/.local/bin/
    
    # Give execution permission
    chmod +x ~/.local/bin/ai-context-gen
    
    echo "📁 Executable installed in ~/.local/bin/ai-context-gen"
    echo ""
    echo "🎯 To use AI Context Generator, run:"
    echo "   ai-context-gen"
    echo ""
    echo "💡 Make sure ~/.local/bin is in your PATH"
    echo "   To add to PATH, add this line to your ~/.bashrc or ~/.zshrc:"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "🎉 Installation complete!"
else
    echo "❌ Compilation error. Check the logs above."
    exit 1
fi
