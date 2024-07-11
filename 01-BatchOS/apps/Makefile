TARGET := riscv64gc-unknown-none-elf
PROFILE ?= release
APP_SOURCE_FOLDER := src/bin
TARGET_FOLDER := target/$(TARGET)/$(PROFILE)

APP_LIST := $(wildcard $(APP_SOURCE_FOLDER)/*.rs)
APP_ELF_LIST := $(patsubst $(APP_SOURCE_FOLDER)/%.rs, $(TARGET_FOLDER)/%, $(APP_LIST))
APP_BIN_LIST := $(patsubst $(APP_SOURCE_FOLDER)/%.rs, $(TARGET_FOLDER)/%.bin, $(APP_LIST))

# tools
OBJCOPY_BIN := rust-objcopy
OBJCOPY_OPTS := --binary-architecture=riscv64 --strip-all -O binary

OBJDUMP_BIN := rust-objdump 
OBJDUMP_OPTS := --arch-name=riscv64

.PHONY: build clean elf
.DEFAULT_GOAL := build

build: elf
	$(foreach elf, $(APP_ELF_LIST), $(OBJCOPY_BIN) $(OBJCOPY_OPTS) $(elf) --strip-all -O binary $(patsubst $(TARGET_FOLDER)/%, $(TARGET_FOLDER)/%.bin, $(elf));)

elf:
	@cargo build --release

clean:
	@cargo clean