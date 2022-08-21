build:
	wasm-pack build --target web

serve:
	npm run serve

setup:
	@echo "===> Expecting the following dependencies to be installed:"
	@echo "     - Rust (https://www.rust-lang.org/tools/install)"
	@echo "     - npm  (https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)"
	cargo install wasm-pack
	npm install