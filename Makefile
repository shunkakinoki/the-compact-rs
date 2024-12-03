#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo
CRATES_FOLDER=crates
THE_COMPACT_PATH=./lib/the-compact
BINDINGS_FOLDER=bindings
BINDINGS_CRATES_FOLDER=$(CRATES_FOLDER)/$(BINDINGS_FOLDER)
BINDINGS_OUT_PATH=$(THE_COMPACT_PATH)/out/$(BINDINGS_FOLDER)

# Target for generating bindings
.PHONY: bindings
bindings:
	rm -rf $(BINDINGS_CRATES_FOLDER)/src/*
	rm -rf $(BINDINGS_OUT_PATH)

# Generate new bindings
	@forge bind --root $(THE_COMPACT_PATH) --crate-name $(BINDINGS_FOLDER)

# Make sure directory exists
	@mkdir -p $(BINDINGS_CRATES_FOLDER)/src

# Move bindings to the correct location
	@cp -r $(BINDINGS_OUT_PATH)/src/* $(BINDINGS_CRATES_FOLDER)/src/

# Target for building the project
.PHONY: build
build: bindings
	@$(CARGO) build

# Target for building the project in release mode
.PHONY: build-release
build-release: bindings
	@$(CARGO) build --release

# Target for cleaning the project
.PHONY: clean
clean:
	@forge clean --root $(THE_COMPACT_PATH)
	@$(CARGO) clean

# Target for formatting the code
.PHONY: fmt
fmt:
	@forge fmt --check --root $(THE_COMPACT_PATH)
	@$(CARGO) fmt

# Target for running tests
.PHONY: test
test:
	@forge test --root $(THE_COMPACT_PATH)
	@$(CARGO) test

# Target for installing forge dependencies
.PHONY: setup
setup:
	@forge install --root $(THE_COMPACT_PATH)
