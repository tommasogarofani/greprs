# Nome del file Makefile per il progetto minigrep

.PHONY: all check fmt lint test build release doc clean run help

# Comando di default eseguito lanciando semplicemente `make`
all: fmt lint check test

## check: Controlla che il codice compili rapidamente senza generare il binario
check:
	cargo check

## fmt: Formatta automaticamente tutto il codice sorgente
fmt:
	cargo fmt

## fmt-check: Verifica se il codice è formattato correttamente senza modificarlo
fmt-check:
	cargo fmt -- --check

## lint: Esegue Clippy per trovare codice non idiomatico o potenziali errori
lint:
	cargo clippy -- -D warnings

## test: Esegue la suite dei test
test:
	cargo test

## build: Compila il progetto in modalità Debug
build:
	cargo build

## release: Compila il binario finale ottimizzato per la produzione
release:
	cargo build --release

## doc: Genera e apre la documentazione del progetto
doc:
	cargo doc --open

## clean: Rimuove i file generati dalla compilazione (cartella target/)
clean:
	cargo clean

## run: Esegue il programma con argomenti di esempio (es. make run ARGS="query file.txt")
run:
	cargo run -- $(ARGS)

## help: Mostra questo messaggio di aiuto
help:
	@echo "Comandi disponibili:"
	@sed -n 's/^##//p' $(MAKEFILE_LIST)