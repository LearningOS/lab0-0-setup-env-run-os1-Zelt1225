# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
LLVMOBJCOPY := llvm-objcopy --binary-architecture=riscv64

build: $(KERNEL_BIN)

$(KERNEL_BIN): kernel
	@$(LLVMOBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@


kernel:
	@cargo build --release

clean:
	@cargo clean

run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

dbg: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-s -S
gdb:
	@riscv64-unknown-elf-gdb \
		-ex 'file target/riscv64gc-unknown-none-elf/release/os' \
		-ex 'set arch riscv:rv64' \
    	-ex 'target remote localhost:1234'



