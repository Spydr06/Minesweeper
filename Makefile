build:
	wasm-pack build --target web

run: build
	python ./tools/server.py

clean:
	rm -rf ./pkg
	rm -rf ./target
	rm -rf ./Cargo.lock
