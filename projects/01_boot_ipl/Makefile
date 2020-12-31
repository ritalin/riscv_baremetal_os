.SECONDEXPANSION:
MAKEFLAGS += --no-builtin-rules
.SUFFIXES:

MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
CURR_DIR := $(patsubst %/,%,$(dir $(MKFILE_PATH)))

PREFIX := riscv64-unknown-elf-

CC := $(PREFIX)gcc
OBJDUMP := $(PREFIX)objdump

CFLAGS := -Wall -Wextra -pedantic -Wextra 
CFLAGS += -static -ffreestanding -nostdlib
CFLAGS+=-march=rv64gc -mabi=lp64

QEMU := qemu-system-riscv64

SRC_IPL := $(CURR_DIR)/boot

OUTPUT := $(CURR_DIR)/_build

K := $(OUTPUT)/kernel

ASM_SOURCES := $(wildcard $(SRC_IPL)/*.s)

OBJS := \
	$K/entry.o

$K/entry.o: $(SRC_ENTRY)/entry.s
	$(AS) -o $@ $<

$K/kernel: $(ASM_SOURCES) $$K/.f
	$(CC) $(CFLAGS) -T $(CURR_DIR)/kernel.ld $(ASM_SOURCES) -o $@

ec:
	echo $(dir )
%/.f:
	mkdir -p $(dir $@)
	touch $@

.PRECIOUS: %/.f

.PHONY: all clean
clean: 
	rm -rf $(OUTPUT)/*

all: $K/kernel 

CPUS = 3

QEMUOPTS = -machine virt -bios none -m 128M -smp $(CPUS) 
QEMUOPTS += -monitor stdio
QEMUOPTS += -kernel $K/kernel

dasm: $K/kernel 
	$(OBJDUMP) -D -m riscv $^

qemu: $K/kernel 
	$(QEMU) $(QEMUOPTS)

GDB_PORT = $(shell expr `id -u` % 5000 + 25000)
GDB_QEMUPUTS = -gdb tcp::$(GDB_PORT)

.gdbinit: $(abspath $(CURR_DIR)/../..)/scripts/.gdbinit.tmpl-riscv
	sed "s/:1234/:$(GDB_PORT)/" < $^ > $@

qemu-gdb: $K/kernel .gdbinit
	$(QEMU) $(QEMUOPTS) -S $(GDB_QEMUPUTS)