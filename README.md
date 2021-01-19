# About

This is a crude rust blinky demo for a Nucleo-L476LG board that has a STM32L476RG
Cortex-M4F MCU.

# Prerequisites

## Required
Requires Rust. See https://www.rust-lang.org/tools/install for details on how to
do that for your particular OS. For me running in WSL2:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

We'll be cross-compiling for an ARM Cortex-M4F. To get the compiler for this:

```
rustup target add thumbv7em-none-eabihf
```

## Optional
If you want to debug a program you will need gdb:

```
sudo apt install gdb-multiarch
```

Debugging would also need a suitable debug probe, and possibly OpenOCD. But I used the ST Link to J-Link update:

https://www.segger.com/products/debug-probes/j-link/models/other-j-links/st-link-on-board/

The whole gcc-arm-none-eabi toolchain to get size, readelf, nm, etc...:

```
sudo apt-get install gcc-arm-none-eabi
```

# Project Generation

The project was kickstarted with:

cargo install cargo-generate
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart

# Building
```
cargo-build
```

# Running

Start a GDB server. I use the GUI JLinkGDBServer.  But it could be OpenOCD.

Modify .gdbinit to have the IP address of the GDB server.

```
gdb-multiarch
```
