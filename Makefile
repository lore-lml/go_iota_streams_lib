ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

build:
	cd lib/streams && cargo build
	cp lib/streams/target/debug/libc_streams_lib.so lib/
	go build -ldflags="-r $(ROOT_DIR)lib" main.go

run: build
	./main
