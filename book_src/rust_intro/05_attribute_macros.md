# Attribute and Function-like Macros 

We already saw function like macros:

```rs
let word = "amazing";
println!("Rust is {}!", word);
```

Another variant is the `vec![]` macro:

```rs
let array = vec![1, 2, 3, 42];
```

## Declarative Macros

`vec![]` is declarative macro and defined as:

```rs
macro_rules! vec {
    () => (
        $crate::vec::Vec::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::vec::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        <[_]>::into_vec(
            // This rustc_box is not required, but it produces a dramatic improvement in compile
            // time when constructing arrays with many elements.
            #[rustc_box]
            $crate::boxed::Box::new([$($x),+])
        )
    );
}
```

There are many sources online about delcarative macros, here we will focus on:

**Procedural Macros**

## Attribute Macros

Attribute macros like derive macros are always procedural macros.

Function like macros can be both declarative and procedural macros. The user of the macro can not distinguish that without checking the definition.

An attribute macro can change the behavior of a function:

```rust
[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // we do nothing here
    Ok(())
}
```

See **05_tokio.rs** for the source code

Run cargo expand:

```shell
cargo expand --bin tokio
```