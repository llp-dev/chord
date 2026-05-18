*[ring](../index.md) / [constant_time](index.md)*

---

# Module `constant_time`

Constant-time operations.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CRYPTO_memcmp`](#crypto-memcmp) | fn |  |
| [`verify_slices_are_equal`](#verify-slices-are-equal) | fn | Returns `Ok(())` if `a == b` and `Err(error::Unspecified)` otherwise. |

## Functions

### `CRYPTO_memcmp`

```rust
unsafe fn CRYPTO_memcmp(a: *const u8, b: *const u8, len: usize) -> i32
```

### `verify_slices_are_equal`

```rust
fn verify_slices_are_equal(a: &[u8], b: &[u8]) -> Result<(), error::Unspecified>
```

Returns `Ok(())` if `a == b` and `Err(error::Unspecified)` otherwise.
The comparison of `a` and `b` is done in constant time with respect to the
contents of each, but NOT in constant time with respect to the lengths of
`a` and `b`.

