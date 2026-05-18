*[rkyv_derive](../index.md) / [archive](index.md)*

---

# Module `archive`

## Contents

- [Modules](#modules)
  - [`enum`](#enum)
  - [`printing`](#printing)
  - [`struct`](#struct)
- [Functions](#functions)
  - [`derive`](#derive)
  - [`archived_doc`](#archived-doc)
  - [`resolver_doc`](#resolver-doc)
  - [`variant_doc`](#variant-doc)
  - [`resolver_variant_doc`](#resolver-variant-doc)
  - [`derive_archive_impl`](#derive-archive-impl)
  - [`impl_auto_trait`](#impl-auto-trait)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`enum`](#enum) | mod |  |
| [`printing`](#printing) | mod |  |
| [`struct`](#struct) | mod |  |
| [`derive`](#derive) | fn |  |
| [`archived_doc`](#archived-doc) | fn |  |
| [`resolver_doc`](#resolver-doc) | fn |  |
| [`variant_doc`](#variant-doc) | fn |  |
| [`resolver_variant_doc`](#resolver-variant-doc) | fn |  |
| [`derive_archive_impl`](#derive-archive-impl) | fn |  |
| [`impl_auto_trait`](#impl-auto-trait) | fn |  |

## Modules

- [`enum`](enum/index.md)
- [`printing`](printing/index.md)
- [`struct`](struct/index.md)

## Functions

### `derive`

```rust
fn derive(input: &mut syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `archived_doc`

```rust
fn archived_doc(name: &syn::Ident) -> String
```

### `resolver_doc`

```rust
fn resolver_doc(name: &syn::Ident) -> String
```

### `variant_doc`

```rust
fn variant_doc(name: &syn::Ident, variant_name: &syn::Ident) -> String
```

### `resolver_variant_doc`

```rust
fn resolver_variant_doc(name: &syn::Ident, variant_name: &syn::Ident) -> String
```

### `derive_archive_impl`

```rust
fn derive_archive_impl(input: &mut syn::DeriveInput, attributes: &crate::attributes::Attributes) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `impl_auto_trait`

```rust
fn impl_auto_trait(input: &syn::DeriveInput, printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, trait_name: &str) -> Result<proc_macro2::TokenStream, syn::Error>
```

