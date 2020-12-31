MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
CURR_DIR := $(patsubst %/,%,$(dir $(MKFILE_PATH)))

QEMU = qemu-system-riscv64

CPUS = 3

QEMUOPTS = -machine virt -bios none -m 128M -smp $(CPUS) 
QEMUOPTS += -monitor stdio

qemu: 
	$(QEMU) $(QEMUOPTS)