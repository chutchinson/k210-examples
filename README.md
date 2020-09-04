# k210-example

Set of example applications for the K210

## Build

- Requires **Rust 1.43**
- Requires **riscv64gc-unknown-none-elf** target
- Requires Kendryte K210 **kflash.py**

Rust 1.43 is needed due to LLVM requirements for a medium code model and static relocation model in order to produce a binary with a base address suitable for the K210's RISC-V architecture.

```bash
rustup target add riscv64gc-unknown-none-elf
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

### Release

```bash
# Maixduino
cargo build --release --examples --features maixduino
# Other
cargo build --release --examples --features maix
```

## Flash

To build an example and flash using **kflash**

- You may need to specify a COM/tty port using -p
  - COM8 (Windows)
  - /dev/ttyS8 (Linux)
- You may need to specify a different board using -B
  - kd233
  - dan
  - bit
  - bit_mic
  - goE
  - goD
  - maixduino
  - trainer

```bash
# Maixduino SRAM
cargo build --release --features maixduino --example blink
python3 kflash.py -s -B maixduino ./target/riscv64gc-unknown-none-elf/release/examples/blink
# Maixduino Flash
cargo objcopy --features maixduino --example blink --release -- -O blink.bin
python3 kflash.py -B maixduino blink.bin
# MAIX BIT (with mic) SRAM
cargo build --release --features maix --example blink
python3 kflash.py -s -B bit_mic ./target/riscv64gc-unknown-none-elf/release/examples/blink
# MAIX BIT (with mic) Flash
cargo objcopy --features maix --example blink --release -- -O blink.bin
python3 kflash.py -B bit_mic blink.bin
```