*[thiserror_impl](../index.md) / [expand](index.md)*

---

# Module `expand`

## Contents

- [Functions](#functions)
  - [`derive`](#derive)
  - [`try_expand`](#try-expand)
  - [`impl_struct`](#impl-struct)
  - [`impl_enum`](#impl-enum)
  - [`call_site_ident`](#call-site-ident)
  - [`fields_pat`](#fields-pat)
  - [`use_as_display`](#use-as-display)
  - [`from_initializer`](#from-initializer)
  - [`type_is_option`](#type-is-option)
  - [`unoptional_type`](#unoptional-type)
  - [`type_parameter_of_option`](#type-parameter-of-option)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`derive`](#derive) | fn |  |
| [`try_expand`](#try-expand) | fn |  |
| [`impl_struct`](#impl-struct) | fn |  |
| [`impl_enum`](#impl-enum) | fn |  |
| [`call_site_ident`](#call-site-ident) | fn |  |
| [`fields_pat`](#fields-pat) | fn |  |
| [`use_as_display`](#use-as-display) | fn |  |
| [`from_initializer`](#from-initializer) | fn |  |
| [`type_is_option`](#type-is-option) | fn |  |
| [`unoptional_type`](#unoptional-type) | fn |  |
| [`type_parameter_of_option`](#type-parameter-of-option) | fn |  |

## Functions

### `derive`

```rust
fn derive(input: &syn::DeriveInput) -> proc_macro2::TokenStream
```

### `try_expand`

```rust
fn try_expand(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream>
```

### `impl_struct`

```rust
fn impl_struct(input: crate::ast::Struct<'_>) -> proc_macro2::TokenStream
```

### `impl_enum`

```rust
fn impl_enum(input: crate::ast::Enum<'_>) -> proc_macro2::TokenStream
```

### `call_site_ident`

```rust
fn call_site_ident(ident: &proc_macro2::Ident) -> proc_macro2::Ident
```

### `fields_pat`

```rust
fn fields_pat(fields: &[crate::ast::Field<'_>]) -> proc_macro2::TokenStream
```

### `use_as_display`

```rust
fn use_as_display(needs_as_display: bool) -> Option<proc_macro2::TokenStream>
```

### `from_initializer`

```rust
fn from_initializer(from_field: &crate::ast::Field<'_>, backtrace_field: Option<&crate::ast::Field<'_>>, source_var: &proc_macro2::Ident) -> proc_macro2::TokenStream
```

### `type_is_option`

```rust
fn type_is_option(ty: &syn::Type) -> bool
```

### `unoptional_type`

```rust
fn unoptional_type(ty: &syn::Type) -> proc_macro2::TokenStream
```

### `type_parameter_of_option`

```rust
fn type_parameter_of_option(ty: &syn::Type) -> Option<&syn::Type>
```

