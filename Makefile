.PHONY: shell clean docker-crosscompile package-app

TOOLCHAIN_NAME = miyoomini-rust-sdl-toolchain
WORKSPACE_DIR := $(shell pwd)
TARGET_BIN = rust-sdl-demo
TARGET_DIR = target/armv7-unknown-linux-gnueabihf/release
OUTPUT_DIR = RustSDLDemo

# Check if Docker image exists
DOCKER_IMAGE_EXISTS := $(shell docker images -q $(TOOLCHAIN_NAME))

# Build Docker image if it doesn't exist
build-image:
ifndef DOCKER_IMAGE_EXISTS
	@echo "Building Docker image..."
	@docker build -t $(TOOLCHAIN_NAME) .
endif

# Open a shell inside the Docker container
shell: build-image
	@docker run -it --rm -v "$(WORKSPACE_DIR)":/root/workspace $(TOOLCHAIN_NAME) /bin/bash

# Cross-compile Rust project for ARMv7
docker-crosscompile: build-image
	@echo "Starting cross-compilation..."
	@docker run -it --rm -v "$(WORKSPACE_DIR)":/root/workspace $(TOOLCHAIN_NAME) /bin/bash -l -c "cargo build --release --target armv7-unknown-linux-gnueabihf"

# Package application files
package-app: docker-crosscompile
	@echo "Packaging application..."
	@mkdir -p $(OUTPUT_DIR)
	@cp $(TARGET_DIR)/$(TARGET_BIN) $(OUTPUT_DIR)/
	@cp -r assets $(OUTPUT_DIR)/
	@cp -r app-support/* $(OUTPUT_DIR)/
	@echo "Application packaged in $(OUTPUT_DIR)/. Copy this directory to /mnt/SDCARD/Apps/ in your Miyoo Mini+."

# Clean up Docker image and build artifacts
clean:
	@echo "Cleaning up..."
	@docker rmi -f $(TOOLCHAIN_NAME) || true
	@rm -rf $(OUTPUT_DIR)
