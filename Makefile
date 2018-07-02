SHELL := /bin/bash
PATH := $(PATH):./node_modules/.bin

.PHONY: all

all:
	cargo build --target wasm32-unknown-unknown --release

	@mkdir -p ./wasm

	wasm-bindgen target/wasm32-unknown-unknown/release/rust_wasm_snake.wasm \
  	--out-dir ./wasm \
		--browser
	
	webpack