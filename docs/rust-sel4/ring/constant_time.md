**ring > constant_time**

# Module: constant_time

## Contents

**Functions**

- [`verify_slices_are_equal`](#verify_slices_are_equal) - Returns `Ok(())` if `a == b` and `Err(error::Unspecified)` otherwise.

---

## ring::constant_time::verify_slices_are_equal

*Function*

Returns `Ok(())` if `a == b` and `Err(error::Unspecified)` otherwise.
The comparison of `a` and `b` is done in constant time with respect to the
contents of each, but NOT in constant time with respect to the lengths of
`a` and `b`.

```rust
fn verify_slices_are_equal(a: &[u8], b: &[u8]) -> Result<(), error::Unspecified>
```



