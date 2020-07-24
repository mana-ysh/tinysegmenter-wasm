
test:
	cargo test

build-wasm:
	wasm-pack build

install-www:
	cd www && npm install

start-www:
	cd www && npm run start