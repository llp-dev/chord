*[ucs2](../index.md) / [macros](index.md)*

---

# Module `macros`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`str_num_ucs2_chars`](#str-num-ucs2-chars) | fn | Count the number of UCS-2 characters in a string. |
| [`str_to_ucs2`](#str-to-ucs2) | fn | Convert a `str` into a null-terminated UCS-2 character array. |

## Functions

### `str_num_ucs2_chars`

```rust
const fn str_num_ucs2_chars(s: &str) -> Result<usize, crate::Error>
```

Count the number of UCS-2 characters in a string. Return an error if
the string cannot be encoded in UCS-2.

### `str_to_ucs2`

```rust
const fn str_to_ucs2<const N: usize>(s: &str) -> Result<[u16; N], crate::Error>
```

Convert a `str` into a null-terminated UCS-2 character array.

