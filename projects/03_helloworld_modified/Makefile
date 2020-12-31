.SECONDEXPANSION:
MAKEFLAGS += --no-builtin-rules
.SUFFIXES:

MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
CURR_DIR := $(patsubst %/,%,$(dir $(MKFILE_PATH)))

PREFIX := riscv64-unknown-elf-

CC := $(PREFIX)gcc
OBJDUMP := $(PREFIX)objdump

QEMU := qemu-system-riscv64

RUST_KERNEL := kernel
RUST_KERNEL_PROFILE := release
RUST_KERNEL_TARGET := riscv64gc-unknown-none-elf
RUST_KERNEL_OUTPUT := $(CURR_DIR)/$(RUST_KERNEL)/target/$(RUST_KERNEL_TARGET)/$(RUST_KERNEL_PROFILE)/

SRC_IPL := $(CURR_DIR)/boot
SRC_KERNEL := $(CURR_DIR)/kernel

OUTPUT := $(CURR_DIR)/_build

K := $(OUTPUT)/kernel

ASM_SOURCES := $(wildcard $(SRC_IPL)/*.s)

LIB := -l$(RUST_KERNEL)

KERNEL_LIBS := \
	-L$(RUST_KERNEL_OUTPUT)

CFLAGS=-Wall -Wextra -pedantic -Wextra 
CFLAGS+=$(if $(filter-out release,$(RUST_KERNEL_PROFILE)),-O0 -g)
CFLAGS+=-static -ffreestanding -nostdlib -fno-rtti -fno-exceptions
CFLAGS+=-march=rv64gc -mabi=lp64

$K/kernel: $(ASM_SOURCES) $(RUST_KERNEL_OUTPUT)/lib$(RUST_KERNEL) $$K/.f
	$(CC) $(CFLAGS) -T$(CURR_DIR)/kernel.ld $(ASM_SOURCES) $(LIB) $(KERNEL_LIBS) -o $@

$(RUST_KERNEL_OUTPUT)/lib$(RUST_KERNEL):
	cargo build --manifest-path $(SRC_KERNEL)/Cargo.toml $(filter --release,--$(RUST_KERNEL_PROFILE))

-include $(wildcard $(RUST_KERNEL_OUTPUT)/*.d)

%/.f:
	mkdir -p $(dir $@)
	touch $@

.PRECIOUS: %/.f

.PHONY: all clean dasmk
clean: 
	rm -rf $(OUTPUT)/*

all: $K/kernel

CPUS = 1

QEMUOPTS = -machine virt -bios none -m 128M -smp $(CPUS) 
QEMUOPTS += -monitor stdio
QEMUOPTS += -kernel $K/kernel

dasm: $K/kernel 
	$(OBJDUMP) -D -m riscv $^

dasmk: 
	$(OBJDUMP) -D -m riscv $(SRC_KERNEL)/target/$(TARGET_KERNEL)/debug/libkernel.a

qemu: $K/kernel 
	$(QEMU) $(QEMUOPTS)

GDB_PORT = $(shell expr `id -u` % 5000 + 25000)
GDB_QEMUPUTS = -gdb tcp::$(GDB_PORT)

.gdbinit: $(abspath $(CURR_DIR)/../..)/scripts/.gdbinit.tmpl-riscv
	sed "s/:1234/:$(GDB_PORT)/" < $^ > $@

qemu-gdb: $K/kernel .gdbinit
	$(QEMU) $(QEMUOPTS) -S $(GDB_QEMUPUTS)