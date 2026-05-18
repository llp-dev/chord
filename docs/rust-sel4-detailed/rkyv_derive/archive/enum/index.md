*[rkyv_derive](../../index.md) / [archive](../index.md) / [enum](index.md)*

---

# Module `enum`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impl_enum`](#impl-enum) | fn |  |
| [`generate_archived_type`](#generate-archived-type) | fn |  |
| [`generate_resolver_type`](#generate-resolver-type) | fn |  |
| [`generate_resolve_arms`](#generate-resolve-arms) | fn |  |
| [`generate_variant_structs`](#generate-variant-structs) | fn |  |
| [`generate_partial_eq_impl`](#generate-partial-eq-impl) | fn |  |
| [`generate_partial_ord_impl`](#generate-partial-ord-impl) | fn |  |
| [`generate_niching_impls`](#generate-niching-impls) | fn |  |

## Functions

### `impl_enum`

```rust
fn impl_enum(printing: &crate::archive::printing::Printing, generics: &syn::Generics, attributes: &crate::attributes::Attributes, data: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_archived_type`

```rust
fn generate_archived_type(printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, generics: &syn::Generics, data: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_resolver_type`

```rust
fn generate_resolver_type(printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, generics: &syn::Generics, data: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_resolve_arms`

```rust
fn generate_resolve_arms(printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, generics: &syn::Generics, data: &syn::DataEnum, name: &syn::Path) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_variant_structs`

```rust
fn generate_variant_structs(printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, generics: &syn::Generics, data: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_partial_eq_impl`

```rust
fn generate_partial_eq_impl(printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, generics: &syn::Generics, data: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_partial_ord_impl`

```rust
fn generate_partial_ord_impl(printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, generics: &syn::Generics, data: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `generate_niching_impls`

```rust
fn generate_niching_impls(printing: &crate::archive::printing::Printing, attributes: &crate::attributes::Attributes, generics: &syn::Generics, data: &syn::DataEnum) -> Result<proc_macro2::TokenStream, syn::Error>
```

