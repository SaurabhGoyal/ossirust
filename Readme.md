# Overview
A simple OS developed by following the guide given at [https://os.phil-opp.com/](https://os.phil-opp.com/).

`Ossirus` is considered the **god of underworld**. OS is the underworld for most application software. I named this os `Ossirust` as it consists of OS, Ossirus and Rust, I know cheeky :p.

# Build
- Add one of the bare metal targets
```
rustup target add thumbv7em-none-eabihf
```
- Build for your target
```
cargo build --target thumbv7em-none-eabihf
```
