#!/bin/bash

# Script de instalação para IA Context Generator

echo "🚀 Instalando IA Context Generator..."

# Compilar em modo release
echo "📦 Compilando aplicação..."
cargo build --release

# Verificar se a compilação foi bem-sucedida
if [ $? -eq 0 ]; then
    echo "✅ Compilação concluída com sucesso!"
    
    # Criar diretório bin no home se não existir
    mkdir -p ~/.local/bin
    
    # Copiar o executável
    cp target/release/ai-context-gen ~/.local/bin/
    
    # Dar permissão de execução
    chmod +x ~/.local/bin/ai-context-gen
    
    echo "📁 Executável instalado em ~/.local/bin/ai-context-gen"
    echo ""
    echo "🎯 Para usar o IA Context Generator, execute:"
    echo "   ai-context-gen"
    echo ""
    echo "💡 Certifique-se de que ~/.local/bin está no seu PATH"
    echo "   Para adicionar ao PATH, adicione esta linha ao seu ~/.bashrc ou ~/.zshrc:"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "🎉 Instalação concluída!"
else
    echo "❌ Erro na compilação. Verifique os logs acima."
    exit 1
fi
