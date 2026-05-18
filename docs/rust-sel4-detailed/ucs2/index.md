# Crate `ucs2`

Utility functions for the UCS-2 character encoding.

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
- [Structs](#structs)
  - [`Ucs2CharFromUtf8`](#ucs2charfromutf8)
- [Enums](#enums)
  - [`Error`](#error)
- [Functions](#functions)
  - [`ucs2_from_utf8_at_offset`](#ucs2-from-utf8-at-offset)
  - [`encode`](#encode)
  - [`encode_with`](#encode-with)
  - [`decode_with`](#decode-with)
  - [`decode`](#decode)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Macros](#macros)
  - [`ucs2_cstr!`](#ucs2-cstr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`Ucs2CharFromUtf8`](#ucs2charfromutf8) | struct | Value returned by `ucs2_from_utf8_at_offset`. |
| [`Error`](#error) | enum | Possible errors returned by the API. |
| [`ucs2_from_utf8_at_offset`](#ucs2-from-utf8-at-offset) | fn | Get a UCS-2 character from a UTF-8 byte slice at the given offset. |
| [`encode`](#encode) | fn | Encodes an input UTF-8 string into a UCS-2 string. |
| [`encode_with`](#encode-with) | fn | Encode UTF-8 string to UCS-2 with a custom callback function. |
| [`decode_with`](#decode-with) | fn | Decode UCS-2 string to UTF-8 with a custom callback function. |
| [`decode`](#decode) | fn | Decode an input UCS-2 string into a UTF-8 string. |
| [`Result`](#result) | type |  |
| [`ucs2_cstr!`](#ucs2-cstr) | macro | Encode a string as UCS-2 with a trailing null character. |

## Modules

- [`macros`](macros/index.md)

## Structs

### `Ucs2CharFromUtf8`

```rust
struct Ucs2CharFromUtf8 {
    val: u16,
    num_bytes: u8,
}
```

Value returned by `ucs2_from_utf8_at_offset`.

#### Fields

- **`val`**: `u16`

  UCS-2 character.

- **`num_bytes`**: `u8`

  Number of bytes needed to encode the character in UTF-8.

## Enums

### `Error`

```rust
enum Error {
    BufferOverflow,
    MultiByte,
}
```

Possible errors returned by the API.

#### Variants

- **`BufferOverflow`**

  Not enough space left in the output buffer.

- **`MultiByte`**

  Input contained a character which cannot be represented in UCS-2.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl Hash for Error`

- <span id="error-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Error`

- <span id="error-ord-cmp"></span>`fn cmp(&self, other: &Error) -> cmp::Ordering` — [`Error`](#error)

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl PartialOrd for Error`

- <span id="error-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Error) -> option::Option<cmp::Ordering>` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

## Functions

### `ucs2_from_utf8_at_offset`

```rust
const unsafe fn ucs2_from_utf8_at_offset(bytes: &[u8], offset: usize) -> core::result::Result<Ucs2CharFromUtf8, Error>
```

Get a UCS-2 character from a UTF-8 byte slice at the given offset.

# Safety

The input `bytes` must be valid UTF-8.

### `encode`

```rust
fn encode(input: &str, buffer: &mut [u16]) -> core::result::Result<usize, Error>
```

Encodes an input UTF-8 string into a UCS-2 string.

The returned `usize` represents the length of the returned buffer,
measured in 2-byte characters.

### `encode_with`

```rust
fn encode_with<F>(input: &str, output: F) -> core::result::Result<(), Error>
where
    F: FnMut(u16) -> core::result::Result<(), Error>
```

Encode UTF-8 string to UCS-2 with a custom callback function.

`output` is a function which receives every encoded character.

### `decode_with`

```rust
fn decode_with<F>(input: &[u16], output: F) -> core::result::Result<usize, Error>
where
    F: FnMut(&[u8]) -> core::result::Result<(), Error>
```

Decode UCS-2 string to UTF-8 with a custom callback function.

`output` is a function which receives every decoded character.
Due to the nature of UCS-2, the function can receive an UTF-8 character
of up to three bytes, for every input character.

### `decode`

```rust
fn decode(input: &[u16], output: &mut [u8]) -> core::result::Result<usize, Error>
```

Decode an input UCS-2 string into a UTF-8 string.

The returned `usize` represents the length of the returned buffer,
in bytes. Due to the nature of UCS-2, the output buffer could end up with
three bytes for every character in the input buffer.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, Error>;
```

## Macros

### `ucs2_cstr!`

Encode a string as UCS-2 with a trailing null character.

The encoding is done at compile time, so the result can be used in a
`const` item. The type returned by the macro is a `[u16; N]` array;
to avoid having to specify what `N` is in a `const` item, take a
reference and store it as `&[u16]`.

# Example

```rust
use ucs2::ucs2_cstr;

const S: &[u16] = &ucs2_cstr!("abc");
assert_eq!(S, [97, 98, 99, 0]);
```

