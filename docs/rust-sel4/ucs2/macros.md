**ucs2 > macros**

# Module: macros

## Contents

**Functions**

- [`str_num_ucs2_chars`](#str_num_ucs2_chars) - Count the number of UCS-2 characters in a string. Return an error if
- [`str_to_ucs2`](#str_to_ucs2) - Convert a `str` into a null-terminated UCS-2 character array.

---

## ucs2::macros::str_num_ucs2_chars

*Function*

Count the number of UCS-2 characters in a string. Return an error if
the string cannot be encoded in UCS-2.

```rust
fn str_num_ucs2_chars(s: &str) -> Result<usize, crate::Error>
```



## ucs2::macros::str_to_ucs2

*Function*

Convert a `str` into a null-terminated UCS-2 character array.

```rust
fn str_to_ucs2<const N>(s: &str) -> Result<[u16; N], crate::Error>
```



