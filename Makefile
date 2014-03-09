RUSTC ?= rustc
RUSTDOC ?= rustdoc
RUSTPKG ?= rustpkg
RUSTFLAGS ?= -O

all:
	$(RUSTC) $(RUSTFLAGS) -L ${RUST_LIB_PATH}/rust-http/build main.rs

test:
	$(RUSTC) --test -L ${RUST_LIB_PATH}/rust-http/build main.rs
	./main
