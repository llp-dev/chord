*[rkyv_derive](../index.md) / [deserialize](index.md)*

---

# Module `deserialize`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive`](#derive) | fn |  |
| [`derive_deserialize_impl`](#derive-deserialize-impl) | fn |  |
| [`generate_deserialize_body`](#generate-deserialize-body) | fn |  |

## Functions

### `derive`

```rust
fn derive(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_deserialize_impl`

```rust
fn derive_deserialize_impl(input: syn::DeriveInput, attributes: &crate::attributes::Attributes) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_deserialize_body`

```rust
fn generate_deserialize_body(input: &syn::DeriveInput, attributes: &crate::attributes::Attributes, deserialize_where: &mut syn::WhereClause, rkyv_path: &syn::Path, self_type: syn::Ident, return_type: &syn::Ident) -> Result<proc_macro2::TokenStream, syn::Error>
```

