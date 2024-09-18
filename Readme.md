+++
title = "[In-Progress] Ossirust: Operating Systems from scratch in Rust"
date = 2024-09-18
description = "Learnings from writing an operating system."

[taxonomies]
tags = ["kernel", "operating-system", "process-scheduling", "system", "rust"]

[extra]
toc = true
quick_navigation_buttons = true
+++

# Operating System


# Overview
A simple OS developed by following the guide given at [https://os.phil-opp.com/](https://os.phil-opp.com/).

`Ossirus` is considered the **god of underworld**. OS is the underworld for most application software. I named this os `Ossirust` as it consists of OS, Ossirus and Rust, I know cheeky :p.

# Implementation

**Code available at https://github.com/SaurabhGoyal/ossirust**

Here we cover the high level view of what we are doing and how. **This follows the tutorial given at [https://os.phil-opp.com/](https://os.phil-opp.com/) but not strictly**, for ex.- for multi-tasking, it implements process scheduler and not async/await as given in the tutorial.
- Bare bones - Getting a simple binary running on the bare metal with ability to show something on the screen.
- Errors (Exceptions) and Input (Interrupts) - Handling errors that can happen during execution. Getting input from external events.
- Memory Management - Things like access control, efficient read-writes of memory.
- MultiTasking - Processes and their scheduling.

## Bare metal rust binary
- Build process
    - Set the bare metal target by setting the architure tripler to use x86_64 architecture without any OS and host.
    - Set the linker to use rust-linker for so that builds the C dependencies without assuming their presence.
    - Recompile needed `core` libs for the custom target.
    - Setting the panic handling strategy to abort the program.
    - This is supported by two things -
        - The custom target file [`x86_64_ossirust.json`](https://github.com/SaurabhGoyal/ossirust/x86_64_ossirust.json)
        - The cargo config file that defines parameters to pass in cargo build process [`.cargo/config.toml`](https://github.com/SaurabhGoyal/ossirust/.cargo.config.toml)
- Booting and running the binary
    - Lots of bootloader magic - essentially two things -
        - Making your kernel bootable by adding logic of standard boot protocols such as ELF to the kernel code - crate `bootable` does that for us and generates.
        - Making a boot disk / image using the bootable kernel build - an application `bootimage` does that for us.
            - This works as a plugin as well. We can build the image using `cargo bootimage`
    - We finally have the bootable image which can be run in a machine or an emulator. For us, we tested in `qemu` using following command -
        `qemu-system-x86_64 -drive format=raw,file=./target/x86_64_ossirust/debug/bootimage-ossirust.bin`
    - A shortcut to achieve above faster is simply setting `bootimage runner` as the default cargo runner in [`.cargo/config.toml`](https://github.com/SaurabhGoyal/ossirust/.cargo.config.toml).

## Build and Run
- Install qemu emulator
- Set Rust nightly - `rustup override set nightly`
- Add custom target for bare-metal - `rustup target add x86_64-unknown-none`
- Adding source-code component to rebuild **core** crate - `rustup component add rust-src`
- Adding llvm tools for correct linking using **rust-lld** - `rustup component add llvm-tools-preview`
- Run - `cargo run`

# Reading Material
- [https://os.phil-opp.com/](https://os.phil-opp.com/)
- Related posts -
    - [Digital Machine Layer by Layer](../digital-machine-layer-by-layer)
    - [Implementing a 16-bit Virtual Machine](../virtual_machine)
