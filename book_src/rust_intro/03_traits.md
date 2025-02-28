# Methods and Traits

We saw free functions: `move_ds()` and `ref_ds()` earlier. 

Now let's extend the Distllation Column with methods by using the `impl` keyword.

**See 03_traits.rs** for source code.

```rs
impl DistillationColumn {
    pub fn new() -> Self {
        Self {
            trays: 20,
            feed_place: 10,
            reflux_ratio: 1.5,
            distiliate_to_feed_ratio: 0.9,
        }
    }
// ...
}
```

This allows us to call a function related to the type `DistillationColumn`

```rust
let ds = DistillationColumn::new();
```

`new` is a common function name if we create an instance of a data structure or class in other programming languages.

Most often we want functions that operate on a per instance basis. 
We could for example have estimations for energy and costs of the distillation column.

```rs
impl DistillationColumn {
    //...
    pub fn energy(&self) -> f32 {
        1.0 / self.reflux_ratio
    }

    pub fn cost(&self, c0: f32, coeff_a: f32, coeff_b: f32) -> f32 {
        c0 + coeff_a * self.trays as f32 + coeff_b * self.reflux_ratio
    }
}
```

Here a reference of `&self` indicates that these both methods are called on a distillation column instance.

You can use the following code:

```rust
let ds = DistillationColumn::new();
let energy = ds.energy();
```

## Traits

However in the Rust ecosystem the name `default` is widely used to create new instances, which is implemented by the `Default` trait.

Traits are similar to interfaces in other programming languages.

```rs
// trait found in standard library
pub trait Default: Sized {
    fn default() -> Self;
}
```

We can implement traits to fit into the Rust ecosystem.

```rust
// implementation of the trait default
impl Default for DistillationColumn {
    fn default() -> Self {
        Self::new()
    }
}
```

Learning traits of the ecosystem like: `Default, Display, From<T>, TryFrom<T>, Iterator` would be part of a proper beginners course.

See the code for an implementation of `Display`.