target extended-remote 172.24.32.1:2331

set print asm-demangle on

monitor flash device = STM32L476RG
monitor reg r13 = (0x00000000)
monitor reg pc = (0x00000004)
monitor flash download = 1

monitor reset

set backtrace limit 32

break main

file target/thumbv7em-none-eabihf/debug/rust-blinky-stm32l476rg
load target/thumbv7em-none-eabihf/debug/rust-blinky-stm32l476rg
