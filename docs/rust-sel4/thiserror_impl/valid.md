**thiserror_impl > valid**

# Module: valid

## Contents

**Functions**

- [`check_field_attrs`](#check_field_attrs)
- [`check_non_field_attrs`](#check_non_field_attrs)
- [`contains_non_static_lifetime`](#contains_non_static_lifetime)

---

## thiserror_impl::valid::check_field_attrs

*Function*

```rust
fn check_field_attrs(fields: &[crate::ast::Field]) -> syn::Result<()>
```



## thiserror_impl::valid::check_non_field_attrs

*Function*

```rust
fn check_non_field_attrs(attrs: &crate::attr::Attrs) -> syn::Result<()>
```



## thiserror_impl::valid::contains_non_static_lifetime

*Function*

```rust
fn contains_non_static_lifetime(ty: &syn::Type) -> bool
```



