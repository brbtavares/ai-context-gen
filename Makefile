# Configuração adicional para desenvolvimento

# Compilar apenas para desenvolvimento
dev:
	cargo run

# Compilar para release
release:
	cargo build --release

# Executar testes
test:
	cargo test

# Limpar arquivos de build
clean:
	cargo clean

# Verificar formatação do código
fmt:
	cargo fmt

# Verificar linting
lint:
	cargo clippy

# Instalar ferramenta localmente
install:
	./install.sh

# Executar com dados de exemplo
demo:
	@echo "Exemplo de uso da aplicação será iniciado..."
	@echo "Pressione Ctrl+C para sair"
	cargo run

.PHONY: dev release test clean fmt lint install demo
