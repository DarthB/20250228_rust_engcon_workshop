# Outline of the Codegen Tasks

What kind of codes shall we generate?

1. Add contract methods of the form:
   - `pub fn contract_trays(&self) -> Result<(), ValidationError>`
   - `pub fn contract_reflux_ratio(&self) -> Result<(), ValidationError>`
   - ...
2. Implement the trait `Validator` using those contract methods

    ```rust
    pub trait Validator {
        fn validate(&self) -> Result<(), ValidationError>;
    }
    ```

3. Implement `TryFrom<DistillationColumn> for ValidatedDistillationColumn` for covenience
4. Implement `Into<DistillationColumn> for ValidatedDistillationColumn` for convenience

## Lets have a look an the Expansion

```shell
cargo expand --bin engcon
```

## Lets investiage EngCon Code

See `vendor/engcon_macros/src/validator/validator_codegen.rs`

