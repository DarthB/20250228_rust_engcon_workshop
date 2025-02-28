# Error Handlng

The syn crate provides some help for error handling, you can use tokens to locate the error.

```rust
let struct_: syn::DataStruct = match input.data {
    syn::Data::Struct(data) => data,
    _ => {
        return Err(syn::Error::new_spanned(
            self_type,
            "Validator Macros only work on structs yet",
        ));
    }
};
```

## Let's add an Validatable Enum

...

## Another Example

We only support boolean binary operators in our rules:

```rust
if !matches!(
            candidate.cmp_op,
            syn::BinOp::Ge(_) | syn::BinOp::Gt(_) | syn::BinOp::Le(_) | syn::BinOp::Lt(_)
        ) {
            // operator must be `<`, `<=`, `>=` or `>`
            return Err(syn::Error::new_spanned(
                candidate.cmp_op, 
                "Only <, <=, >= and > are supported operators"
            ));
        }
```

## Let see this live too

...
