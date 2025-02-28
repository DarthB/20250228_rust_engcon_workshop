# Variables and Data Structures

A data structure in Rust is defined with the `struct` keyword:

```rust
pub struct DistillationColumn {
    trays: i32,
    feed_place: i32,
    reflux_ratio: f32,
    distiliate_to_feed_ratio: f32,
}
```

**See 02_data.rs** for source code.

In a data structure fields are defined by a field name followed by a doube colon and then the type.
Fields are separated by comma. More information about [Rust types here](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch03-02-data-types.html).

Variables defined in function scope can often deduct their type.

```rust


fn main() {
    let stages = 20;
    let feed_place = 10;
    let reflux_ratio = 1.5;
    let d2f = 0.9;
    
    // force type with
    // let d2f:f32 = 0.9;

    // ...
```

A data structure can be initialized with an initializer list.

Normally the field name is followed by a double colon and then the value.

However if the variable of the outer-scope fits the field name a shortcut is possible.

```rust
    let ds = DistillationColumn {
        trays: stages,                 // different names
        feed_place,                    // var name as field name
        reflux_ratio,                  // var name as field name
        distiliate_to_feed_ratio: d2f, // different names
    };
    // ...
```

Rust has a strong ownership system. Therefore we distinguish between owned values and references:

```rust
    // ...
    ref_ds(&ds);
    move_ds(ds);
}

fn move_ds(ds: DistillationColumn) {
    println!(
        "MOVE: Distillation Column has {} trays and feeds at {} and operaters with d2f={}, and rr={}",
        ds.trays, ds.feed_place, ds.distiliate_to_feed_ratio, ds.reflux_ratio
    );
}

fn ref_ds(ds: &DistillationColumn) {
    println!(
        "REF:  Distillation Column has {} trays and feeds at {} and operaters with d2f={}, and rr={}",
        ds.trays, ds.feed_place, ds.distiliate_to_feed_ratio, ds.reflux_ratio
    );
}
```

Ok let's just run it:

```shell
cargo run --bin data
```

```
   Compiling hello_rust v0.1.0 (E:\Repositories\rust_proc_macros\workshop\code\01_hello_rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `E:\Repositories\rust_proc_macros\target\debug\data.exe`
REF:  Distillation Column has 20 trays and feeds at 10 and operaters with d2f=0.9, and rr=1.5
MOVE: Distillation Column has 20 trays and feeds at 10 and operaters with d2f=0.9, and rr=1.5
```

## The borrow checker, the unusal friend you need to get used to

The function output is the same!

However they do not do the same!

If we change the order of the function calls we receive:

```shell
error[E0382]: borrow of moved value: `ds`
  --> workshop\code\01_hello_rust\src\02_data.rs:22:12
   |
13 |     let ds = DistillationColumn {
   |         -- move occurs because `ds` has type `DistillationColumn`, which does not implement the `Copy` trait
...
20 |     move_ds(ds);
   |             -- value moved here
21 |
22 |     ref_ds(&ds);
   |            ^^^ value borrowed here after move
   |
note: consider changing this parameter type in function `move_ds` to borrow instead if owning the value isn't necessary
  --> workshop\code\01_hello_rust\src\02_data.rs:25:16
   |
25 | fn move_ds(ds: DistillationColumn) {
   |    -------     ^^^^^^^^^^^^^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `DistillationColumn` implemented `Clone`, you could clone the value
  --> workshop\code\01_hello_rust\src\02_data.rs:1:1
   |
1  | pub struct DistillationColumn {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
20 |     move_ds(ds);
   |             -- you could clone this value

```

**This is due to Rusts Ownership Model**

The variable `ds` is moved into the function thus `main` is not the owner anymore and can not
use a reference to the variable `ds` anymore.