*[rkyv_derive](../index.md) / [serialize](index.md)*

---

# Module `serialize`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive`](#derive) | fn |  |
| [`derive_serialize_impl`](#derive-serialize-impl) | fn |  |
| [`generate_serialize_body`](#generate-serialize-body) | fn |  |

## Functions

### `derive`

```rust
fn derive(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `derive_serialize_impl`

```rust
fn derive_serialize_impl(input: syn::DeriveInput, attributes: &crate::attributes::Attributes) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_serialize_body`

```rust
fn generate_serialize_body(input: &syn::DeriveInput, attributes: &crate::attributes::Attributes, serialize_where: &mut syn::WhereClause, rkyv_path: &syn::Path, resolver: syn::Ident, name: syn::Path) -> Result<proc_macro2::TokenStream, syn::Error>
```

