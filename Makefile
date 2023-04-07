.PHONY: build

install:
	cargo install hyperfine

build: install
	cargo build -r
	clear

test-all: build
	./test-all.sh 69 50000