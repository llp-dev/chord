*[syn](../../index.md) / [lit](../index.md) / [value](index.md)*

---

# Module `value`

## Contents

- [Functions](#functions)
  - [`byte`](#byte)
  - [`next_chr`](#next-chr)
  - [`parse_lit_str`](#parse-lit-str)
  - [`parse_lit_str_cooked`](#parse-lit-str-cooked)
  - [`parse_lit_str_raw`](#parse-lit-str-raw)
  - [`parse_lit_byte_str`](#parse-lit-byte-str)
  - [`parse_lit_byte_str_cooked`](#parse-lit-byte-str-cooked)
  - [`parse_lit_byte_str_raw`](#parse-lit-byte-str-raw)
  - [`parse_lit_c_str`](#parse-lit-c-str)
  - [`parse_lit_c_str_cooked`](#parse-lit-c-str-cooked)
  - [`parse_lit_c_str_raw`](#parse-lit-c-str-raw)
  - [`parse_lit_byte`](#parse-lit-byte)
  - [`parse_lit_char`](#parse-lit-char)
  - [`backslash_x`](#backslash-x)
  - [`backslash_u`](#backslash-u)
  - [`parse_lit_int`](#parse-lit-int)
  - [`parse_lit_float`](#parse-lit-float)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`byte`](#byte) | fn | Get the byte at offset idx, or a default of `b'\0'` if we're looking past the end of the input buffer. |
| [`next_chr`](#next-chr) | fn |  |
| [`parse_lit_str`](#parse-lit-str) | fn |  |
| [`parse_lit_str_cooked`](#parse-lit-str-cooked) | fn |  |
| [`parse_lit_str_raw`](#parse-lit-str-raw) | fn |  |
| [`parse_lit_byte_str`](#parse-lit-byte-str) | fn |  |
| [`parse_lit_byte_str_cooked`](#parse-lit-byte-str-cooked) | fn |  |
| [`parse_lit_byte_str_raw`](#parse-lit-byte-str-raw) | fn |  |
| [`parse_lit_c_str`](#parse-lit-c-str) | fn |  |
| [`parse_lit_c_str_cooked`](#parse-lit-c-str-cooked) | fn |  |
| [`parse_lit_c_str_raw`](#parse-lit-c-str-raw) | fn |  |
| [`parse_lit_byte`](#parse-lit-byte) | fn |  |
| [`parse_lit_char`](#parse-lit-char) | fn |  |
| [`backslash_x`](#backslash-x) | fn |  |
| [`backslash_u`](#backslash-u) | fn |  |
| [`parse_lit_int`](#parse-lit-int) | fn |  |
| [`parse_lit_float`](#parse-lit-float) | fn |  |

## Functions

### `byte`

```rust
fn byte<S: AsRef<[u8]> + ?Sized>(s: &S, idx: usize) -> u8
```

Get the byte at offset idx, or a default of `b'\0'` if we're looking
past the end of the input buffer.

### `next_chr`

```rust
fn next_chr(s: &str) -> char
```

### `parse_lit_str`

```rust
fn parse_lit_str(s: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```

### `parse_lit_str_cooked`

```rust
fn parse_lit_str_cooked(s: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```

### `parse_lit_str_raw`

```rust
fn parse_lit_str_raw(s: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```

### `parse_lit_byte_str`

```rust
fn parse_lit_byte_str(s: &str) -> Option<(alloc::vec::Vec<u8>, alloc::boxed::Box<str>)>
```

### `parse_lit_byte_str_cooked`

```rust
fn parse_lit_byte_str_cooked(s: &str) -> Option<(alloc::vec::Vec<u8>, alloc::boxed::Box<str>)>
```

### `parse_lit_byte_str_raw`

```rust
fn parse_lit_byte_str_raw(s: &str) -> Option<(alloc::vec::Vec<u8>, alloc::boxed::Box<str>)>
```

### `parse_lit_c_str`

```rust
fn parse_lit_c_str(s: &str) -> Option<(alloc::ffi::CString, alloc::boxed::Box<str>)>
```

### `parse_lit_c_str_cooked`

```rust
fn parse_lit_c_str_cooked(s: &str) -> Option<(alloc::ffi::CString, alloc::boxed::Box<str>)>
```

### `parse_lit_c_str_raw`

```rust
fn parse_lit_c_str_raw(s: &str) -> Option<(alloc::ffi::CString, alloc::boxed::Box<str>)>
```

### `parse_lit_byte`

```rust
fn parse_lit_byte(s: &str) -> Option<(u8, alloc::boxed::Box<str>)>
```

### `parse_lit_char`

```rust
fn parse_lit_char(s: &str) -> Option<(char, alloc::boxed::Box<str>)>
```

### `backslash_x`

```rust
fn backslash_x<S>(s: &S) -> Option<(u8, &S)>
where
    S: Index<core::ops::RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized
```

### `backslash_u`

```rust
fn backslash_u<S>(s: &S) -> Option<(char, &S)>
where
    S: Index<core::ops::RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized
```

### `parse_lit_int`

```rust
fn parse_lit_int(s: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```

### `parse_lit_float`

```rust
fn parse_lit_float(input: &str) -> Option<(alloc::boxed::Box<str>, alloc::boxed::Box<str>)>
```

