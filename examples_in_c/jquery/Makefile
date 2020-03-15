build:
	mkdir -p tmp/src
	clang src/main.c -I./ -S -emit-llvm --target=wasm32 -o tmp/src/main.ll
	llvm-link -o tmp/main.bc tmp/src/*\.ll
	opt -O3 tmp/main.bc -o tmp/main.bc
	llc -mtriple=wasm32-unknown-unknown -O3 -filetype=obj tmp/main.bc -o tmp/main.o
	wasm-ld tmp/main.o -o main.wasm --no-entry --export-dynamic -allow-undefined
	rm -rf tmp
serve:
	python3 -m http.server 8080
lint:
	clang-lint -i src/main.c
deps:
	sudo dnf install clang llvm lld
	