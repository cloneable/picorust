# picorust

Raspberry Pi Pico experiments with Rust.

## Setup

```
sudo apt install automake autoconf build-essential libtool gdb-multiarch binutils-multiarch gcc-arm-none-eabi gcc g++ libftdi-dev libusb-1.0.0-dev libudev-dev minicom netcat

rustup component add llvm-tools-preview
rustup target add thumbv6m-none-eabi

cargo install cargo-binutils rustfilt elf2uf2-rs probe-run flip-link defmt-print
```

```
sudo raspi-config nonint do_serial 2
reboot
```

## Debug

Manual debug session:

```
cargo build
gdb-multiarch target/thumbv6m-none-eabi/debug/picorust

./scripts/gdb-listen.sh
gdb> target extended-remote localhost:3333
gdb> load
```

Show serial/UART output:

```
minicom -b 115200 -D /dev/serial0
```

Translate RTT defmt logging output via OpenOCD:

```
nc localhost 4444 | defmt-print -e target/thumbv6m-none-eabi/debug/picorust
```
