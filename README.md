# `🚀 Starship`

> A Starlight kernel loader via UART

![Crates.io License](https://img.shields.io/crates/l/starlight-comet?style=for-the-badge&color=3079B3)
![GitHub Release](https://img.shields.io/github/v/release/StarlightConsole/Starship?style=for-the-badge&color=3079B3)

## Installation

### Pre-built binaries

Installing Starship manually will require a properly formatted microSD card for the Raspberry Pi,
with a [`config.txt ↗`](https://www.raspberrypi.com/documentation/computers/config_txt.html)
including `arm_64bit=1` and `init_uart_clock=48000000`.
([`JTAG ↗`](https://en.wikipedia.org/wiki/JTAG) support is currently WIP)

When your SD card is formatted and configured, simply put the latest
[`kernel8.img ↗`](https://github.com/StarlightConsole/Starship/releases/latest/downloads/kernel8.img)
(found under the [`Releases ↗`](https://github.com/StarlightConsole/Starship/releases/latest) tab)
onto the card and boot the device.

### Manual Installation

To build the Starship binary, you're gonna need the [`Rust Compiler ↗`](https://github.com/rust-lang/rust)
and [`Cargo ↗`](https://github.com/rust-lang/cargo), Rust's Package Manager
(you'll probably have both of which if you're lurking in Starlight's repositories ;).
The easiest way to install Rust is with [`Rustup ↗`](https://github.com/rust-lang/rustup).
You're also going to need some additional tools such as [`Make ↗`](https://www.gnu.org/software/make/),
but you already have those installed (if you don't, you should consider to *not* dive head-first into the world of OS development).
Lastly, run `make all` in your favorite shell.

## Features

* `☄️ Comet Integration`
Starship integrates tightly with [`☄️ Comet ↗`](https://github.com/StarlightConsole/Comet),
enabling [`🌟 Starlight ↗`](https://github.com/StarlightConsole/Starlight) developers to upload new Starlight builds via UART.

* `✨ No Runtime Overhead`
The Starship binary has full relocation support, which means that Starship moves itself out of the way for the kernel.
Starship operates only on startup, making the kernel regain the memory Starship used after the kernel has been loaded.
This results in **no runtime overhead**! :)

## Usage

See [`☄️ Comet ↗`](https://github.com/StarlightConsole/Comet)'s repository for examples of Starship usages.

## Misc

Starship and related projects are designed and developed by [`@yolocat-dev ↗`](https://github.com/yolocat-dev), though contributions are always welcome!

Starship is, as our other repositories, licensed under the Apache License 2.0. Feel free to read the actual legal stuff in the [`LICENSE ↗`](https://github.com/StarlightConsole/Starship/blob/main/LICENSE) file.
