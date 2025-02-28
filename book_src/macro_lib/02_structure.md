# The Structure of EngCon

We will use two crates:

1. **engcon**: Exposes traits and types, optionally reexports the macros
2. **engcon_macros**: This crate contains the procedural macro implementations

This is a pattern I found in our macro libraris like [strum](https://crates.io/crates/strum) (Strum is a set of macros and traits for working with enums and strings easier in Rust.)


---

```toml
[package]
name = "engcon"
version = "0.1.0"
edition = "2021"
authors = ["Tim Janus <tim@janus.rs>"]
license = "MIT OR Apache-2.0"
#...

[features]
derive = ["engcon_macros"]

[dependencies]
engcon_macros = { path = "../engcon_macros", optional = true, version = "0.1" }

[dev-dependencies]
engcon_macros = { path = "../engcon_macros", version = "0.1" }
```

---

```toml
[package]
name = "engcon_macros"
version = "0.1.0"
edition = "2021"
authors = ["Tim Janus <tim@janus.rs>"]
license = "MIT OR Apache-2.0"
#...

[lib]
path = "src/lib.rs"
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["extra-traits"] }
quote = { version = "1.0" }
proc-macro2 = { version = "1.0" }

[dev-dependencies]
engcon = { path = "../engcon" }

```

---
