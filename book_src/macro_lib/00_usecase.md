# Use Case - Validation of a Distillation Column

Let's imagine an use-case where an engineer needs to both:

1. Input data that defines the specifications of a distillation column.
2. Simulate and optimize such a distillation column.

In the first case a relaxed type that allows malformed input (at least temporary) is needed. Thus type should provide methods like:



In the second case we would like to use Rust type system to ensure that only valid input for a distillation column is used for `simulation/optimization`.


## How does the code look like?

We use the [engcon](https://crates.io/crates/engcon) crate.

```rust
use engcon::*;

#[derive(Debug, Clone, Default, Copy, PartialEq, Validatable)]
pub struct DistillationColumn {
    #[validate_value(x >= 3)]
    pub trays: i32,

    #[validate_value(x < trays, x >= 1)]
    pub feed_place: i32,

    #[validate_value(x > 0.0)]
    pub reflux_ratio: f32,

    #[validate_value(x > 0.0, x < 1.0)]
    pub distiliate_to_feed_ratio: f32,
}
```
