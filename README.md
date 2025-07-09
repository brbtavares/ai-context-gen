# IA Context Generator

Uma ferramenta de linha de comando em Rust para criar e gerenciar uma janela de contexto que facilita as interações com IAs durante o desenvolvimento de projetos.

## 🚀 Funcionalidades

- **📝 Adicionar Entradas**: Crie novas entradas de contexto com título, conteúdo e tags
- **👀 Visualizar Entradas**: Exiba todas as entradas salvas com formatação colorida
- **🔍 Buscar Entradas**: Pesquise por título, conteúdo ou tags
- **🗑️ Limpar Entradas**: Remove todas as entradas do contexto
- **📊 Resumo do Contexto**: Estatísticas sobre as entradas e tags mais usadas
- **💾 Persistência**: Dados salvos automaticamente em `~/.ia-context-gen/context.json`

## 📋 Pré-requisitos

- Rust 1.70 ou superior
- Cargo

## 🔧 Instalação

1. Clone o repositório:
```bash
git clone <url-do-repositorio>
cd ia-context-gen
```

2. Instale usando o script de instalação:
```bash
./install.sh
```

Ou compile manualmente:
```bash
cargo build --release
```

3. Execute a aplicação:
```bash
# Se instalou com o script
ia-context-gen

# Ou execute diretamente
cargo run
```

## 🛠️ Desenvolvimento

### Comandos úteis (via Makefile)
```bash
make dev        # Executar em modo desenvolvimento
make release    # Compilar para release
make test       # Executar testes
make clean      # Limpar arquivos de build
make fmt        # Formatar código
make lint       # Verificar linting
make install    # Instalar localmente
make demo       # Executar demonstração
```

### Exemplo de contexto inicial
O arquivo `example_context.json` contém exemplos de entradas que você pode usar como referência ou importar para testar a aplicação.

## 📖 Como Usar

### Executar a aplicação
```bash
cargo run
```

### Menu Principal
A aplicação apresenta um menu interativo com as seguintes opções:

1. **📝 Adicionar nova entrada de contexto**
   - Digite o título da entrada
   - Digite o conteúdo (finalize com "FIM" em uma linha separada)
   - Adicione tags separadas por vírgula

2. **👀 Visualizar todas as entradas**
   - Mostra todas as entradas salvas
   - Exibe apenas as primeiras 3 linhas do conteúdo

3. **🔍 Buscar entradas**
   - Pesquise por qualquer termo
   - Busca em títulos, conteúdo e tags

4. **🗑️ Limpar todas as entradas**
   - Remove todas as entradas (requer confirmação)

5. **📊 Gerar resumo do contexto**
   - Mostra estatísticas do contexto
   - Tags mais usadas
   - Entrada mais recente

6. **🚪 Sair**
   - Encerra a aplicação

## 📁 Estrutura de Dados

As entradas são armazenadas com a seguinte estrutura:

```rust
struct ContextEntry {
    timestamp: DateTime<Utc>,
    title: String,
    content: String,
    tags: Vec<String>,
}
```

## 🎯 Casos de Uso

### Para Desenvolvedores
- Salvar trechos de código importantes
- Documentar decisões de arquitetura
- Manter registro de bugs e soluções
- Criar templates de prompts para IAs

### Para Interações com IA
- Manter contexto de conversas anteriores
- Salvar prompts que funcionaram bem
- Documentar resultados de experimentos
- Criar base de conhecimento do projeto

## 🔮 Exemplo de Uso

```
🚀 IA Context Generator - Janela de Contexto
==================================================

Selecione uma opção:
1. 📝 Adicionar nova entrada de contexto
2. 👀 Visualizar todas as entradas
3. 🔍 Buscar entradas
4. 🗑️  Limpar todas as entradas
5. 📊 Gerar resumo do contexto
6. 🚪 Sair

Digite sua escolha (1-6): 1
Título da entrada: Configuração do Rust
📝 Digite o conteúdo da entrada:
Digite 'FIM' em uma linha separada para finalizar:
Para configurar um novo projeto Rust:
1. cargo new projeto
2. cd projeto
3. cargo run
FIM
Tags (separadas por vírgula): rust, setup, cargo
✅ Entrada adicionada com sucesso!
```

## 🛠️ Dependências

- `serde` - Serialização/deserialização JSON
- `chrono` - Manipulação de datas e horários
- `dirs` - Obtenção de diretórios do sistema
- `colored` - Colorização da saída no terminal
- `crossterm` - Manipulação de terminal

## 📝 Desenvolvimento

Este projeto foi desenvolvido especificamente para auxiliar no desenvolvimento de projetos, fornecendo uma maneira rápida e eficiente de manter contexto durante as interações com IAs.

### Compilar para release
```bash
cargo build --release
```

### Executar testes
```bash
cargo test
```

## 🤝 Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Fazer fork do projeto
2. Criar uma branch para sua feature
3. Fazer commit das mudanças
4. Fazer push para a branch
5. Abrir um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT - veja o arquivo LICENSE para detalhes.
