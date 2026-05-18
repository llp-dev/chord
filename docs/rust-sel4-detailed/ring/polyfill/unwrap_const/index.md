*[ring](../../index.md) / [polyfill](../index.md) / [unwrap_const](index.md)*

---

# Module `unwrap_const`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unwrap_const`](#unwrap-const) | fn | Polyfill for `Option::unwrap()` as a const fn; feature `const_option`. |

## Functions

### `unwrap_const`

```rust
const fn unwrap_const<T>(x: Option<T>) -> T
where
    T: Copy
```

Polyfill for `Option::unwrap()` as a const fn; feature `const_option`.
https://github.com/rust-lang/rust/issues/67441.
TODO(MSRV): Replace this with `x.unwrap()`.

`T: Copy` avoids "constant functions cannot evaluate destructors."

