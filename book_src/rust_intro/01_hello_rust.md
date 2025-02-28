# Hello Rust

**See 01_hello.rs** for source code.

First see how we could setup a project on our own.

Type:

```shell
cargo new 01_hello_rust
```

Let's switch in the directory and start the project:

```shell
cd 01_hello_rust
cargo run
```

or if you using GitHub simply type:

```shell
cargo run --bin hello
```

And you receive the output:

```
   Compiling hello_rust v0.1.0 (E:\Repositories\rust_proc_macros\workshop\code\01_hello_rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running `E:\Repositories\rust_proc_macros\target\debug\hello_rust.exe`
Hello, world!
```

In the directory the following files have been created:

- Cargo.toml
- src/main.rs

Let's have a look at those files:

The `.rs` source file containing a hello world.

```rust
fn main() {
    println!("Hello, world!");
}
```
and a `.toml` file with the project/package description:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2024"

[dependencies]
```

In the workshop we use several binaries the toml looks a bit different:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "hello"
path = "src/01_hello.rs"

# ...
[dependencies]
# ...
```