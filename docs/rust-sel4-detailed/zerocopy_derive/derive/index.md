*[zerocopy_derive](../index.md) / [derive](index.md)*

---

# Module `derive`

## Contents

- [Modules](#modules)
  - [`from_bytes`](#from-bytes)
  - [`into_bytes`](#into-bytes)
  - [`known_layout`](#known-layout)
  - [`try_from_bytes`](#try-from-bytes)
  - [`unaligned`](#unaligned)
- [Functions](#functions)
  - [`derive_immutable`](#derive-immutable)
  - [`derive_hash`](#derive-hash)
  - [`derive_eq`](#derive-eq)
  - [`derive_split_at`](#derive-split-at)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`from_bytes`](#from-bytes) | mod |  |
| [`into_bytes`](#into-bytes) | mod |  |
| [`known_layout`](#known-layout) | mod |  |
| [`try_from_bytes`](#try-from-bytes) | mod |  |
| [`unaligned`](#unaligned) | mod |  |
| [`derive_immutable`](#derive-immutable) | fn |  |
| [`derive_hash`](#derive-hash) | fn |  |
| [`derive_eq`](#derive-eq) | fn |  |
| [`derive_split_at`](#derive-split-at) | fn |  |

## Modules

- [`from_bytes`](from_bytes/index.md)
- [`into_bytes`](into_bytes/index.md)
- [`known_layout`](known_layout/index.md)
- [`try_from_bytes`](try_from_bytes/index.md)
- [`unaligned`](unaligned/index.md)

## Functions

### `derive_immutable`

```rust
fn derive_immutable(ctx: &crate::util::Ctx, _top_level: crate::util::Trait) -> proc_macro2::TokenStream
```

### `derive_hash`

```rust
fn derive_hash(ctx: &crate::util::Ctx, _top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_eq`

```rust
fn derive_eq(ctx: &crate::util::Ctx, _top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_split_at`

```rust
fn derive_split_at(ctx: &crate::util::Ctx, _top_level: crate::util::Trait) -> Result<proc_macro2::TokenStream, syn::Error>
```

