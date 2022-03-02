# Sample Makefile for the WACC Compiler lab: edit this to build your own comiler

# Useful locations

SOURCE_DIR	 := src
OUTPUT_DIR	 := target

# The make rules:

# run the antlr build script then attempts to compile all .java files within src/antlr
all: wacc

wacc_docker:
	docker build -t wacc_32 --target release .

wacc:
	cargo build --release

test: test_unit test_integration

test_unit:
	cargo test

test_integration: wacc
	(cd ./test_integration && bash ./test_integration.sh)

# clean up all of the compiled files
clean:
	cargo clean
	rm -rf ./test_integration/wacc_examples_assembled/wacc_examples_assembled_ours
	rm -rf ./test_integration/wacc_examples_exit_codes/wacc_examples_exit_codes_ours

.PHONY: all clean test test_unit test_integration wacc wacc_docker
