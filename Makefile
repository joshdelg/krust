# I refuse to use the implicit rules because they make me sad :(

# Toolchain vars
ARM := arm-none-eabi
CC := ${ARM}-gcc
ASM := ${ARM}-as
OBJ := ${ARM}-objdump
BIN := ${ARM}-objcopy

# Directory management
SRC_DIR := ./src/
RUST_DIR := ./target/arm-none-eabihf/debug/

# Flags, flags, flags
AS_FLAGS := -c --warn --fatal-warnings -mcpu=arm1176jzf-s -march=armv6zk
CC_FLAGS := -O -Wall -Wno-unused-variable -Werror -MMD -nostdlib -nostartfiles -ffreestanding  -march=armv6 -std=gnu99

MEMMAP := ${SRC_DIR}memmap

run: krust.bin
	my-install $^

krust.bin: krust.elf
	${BIN} $^ -O binary $@


krust.elf: ${SRC_DIR}start.o ${RUST_DIR}libkrust.rlib
	${CC} ${CC_FLAGS} -T ${MEMMAP} $^ -o $@
	${OBJ} -D $@ > krust.list

${SRC_DIR}start.o: ${SRC_DIR}start.S
	${ASM} ${AS_FLAGS} $^ -o $@

${RUST_DIR}libkrust.rlib: ${SRC_DIR}lib.rs
	xargo build --target arm-none-eabihf

clean:
	rm -rf target
	rm -f $(wildcard *.o) $(wildcard *.elf) $(wildcard *.list) $(wildcard *.bin)
	rm -f $(wildcard ${SRC_DIR}*.o)