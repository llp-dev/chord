*[thiserror_impl](../index.md) / [valid](index.md)*

---

# Module `valid`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`check_non_field_attrs`](#check-non-field-attrs) | fn |  |
| [`check_field_attrs`](#check-field-attrs) | fn |  |
| [`contains_non_static_lifetime`](#contains-non-static-lifetime) | fn |  |

## Functions

### `check_non_field_attrs`

```rust
fn check_non_field_attrs(attrs: &crate::attr::Attrs<'_>) -> syn::Result<()>
```

### `check_field_attrs`

```rust
fn check_field_attrs(fields: &[crate::ast::Field<'_>]) -> syn::Result<()>
```

### `contains_non_static_lifetime`

```rust
fn contains_non_static_lifetime(ty: &syn::Type) -> bool
```

