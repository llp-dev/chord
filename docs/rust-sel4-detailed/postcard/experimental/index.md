*[postcard](../index.md) / [experimental](index.md)*

---

# Module `experimental`

# Experimental Postcard Features

Items inside this module require various feature flags, and are not
subject to SemVer stability. Items may be removed or deprecated at
any point.

## Derive

The `experimental-derive` feature enables one experimental feature:

* Max size calculation

### Max Size Calculation

This features enables calculation of the Max serialized size of a message as
an associated `usize` constant called `POSTCARD_MAX_SIZE`. It also provides a
`#[derive(MaxSize)]` macro that can be used for calculating user types.

This is useful for determining the maximum buffer size needed when receiving
or sending a message that has been serialized.

NOTE: This only covers the size of "plain" flavored messages, e.g. not with COBS
or any other Flavors applied. The overhead for these flavors must be calculated
separately.

Please report any missing types, or any incorrectly calculated values.

### Message Schema Generation

This now lives in the `postcard-schema` crate.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serialized_size`](#serialized-size) | fn |  |

## Functions

### `serialized_size`

```rust
fn serialized_size<T>(value: &T) -> crate::error::Result<usize>
where
    T: Serialize + ?Sized
```

Compute the size of the postcard serialization of `T`.

