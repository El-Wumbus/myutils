FILES=$(wildcard src/*.c)
CC=gcc
CFLAGS=-O2 -Wall -Wextra 
OUTFILE=whoami

defualt: build

build: clean
	mkdir -p ./build
	$(CC) $(FILES) -o build/$(OUTFILE) $(CFLAGS)

run: build
	./build/whoami

clean:
	rm -rf ./build