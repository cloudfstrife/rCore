# build env
TARGET := riscv64gc-unknown-none-elf
PROFILE ?= release
BUILD_OPTS := 
ifeq ($(PROFILE),release) 
	BUILD_OPTS += --release
else ifeq ($(PROFILE),debug) 
	KERNEL_ELF := target/$(TARGET)/$(PROFILE)/os
	KERNEL_BIN := $(KERNEL_ELF).bin
endif

# build output
KERNEL_ELF := target/$(TARGET)/$(PROFILE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

# tools
OBJCOPY_BIN := rust-objcopy
OBJCOPY_OPTS := --binary-architecture=riscv64 --strip-all -O binary

OBJDUMP_BIN := rust-objdump 
OBJDUMP_OPTS := --arch-name=riscv64

# qemu
KERNEL_ENTRY_PA := 0x80200000
## sbi
BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../../sbi/$(SBI)-$(BOARD).bin
## qemu
QEMU_BIN=qemu-system-riscv64
QEMU_OPTS := -machine virt \
-nographic \
-bios $(BOOTLOADER) \
-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

# gdb
GDB_BIN := gdb-multiarch
GDB_OPTS := -ex \'file $(KERNEL_ELF)\' \
-ex \'set arch riscv:rv64\' \
-ex \'target remote localhost:1234\'


# target
$(KERNEL_ELF):
	cargo build ${BUILD_OPTS}

$(KERNEL_BIN):$(KERNEL_ELF)
	$(OBJCOPY_BIN) $(KERNEL_ELF) $(OBJCOPY_OPTS) $@

.PHONY: build clean debug gdb
.DEFAULT_GOAL := build

clean:
	@cargo clean

build:$(KERNEL_BIN)

run:$(KERNEL_BIN)
	$(QEMU_BIN) $(QEMU_OPTS)

debug:$(KERNEL_BIN)
	$(QEMU_BIN) $(QEMU_OPTS) -s -S

gdb:
	@echo $(GDB_BIN) $(GDB_OPTS)