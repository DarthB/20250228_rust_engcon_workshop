# Summary and Disclaimer

We crashed through a lot of concepts:

- Setup a Rust Project
- Write Functions
- Create Datastructures
- Methods, Traits and Trait Implementation
- Usage and Expansion of Derive Macros
- Usage and Expansion of Attribute Macros
- Function-Like Macros (vec![], println!())

That's a good preparation for what comes, however...

## Disclaimer

This is just a crash course giving some clues about Rust concepts for people who have a strong background in programming other languages already.

We didn't talk about many Rust features in the deserved detail:

- Ownership only scratched
- Mutability (without that you cannot change a variable)
- Lifetimes not mentioned
- Generics not mentioned

Investing the time to understand these Rust concepts is worthwhile though.

## Next: How do we implement macros?

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
