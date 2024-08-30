# Overview
A simple OS developed by following the guide given at [https://os.phil-opp.com/](https://os.phil-opp.com/).

`Ossirus` is considered the **god of underworld**. OS is the underworld for most application software. I named this os `Ossirust` as it consists of OS, Ossirus and Rust, I know cheeky :p.

# Build
- Set Rust nightly
```
rustup override set nightly
```
- Add custom target for bare-metal
```
rustup target add x86_64-unknown-none
```
- Adding source-code component to rebuild `core` crate from
```
rustup component add rust-src
```
- Adding llvm tools for correct linking using `rust-lld`.
```
rustup component add llvm-tools-preview
```
- Build for your target. (`-Z` tells to install two unstable features - `build-std` and `build-std-features` with former telling which crates to build from scratch for custom target and latter telling which features to enable when building.)
```
cargo build -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem  --target x86_64_ossirust.json 
```

## Output
```
saurabh@Saurabh-Raider:/mnt/d/Saurabh/Personal/ossirust$ cargo build -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem  --target x86_64_ossirust.json 
   Compiling compiler_builtins v0.1.109
   Compiling core v0.0.0 (/home/saurabh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling rustc-std-workspace-core v1.99.0 (/home/saurabh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling ossirust v0.1.0 (/mnt/d/Saurabh/Personal/ossirust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 17.43s
saurabh@Saurabh-Raider:/mnt/d/Saurabh/Personal/ossirust$ 
```