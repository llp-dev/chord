# Crate `bytecheck_derive`

Procedural macros for bytecheck.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`attributes`](#attributes) | mod |  |
| [`repr`](#repr) | mod |  |
| [`util`](#util) | mod |  |
| [`derive_check_bytes`](#derive-check-bytes) | fn |  |
| [`check_arm_named_field`](#check-arm-named-field) | fn |  |
| [`check_arm_unnamed_field`](#check-arm-unnamed-field) | fn |  |

## Modules

- [`attributes`](attributes/index.md)
- [`repr`](repr/index.md)
- [`util`](util/index.md)

## Functions

### `derive_check_bytes`

```rust
fn derive_check_bytes(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error>
```

### `check_arm_named_field`

```rust
fn check_arm_named_field(f: &syn::Field, crate_path: &syn::Path, name: &syn::Ident, variant: &syn::Ident) -> proc_macro2::TokenStream
```

### `check_arm_unnamed_field`

```rust
fn check_arm_unnamed_field(i: usize, f: &syn::Field, crate_path: &syn::Path, name: &syn::Ident, variant: &syn::Ident) -> proc_macro2::TokenStream
```

