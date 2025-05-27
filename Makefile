.PHONY : all help build flash run clean

CARGO := cargo
WLINK := wlink
BUILD_TYPE := release
ARCH := riscv32imfc-unknown-none-elf
TARGET := target/$(ARCH)/$(BUILD_TYPE)/demo

all: help

help:
	@echo "This is a Makefile for the project."
	@echo "Please use 'make <target>' to build specific targets."
	@echo "Available targets:"
	@echo "  help		- Show this help message"
	@echo "  build		- Build the project"
	@echo "  flash		- Flash the project to the target device"
	@echo "  run		- Build and flash"
	@echo "  clean		- Clean up build artifacts"

build:
	@$(CARGO) $@ --$(BUILD_TYPE)

WLINK_FLAGS := -v flash --enable-sdi-print --watch-serial

flash:
	@$(WLINK) $(WLINK_FLAGS) $(TARGET)

run:
	@$(CARGO) $@ --$(BUILD_TYPE)

clean:
	@$(CARGO) $@

%:
	@$(CARGO) $@
