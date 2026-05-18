*[zerocopy_derive](../../index.md) / [derive](../index.md) / [from_bytes](index.md)*

---

# Module `from_bytes`

## Contents

- [Functions](#functions)
  - [`find_zero_variant`](#find-zero-variant)
  - [`derive_from_zeros`](#derive-from-zeros)
  - [`derive_from_bytes`](#derive-from-bytes)
  - [`derive_from_zeros_struct`](#derive-from-zeros-struct)
  - [`derive_from_zeros_enum`](#derive-from-zeros-enum)
  - [`derive_from_zeros_union`](#derive-from-zeros-union)
  - [`derive_from_bytes_struct`](#derive-from-bytes-struct)
  - [`derive_from_bytes_enum`](#derive-from-bytes-enum)
  - [`derive_from_bytes_union`](#derive-from-bytes-union)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`find_zero_variant`](#find-zero-variant) | fn | Returns `Ok(index)` if variant `index` of the enum has a discriminant of zero. |
| [`derive_from_zeros`](#derive-from-zeros) | fn |  |
| [`derive_from_bytes`](#derive-from-bytes) | fn |  |
| [`derive_from_zeros_struct`](#derive-from-zeros-struct) | fn |  |
| [`derive_from_zeros_enum`](#derive-from-zeros-enum) | fn |  |
| [`derive_from_zeros_union`](#derive-from-zeros-union) | fn |  |
| [`derive_from_bytes_struct`](#derive-from-bytes-struct) | fn |  |
| [`derive_from_bytes_enum`](#derive-from-bytes-enum) | fn |  |
| [`derive_from_bytes_union`](#derive-from-bytes-union) | fn |  |

## Functions

### `find_zero_variant`

```rust
fn find_zero_variant(enm: &syn::DataEnum) -> Result<usize, bool>
```

Returns `Ok(index)` if variant `index` of the enum has a discriminant of
zero. If `Err(bool)` is returned, the boolean is true if the enum has
unknown discriminants (e.g. discriminants set to const expressions which we
can't evaluate in a proc macro). If the enum has unknown discriminants, then
it might have a zero variant that we just can't detect.

### `derive_from_zeros`

```rust
fn derive_from_zeros(ctx: &crate::util::Ctx, top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_from_bytes`

```rust
fn derive_from_bytes(ctx: &crate::util::Ctx, top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_from_zeros_struct`

```rust
fn derive_from_zeros_struct(ctx: &crate::util::Ctx, strct: &syn::DataStruct) -> proc_macro2::TokenStream
```

### `derive_from_zeros_enum`

```rust
fn derive_from_zeros_enum(ctx: &crate::util::Ctx, enm: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_from_zeros_union`

```rust
fn derive_from_zeros_union(ctx: &crate::util::Ctx, unn: &syn::DataUnion) -> proc_macro2::TokenStream
```

### `derive_from_bytes_struct`

```rust
fn derive_from_bytes_struct(ctx: &crate::util::Ctx, strct: &syn::DataStruct) -> proc_macro2::TokenStream
```

### `derive_from_bytes_enum`

```rust
fn derive_from_bytes_enum(ctx: &crate::util::Ctx, enm: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_from_bytes_union`

```rust
fn derive_from_bytes_union(ctx: &crate::util::Ctx, unn: &syn::DataUnion) -> proc_macro2::TokenStream
```

