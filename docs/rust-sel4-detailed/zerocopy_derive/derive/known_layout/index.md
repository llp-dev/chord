*[zerocopy_derive](../../index.md) / [derive](../index.md) / [known_layout](index.md)*

---

# Module `known_layout`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive_known_layout_for_repr_c_struct`](#derive-known-layout-for-repr-c-struct) | fn |  |
| [`derive`](#derive) | fn |  |

## Functions

### `derive_known_layout_for_repr_c_struct`

```rust
fn derive_known_layout_for_repr_c_struct<'a>(ctx: &'a crate::util::Ctx, repr: &Repr<core::convert::Infallible, core::num::NonZeroU32>, fields: &[(&'a syn::Visibility, proc_macro2::TokenStream, &'a syn::Type)]) -> Option<(crate::util::SelfBounds<'a>, proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)>
```

### `derive`

```rust
fn derive(ctx: &crate::util::Ctx, _top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

