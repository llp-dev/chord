*[futures_util](../index.md) / [never](index.md)*

---

# Module `never`

This module contains the `Never` type.

Values of this type can never be created and will never exist.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Never`](#never) | type | A type with no possible values. |

## Type Aliases

### `Never`

```rust
type Never = core::convert::Infallible;
```

A type with no possible values.

This is used to indicate values which can never be created, such as the
error type of infallible futures.

This type is a stable equivalent to the `!` type from `std`.

This is currently an alias for `std::convert::Infallible`, but in
the future it may be an alias for [`!`][never](#never).
See ["Future compatibility" section of `std::convert::Infallible`][`infallible`](../../smallvec/index.md) for more.



