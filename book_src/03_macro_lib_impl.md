# Implementation Details of EngCon

You can find EngCon here:

- [GitHub Repository](https://github.com/DarthB/engcon) - Happy if you leave a star!
- [Package on Crates.io](https://crates.io/crates/engcon)
- [Documentation on docs.rs](https://docs.rs/engcon/0.1.0/engcon/)

**All the tooling also to publish Rust crates is part of `cargo`**

## How to best access the code

Run the command `cargo vendor` in the root folder and cargo makes all
crates accessible from your harddrive. Thats easy to navigate and works
if your offline.

This means you have all the information you need on your computer:

```shell
cargo doc --open
```

Creates the documentation with all dependencies.

**Very nice when traveling**

Rust comes with a very thoughtful collection of tools integrated in cargo. 