*[zerocopy_derive](../../index.md) / [derive](../index.md) / [unaligned](index.md)*

---

# Module `unaligned`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_unaligned`](#derive-unaligned) | fn |  |
| [`derive_unaligned_struct`](#derive-unaligned-struct) | fn | A struct is `Unaligned` if: - `repr(align)` is no more than 1 and either - `repr(C)` or `repr(transparent)` and - all fields `Unaligned` - `repr(packed)` |
| [`derive_unaligned_enum`](#derive-unaligned-enum) | fn | An enum is `Unaligned` if: - No `repr(align(N > 1))` - `repr(u8)` or `repr(i8)` |
| [`derive_unaligned_union`](#derive-unaligned-union) | fn | Like structs, a union is `Unaligned` if: - `repr(align)` is no more than 1 and either - `repr(C)` or `repr(transparent)` and - all fields `Unaligned` - `repr(packed)` |

## Functions

### `derive_unaligned`

```rust
fn derive_unaligned(ctx: &crate::util::Ctx, _top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_unaligned_struct`

```rust
fn derive_unaligned_struct(ctx: &crate::util::Ctx, strct: &syn::DataStruct) -> Result<proc_macro2::TokenStream, syn::Error>
```

A struct is `Unaligned` if:
- `repr(align)` is no more than 1 and either
  - `repr(C)` or `repr(transparent)` and
    - all fields `Unaligned`
  - `repr(packed)`

### `derive_unaligned_enum`

```rust
fn derive_unaligned_enum(ctx: &crate::util::Ctx, enm: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

An enum is `Unaligned` if:
- No `repr(align(N > 1))`
- `repr(u8)` or `repr(i8)`

### `derive_unaligned_union`

```rust
fn derive_unaligned_union(ctx: &crate::util::Ctx, unn: &syn::DataUnion) -> Result<proc_macro2::TokenStream, syn::Error>
```

Like structs, a union is `Unaligned` if:
- `repr(align)` is no more than 1 and either
  - `repr(C)` or `repr(transparent)` and
    - all fields `Unaligned`
  - `repr(packed)`

