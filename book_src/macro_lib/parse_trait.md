# Parse Trait for Validation Rule

- See the `Parse` trait of implementation for `ValidationRule`
  
  ```rust
  pub trait Parse: Sized {
      fn parse(input: ParseStream) -> Result<Self>;
  }
  ```

  ```rust
  #[validate_value(x < trays, x >= 1)]
  pub feed_place: i32,
  ```

## Look at the code together
