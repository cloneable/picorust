# picorust

Raspberry Pi Pico experiments with Rust.

## Setup

```
sudo apt install automake autoconf build-essential libtool gdb-multiarch gcc-arm-none-eabi gcc g++ libftdi-dev libusb-1.0.0-dev libudev-dev

rustup component add llvm-tools-preview
rustup target add thumbv6m-none-eabi

cargo install cargo-binutils rustfilt elf2uf2-rs probe-run flip-link
```

```
sudo apt install minicom
sudo raspi-config nonint do_serial 2
reboot
```

## Debug

> TODO: VSCode debug

```
cargo run
gdb-multiarch target/thumbv6m-none-eabi/debug/picorust

./scripts/gdb-listen.sh
gdb> target extended-remote localhost:3333
gdb> load
```