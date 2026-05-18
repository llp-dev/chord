*[rkyv_derive](../../index.md) / [archive](../index.md) / [struct](index.md)*

---

# Module `struct`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impl_struct`](#impl-struct) | fn |  |
| [`generate_resolve_statements`](#generate-resolve-statements) | fn |  |
| [`generate_archived_type`](#generate-archived-type) | fn |  |
| [`generate_resolver_type`](#generate-resolver-type) | fn |  |
| [`generate_partial_eq_impl`](#generate-partial-eq-impl) | fn |  |
| [`generate_partial_ord_impl`](#generate-partial-ord-impl) | fn |  |
| [`generate_copy_optimization`](#generate-copy-optimization) | fn |  |
| [`generate_niching_impls`](#generate-niching-impls) | fn |  |

## Functions

### `impl_struct`

```rust
fn impl_struct(printing: &crate::archive::printing::Printing, generics: &syn::Generics, attributes: &crate::attributes::Attributes, fields: &syn::Fields) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_resolve_statements`

```rust
fn generate_resolve_statements(printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, fields: &syn::Fields, this: proc_macro2::Ident) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_archived_type`

```rust
fn generate_archived_type(printing: &crate::archive::printing::Printing, generics: &syn::Generics, attributes: &crate::attributes::Attributes, fields: &syn::Fields) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_resolver_type`

```rust
fn generate_resolver_type(printing: &crate::archive::printing::Printing, generics: &syn::Generics, attributes: &crate::attributes::Attributes, fields: &syn::Fields) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_partial_eq_impl`

```rust
fn generate_partial_eq_impl(printing: &crate::archive::printing::Printing, generics: &syn::Generics, attributes: &crate::attributes::Attributes, fields: &syn::Fields) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_partial_ord_impl`

```rust
fn generate_partial_ord_impl(printing: &crate::archive::printing::Printing, generics: &syn::Generics, attributes: &crate::attributes::Attributes, fields: &syn::Fields) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_copy_optimization`

```rust
fn generate_copy_optimization(printing: &crate::archive::printing::Printing, generics: &syn::Generics, attributes: &crate::attributes::Attributes, fields: &syn::Fields) -> Result<Option<proc_macro2::TokenStream>, syn::Error>
```

### `generate_niching_impls`

```rust
fn generate_niching_impls(printing: &crate::archive::printing::Printing, generics: &syn::Generics, attributes: &crate::attributes::Attributes, fields: &syn::Fields) -> Result<proc_macro2::TokenStream, syn::Error>
```

