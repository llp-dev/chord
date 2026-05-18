*[ruzstd](../../index.md) / [encoding](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`find_min_size`](#find-min-size) | fn | Returns the minimum number of bytes needed to represent this value, as either 1, 2, 4, or 8 bytes. |
| [`minify_val`](#minify-val) | fn | Returns the same value, but represented using the smallest number of bytes needed. |

## Functions

### `find_min_size`

```rust
fn find_min_size(val: u64) -> usize
```

Returns the minimum number of bytes needed to represent this value, as
either 1, 2, 4, or 8 bytes. A value of 0 will still return one byte.

Used for variable length fields like `Dictionary_ID` or `Frame_Content_Size`.

### `minify_val`

```rust
fn minify_val(val: u64) -> alloc::vec::Vec<u8>
```

Returns the same value, but represented using the smallest number of bytes needed.
Returned vector will be 1, 2, 4, or 8 bytes in length. Zero is represented as 1 byte.

Operates in **little-endian**.

