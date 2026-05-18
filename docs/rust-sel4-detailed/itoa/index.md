# Crate `itoa`

[![github]](https://github.com/dtolnay/itoa)&ensp;[![crates-io]](https://crates.io/crates/itoa)&ensp;[![docs-rs]](https://docs.rs/itoa)



<br>

This crate provides a fast conversion of integer primitives to decimal
strings. The implementation comes straight from [libcore] but avoids the
performance penalty of going through [`core::fmt::Formatter`](../serde_json/ser/index.md).

See also [`zmij`](#zmij) for printing floating point primitives.


# Example

```rust
fn main() {
    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(128u64);
    assert_eq!(printed, "128");
}
```

# Performance

The [itoa-benchmark] compares this library and other Rust integer formatting
implementations across a range of integer sizes. The vertical axis in this
chart shows nanoseconds taken by a single execution of
`itoa::Buffer::new().format(value)` so a lower result indicates a faster
library.

![performance](https://raw.githubusercontent.com/dtolnay/itoa/master/itoa-benchmark.png)

## Contents

- [Modules](#modules)
  - [`u128_ext`](#u128-ext)
  - [`private`](#private)
- [Structs](#structs)
  - [`Buffer`](#buffer)
  - [`DecimalPairs`](#decimalpairs)
- [Traits](#traits)
  - [`Integer`](#integer)
  - [`Unsigned`](#unsigned)
- [Functions](#functions)
  - [`divmod100`](#divmod100)
  - [`slice_buffer_to_str`](#slice-buffer-to-str)
  - [`enc_16lsd`](#enc-16lsd)
  - [`div_rem_1e16`](#div-rem-1e16)
- [Macros](#macros)
  - [`impl_Integer!`](#impl-integer)
  - [`impl_Integer_size!`](#impl-integer-size)
  - [`impl_Unsigned!`](#impl-unsigned)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`u128_ext`](#u128-ext) | mod |  |
| [`private`](#private) | mod |  |
| [`Buffer`](#buffer) | struct | A correctly sized stack allocation for the formatted integer to be written into. |
| [`DecimalPairs`](#decimalpairs) | struct |  |
| [`Integer`](#integer) | trait | An integer that can be written into an [`itoa::Buffer`][Buffer]. |
| [`Unsigned`](#unsigned) | trait |  |
| [`divmod100`](#divmod100) | fn |  |
| [`slice_buffer_to_str`](#slice-buffer-to-str) | fn | This function converts a slice of ascii characters into a `&str` starting from `offset`. |
| [`enc_16lsd`](#enc-16lsd) | fn |  |
| [`div_rem_1e16`](#div-rem-1e16) | fn |  |
| [`impl_Integer!`](#impl-integer) | macro |  |
| [`impl_Integer_size!`](#impl-integer-size) | macro |  |
| [`impl_Unsigned!`](#impl-unsigned) | macro |  |

## Modules

- [`u128_ext`](u128_ext/index.md)
- [`private`](private/index.md)

## Structs

### `Buffer`

```rust
struct Buffer {
    bytes: [core::mem::MaybeUninit<u8>; 40],
}
```

A correctly sized stack allocation for the formatted integer to be written
into.

# Example

```rust
let mut buffer = itoa::Buffer::new();
let printed = buffer.format(1234);
assert_eq!(printed, "1234");
```

#### Implementations

- <span id="buffer-new"></span>`fn new() -> Buffer` — [`Buffer`](#buffer)

  This is a cheap operation; you don't need to worry about reusing buffers

  for efficiency.

- <span id="buffer-format"></span>`fn format<I: Integer>(&mut self, i: I) -> &str`

  Print an integer into this buffer and return a reference to its string

  representation within the buffer.

#### Trait Implementations

##### `impl Clone for Buffer`

- <span id="buffer-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for Buffer`

##### `impl Default for Buffer`

- <span id="buffer-default"></span>`fn default() -> Buffer` — [`Buffer`](#buffer)

### `DecimalPairs`

```rust
struct DecimalPairs([u8; 200]);
```

## Traits

### `Integer`

```rust
trait Integer: private::Sealed { ... }
```

An integer that can be written into an [`itoa::Buffer`][Buffer].

This trait is sealed and cannot be implemented for types outside of itoa.

#### Associated Constants

- `const MAX_STR_LEN: usize`

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `Unsigned`

```rust
trait Unsigned: Integer { ... }
```

#### Required Methods

- `fn fmt(self, buf: &mut <Self as >::Buffer) -> usize`

#### Implementors

- `u128`
- `u16`
- `u32`
- `u64`
- `u8`

## Functions

### `divmod100`

```rust
fn divmod100(value: u32) -> (u32, u32)
```

### `slice_buffer_to_str`

```rust
unsafe fn slice_buffer_to_str(buf: &[core::mem::MaybeUninit<u8>], offset: usize) -> &str
```

This function converts a slice of ascii characters into a `&str` starting
from `offset`.

# Safety

`buf` content starting from `offset` index MUST BE initialized and MUST BE
ascii characters.

### `enc_16lsd`

```rust
fn enc_16lsd<const OFFSET: usize>(buf: &mut [core::mem::MaybeUninit<u8>], n: u64)
```

### `div_rem_1e16`

```rust
fn div_rem_1e16(n: u128) -> (u128, u64)
```

## Macros

### `impl_Integer!`

### `impl_Integer_size!`

### `impl_Unsigned!`

