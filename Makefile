#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo
CRATES_FOLDER=crates
CONTRACTS_PATH=./contracts
BINDINGS_FOLDER=bindings
BINDINGS_CRATES_FOLDER=$(CRATES_FOLDER)/$(BINDINGS_FOLDER)
BINDINGS_OUT_PATH=$(CONTRACTS_PATH)/out/$(BINDINGS_FOLDER)

# Target for generating bindings
.PHONY: bindings
bindings:
	rm -rf $(BINDINGS_CRATES_FOLDER)
	rm -rf $(BINDINGS_OUT_PATH)

# Generate new bindings
	@forge bind --root $(CONTRACTS_PATH) --crate-name $(BINDINGS_FOLDER)

# Move bindings to the correct location
	@mv -f $(BINDINGS_OUT_PATH) $(CRATES_FOLDER)

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
	@forge clean --root $(CONTRACTS_PATH)
	@$(CARGO) clean

# Target for formatting the code
.PHONY: fmt
fmt:
	@forge fmt --check --root $(CONTRACTS_PATH)
	@$(CARGO) fmt

# Target for running tests
.PHONY: test
test:
	@forge test --root $(CONTRACTS_PATH)
	@$(CARGO) test

# Target for installing forge dependencies
.PHONY: setup
setup:
	@forge install --root $(CONTRACTS_PATH)
