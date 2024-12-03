#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo
CRATES_FOLDER=crates
BINDINGS_FOLDER=bindings

# Define binding libraries and their paths
BINDING_LIBS := the-compact the-compact-patch the-compact-server

# Define mapping for destination folders
BINDING_DEST_NAMES := \
	the-compact:v0 \
	the-compact-patch:patch \
	the-compact-server:server

<<<<<<< HEAD
# Define reverse mapping for command convenience
V0_TARGET := the-compact
PATCH_TARGET := the-compact-patch
SERVER_TARGET := the-compact-server

# Function to get destination name from mapping
get_dest_name = $(word 2,$(subst :, ,$(filter $(1):%,$(BINDING_DEST_NAMES))))

# Convenience targets for specific bindings
.PHONY: bindings-v0
bindings-v0: bindings-$(V0_TARGET)

.PHONY: bindings-patch
bindings-patch: bindings-$(PATCH_TARGET)

.PHONY: bindings-server
bindings-server: bindings-$(SERVER_TARGET)
=======
# Function to get destination name from mapping
get_dest_name = $(word 2,$(subst :, ,$(filter $(1):%,$(BINDING_DEST_NAMES))))

BINDING_PATHS := $(addprefix ./lib/,$(BINDING_LIBS))
BINDING_TARGETS := $(addprefix bindings-,$(BINDING_LIBS))
>>>>>>> origin/main

# Target for generating bindings for a specific lib
.PHONY: bindings-%
bindings-%:
	$(eval LIB_NAME := $*)
	$(eval LIB_PATH := ./lib/$(LIB_NAME))
	$(eval BINDING_OUT_PATH := $(LIB_PATH)/out/bindings)
	$(eval DEST_NAME := $(call get_dest_name,$(LIB_NAME)))
	$(eval BINDING_DEST_PATH := $(CRATES_FOLDER)/$(BINDINGS_FOLDER)/$(DEST_NAME))

# Clean old bindings
	rm -rf $(BINDING_OUT_PATH)
	rm -rf $(BINDING_DEST_PATH)/src

# Generate new bindings
	@forge bind --root $(LIB_PATH) --crate-name bindings

# Make sure destination directory exists
	@mkdir -p $(BINDING_DEST_PATH)/src

# Move bindings to the correct location
	@cp -r $(BINDING_OUT_PATH)/src/* $(BINDING_DEST_PATH)/src/

# Meta-target to generate all bindings
.PHONY: bindings
bindings: $(BINDING_TARGETS)

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
