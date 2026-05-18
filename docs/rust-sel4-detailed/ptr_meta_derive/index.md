# Crate `ptr_meta_derive`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`attributes`](#attributes) | mod |  |
| [`derive_pointee_impl`](#derive-pointee-impl) | fn |  |
| [`pointee_impl`](#pointee-impl) | fn |  |

## Modules

- [`attributes`](attributes/index.md)

## Functions

### `derive_pointee_impl`

```rust
fn derive_pointee_impl(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `pointee_impl`

```rust
fn pointee_impl(attributes: self::attributes::Attributes, item: syn::ItemTrait) -> Result<proc_macro2::TokenStream, syn::Error>
```

