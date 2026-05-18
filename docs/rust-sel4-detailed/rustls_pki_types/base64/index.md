*[rustls_pki_types](../index.md) / [base64](index.md)*

---

# Module `base64`

## Contents

- [Structs](#structs)
  - [`CodePoint`](#codepoint)
- [Enums](#enums)
  - [`Error`](#error)
- [Functions](#functions)
  - [`decode_secret`](#decode-secret)
  - [`decode_public`](#decode-public)
  - [`decoded_length`](#decoded-length)
  - [`decode`](#decode)
  - [`u8_in_range`](#u8-in-range)
  - [`u8_less_than`](#u8-less-than)
  - [`u8_equals`](#u8-equals)
  - [`u8_nonzero`](#u8-nonzero)
  - [`u8_broadcast8`](#u8-broadcast8)
  - [`u8_broadcast16`](#u8-broadcast16)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CodePoint`](#codepoint) | struct |  |
| [`Error`](#error) | enum |  |
| [`decode_secret`](#decode-secret) | fn | Decode base64 `input`, writing the result into `output`. |
| [`decode_public`](#decode-public) | fn | Decode base64 `input`, writing the result into `output`. |
| [`decoded_length`](#decoded-length) | fn | Provide an upper limit on how much space could be required to decode a base64 encoding of len `base64_len`. |
| [`decode`](#decode) | fn |  |
| [`u8_in_range`](#u8-in-range) | fn | Returns 0xff if `a` in `lo..=hi`. |
| [`u8_less_than`](#u8-less-than) | fn | Returns 0xff if a < b, 0 otherwise. |
| [`u8_equals`](#u8-equals) | fn | Returns 0xff if a == b, 0 otherwise. |
| [`u8_nonzero`](#u8-nonzero) | fn | Returns 0xff if a != 0, 0 otherwise. |
| [`u8_broadcast8`](#u8-broadcast8) | fn | Broadcasts the top bit of `x` |
| [`u8_broadcast16`](#u8-broadcast16) | fn | Broadcasts the top bit of `x` |

## Structs

### `CodePoint`

```rust
struct CodePoint(u8);
```

#### Implementations

- <span id="codepoint-const-whitespace"></span>`const WHITESPACE: Self`

- <span id="codepoint-const-pad"></span>`const PAD: Self`

- <span id="codepoint-const-invalid"></span>`const INVALID: Self`

#### Trait Implementations

##### `impl Clone for CodePoint`

- <span id="codepoint-clone"></span>`fn clone(&self) -> CodePoint` — [`CodePoint`](#codepoint)

##### `impl Copy for CodePoint`

##### `impl Debug for CodePoint`

- <span id="codepoint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CodePoint`

##### `impl PartialEq for CodePoint`

- <span id="codepoint-partialeq-eq"></span>`fn eq(&self, other: &CodePoint) -> bool` — [`CodePoint`](#codepoint)

##### `impl StructuralPartialEq for CodePoint`

## Enums

### `Error`

```rust
enum Error {
    InvalidCharacter(u8),
    PrematurePadding,
    InvalidTrailingPadding,
    InsufficientOutputSpace,
}
```

#### Variants

- **`InvalidCharacter`**

  Given character is not valid in base64 alphabet.

- **`PrematurePadding`**

  A padding character (`=`) appeared outside the final
  block of 4 characters.

- **`InvalidTrailingPadding`**

  The padding characters at the end of the input were invalid.

- **`InsufficientOutputSpace`**

  Not enough space in output buffer.
  
  Use `decoded_length` to get an upper bound.

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

## Functions

### `decode_secret`

```rust
fn decode_secret<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a [u8], Error>
```

Decode base64 `input`, writing the result into `output`.

`input` is treated as secret, so efforts are made to avoid
leaking its value via side channels, such as timing,
memory accesses, and execution trace.

The following is deemed non-secret information:

- Appearance of whitespace in `input`
- Erroneous characters in `input` (indeed, the first illegal
  character is quoted in the error type)
- The length of `input`
- The length of `output`

Returns the prefix of `output` that was written to.

### `decode_public`

```rust
fn decode_public<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a [u8], Error>
```

Decode base64 `input`, writing the result into `output`.

`input` is treated as public information, so its value may
be leaked via side channels.

Returns the prefix of `output` that was written to.

### `decoded_length`

```rust
const fn decoded_length(base64_len: usize) -> usize
```

Provide an upper limit on how much space could be required
to decode a base64 encoding of len `base64_len`.

### `decode`

```rust
fn decode<'a>(input: &[u8], output: &'a mut [u8], decode_byte: impl Fn(u8) -> CodePoint) -> Result<&'a [u8], Error>
```

### `u8_in_range`

```rust
fn u8_in_range(a: u8, lo: u8, hi: u8) -> u8
```

Returns 0xff if `a` in `lo..=hi`.

lo..=hi must not be 0..=255.  Callers in this file have constant
`lo` and `hi`, and this function is private to this file.

### `u8_less_than`

```rust
fn u8_less_than(a: u8, b: u8) -> u8
```

Returns 0xff if a < b, 0 otherwise.

### `u8_equals`

```rust
const fn u8_equals(a: u8, b: u8) -> u8
```

Returns 0xff if a == b, 0 otherwise.

### `u8_nonzero`

```rust
const fn u8_nonzero(x: u8) -> u8
```

Returns 0xff if a != 0, 0 otherwise.

### `u8_broadcast8`

```rust
const fn u8_broadcast8(x: u8) -> u8
```

Broadcasts the top bit of `x`

In other words, if the top bit of `x` is set,
returns 0xff else 0x00.

### `u8_broadcast16`

```rust
const fn u8_broadcast16(x: u16) -> u8
```

Broadcasts the top bit of `x`

In other words, if the top bit of `x` is set,
returns 0xff else 0x00.

