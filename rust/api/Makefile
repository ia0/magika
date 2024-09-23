# Usage: make ARG=<file>
# Example: make ARG=Makefile
#        - make ARG=src/lib.rs

.PHONY: run
run: example
	LD_LIBRARY_PATH=../target/release ./$< $(ARG)

example: example.c ../target/release/libmagika_api.so
	gcc $< -o $@ -lmagika_api -L../target/release

../target/release/libmagika_api.so:
	cargo build --release

-include ../target/release/libmagika_api.d
