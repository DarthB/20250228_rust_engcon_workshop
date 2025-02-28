# Modularization for Macros

---

Rust code --> `parse` --> `filter` ---> `analyze` ---> `intermediate` ---> `codegen` ---> Rust Code

---

- `parse` - use the macro `parse_macro_input!(input as DeriveInput);` to get a syntax tree
- `filter` - reduce the to syntax tree that only contains elements needed
- `analyze` - check rules of domain-specific language (DSL)
  - e.g. we only support `<` `<=` `>=` and `>` as binary operators in validation rules
- `intermediate` - transform into a structure that is easy to use for code generation
  - e.g. we replace a variable `trays` in the rules with `self.trays`
- `codegen` - Cannot fail if we were able to produce intermediate code

**Remark:** All steps but `codegen` might fail with `syn::Error`

## Reminder Function Signature for Macro Implementations

For Derive Macros, e.g. `Default` and so on.

```rust
#[proc_macro]
pub fn macro_name(input: TokenStream) -> TokenStream {
    // ...
}
```

For Attribue Macros (Tokio Async Example)

```rust
#[proc_macro_attribute]
pub fn macro_name(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
}
```