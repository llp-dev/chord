# Crate `bytemuck_derive`

Derive macros for [bytemuck](https://docs.rs/bytemuck) traits.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`traits`](#traits) | mod |  |
| [`derive_marker_trait`](#derive-marker-trait) | fn | Basic wrapper for error handling |
| [`find_and_parse_helper_attributes`](#find-and-parse-helper-attributes) | fn | Find `#[name(key = "value")]` helper attributes on the struct, and return their `"value"`s parsed with `parser`. |
| [`derive_marker_trait_inner`](#derive-marker-trait-inner) | fn |  |
| [`add_trait_marker`](#add-trait-marker) | fn | Add a trait marker to the generics if it is not already present |

## Modules

- [`traits`](traits/index.md)

## Functions

### `derive_marker_trait`

```rust
fn derive_marker_trait<Trait: Derivable>(input: syn::DeriveInput) -> proc_macro2::TokenStream
```

Basic wrapper for error handling

### `find_and_parse_helper_attributes`

```rust
fn find_and_parse_helper_attributes<P: syn::parse::Parser + Copy>(attributes: &[syn::Attribute], name: &str, key: &str, parser: P, example_value: &str, invalid_value_msg: &str) -> syn::Result<Vec<<P as >::Output>>
```

Find `#[name(key = "value")]` helper attributes on the struct, and return
their `"value"`s parsed with `parser`.

Returns an error if any attributes with the given `name` do not match the
expected format. Returns `Ok([])` if no attributes with `name` are found.

### `derive_marker_trait_inner`

```rust
fn derive_marker_trait_inner<Trait: Derivable>(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream>
```

### `add_trait_marker`

```rust
fn add_trait_marker(generics: &mut syn::Generics, trait_name: &syn::Path)
```

Add a trait marker to the generics if it is not already present

