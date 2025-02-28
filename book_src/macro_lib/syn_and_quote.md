# Lets introduce syn and quote

Syn is a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code.

```rust
#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    // ...
```

Thats the first step we do, and we end up with that syntax tree:

```rust
pub struct DeriveInput {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data,
}
```

## Quote generates Rust Tokenstreams

This crate provides the quote! macro:

1. Write nearlly normal Rust Code
2. Get out the `TokenStream` type

```rust
quote! {
    #[automatically_derived]
    impl TryFrom<#type_name> for Validated<#type_name> {
        type Error = ValidationError;

        fn try_from(value: #type_name) -> Result<Self, Self::Error> {
            match value.validate() {
                Ok(_) => {
                    let reval = unsafe { Validated::new_unchecked(value)};
                    Ok(reval)
                }
                Err(err) => Err(err)
            }
        }
    }
}
```
