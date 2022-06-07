build:
	wasm-pack build --target web

run: build
	python ./tools/server.py

package: build
	mkdir -p package/src
	cp -rv pkg stylesheet.css index.html tools/server.py package/
	cp -v src/main.js package/src/main.js
	tar czfv package.tar.gz ./package
	zip package.zip package

clean:
	rm -rf ./pkg
	rm -rf ./target
	rm -rf ./Cargo.lock
	rm -rf ./package
	rm ./package.zip ./package.tar.gz
