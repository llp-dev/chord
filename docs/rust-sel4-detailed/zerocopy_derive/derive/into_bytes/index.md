*[zerocopy_derive](../../index.md) / [derive](../index.md) / [into_bytes](index.md)*

---

# Module `into_bytes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_into_bytes`](#derive-into-bytes) | fn |  |
| [`derive_into_bytes_struct`](#derive-into-bytes-struct) | fn |  |
| [`derive_into_bytes_enum`](#derive-into-bytes-enum) | fn |  |
| [`derive_into_bytes_union`](#derive-into-bytes-union) | fn |  |

## Functions

### `derive_into_bytes`

```rust
fn derive_into_bytes(ctx: &crate::util::Ctx, _top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_into_bytes_struct`

```rust
fn derive_into_bytes_struct(ctx: &crate::util::Ctx, strct: &syn::DataStruct) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_into_bytes_enum`

```rust
fn derive_into_bytes_enum(ctx: &crate::util::Ctx, enm: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_into_bytes_union`

```rust
fn derive_into_bytes_union(ctx: &crate::util::Ctx, unn: &syn::DataUnion) -> Result<proc_macro2::TokenStream, syn::Error>
```

