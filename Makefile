# Sample Makefile for the WACC Compiler lab: edit this to build your own comiler

# Useful locations

SOURCE_DIR	 := src
OUTPUT_DIR	 := target

# The make rules:

# run the antlr build script then attempts to compile all .java files within src/antlr
all:
	cargo build --release

# clean up all of the compiled files
clean:
	cargo clean

.PHONY: all clean
