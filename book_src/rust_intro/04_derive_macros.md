# Derive Macros

First of all if you didn't use the git repo but started on your own:

You're required to add serde for serialization and deserialization to the project:

```shell
cargo add serde
cargo add serde_json
```

This add dependencies to our `Cargo.toml` which should look similar too

```toml
#...
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Derive Functionality with Macros

We will now switch from self implemented methods to derive macros:

```rust
use serde::{Deserialize, Serialize};
use std::fs;

// this time we derive the implementation of fn default():
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct DistillationColumn {
    trays: i32,
    feed_place: i32,
    reflux_ratio: f32,

    // often macros add more customizations with attributes:
    #[serde(rename = "d2f")]
    distiliate_to_feed_ratio: f32,
}
// ...
```

Feel free to explore **04_derive.rs** for the `main()` and `export_json()` functions. 

The souce code is less than 30 lines and we export the distillation column to json.

If you run `cargo run --bin derive` you should see the following output:

```json
JSON written to output.json:
{
  "trays": 0,
  "feed_place": 0,
  "reflux_ratio": 0.0,
  "d2f": 0.0
}
```

## What does derive do?

Shortly spoken derive generates code based on the underlying struct and additional annotations like the `#[serde(rename = "d2f")]`

Let's investigate how the code looks

1. Ensure you have `cargo-expand` installed:

```shell
cargo install cargo-expand
```

2. Use `cargo-expand` on the source:

```shell
cargo expand --bin derive
```

Most of the code is the serde implementation, expanded it has more than 400 lines of code.

We focus on the `PartialEq` code for now:

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for DistillationColumn {
    #[inline]
    fn eq(&self, other: &DistillationColumn) -> bool {
        self.trays == other.trays && self.feed_place == other.feed_place
            && self.reflux_ratio == other.reflux_ratio
            && self.distiliate_to_feed_ratio == other.distiliate_to_feed_ratio
    }
}
```

- it implements the trait `::core::cmp::PartialEq` for `DistillationColumn`
- In it default derived form it simply uses `==` for all field names.

Feel free to explore the remaing code. It might be easier in a file:

```shell
cargo expand --bin derive > tmp.rs
```
