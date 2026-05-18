**ucs2**

# Module: ucs2

## Contents

**Modules**

- [`macros`](#macros)

**Macros**

- [`ucs2_cstr`](#ucs2_cstr) - Encode a string as UCS-2 with a trailing null character.

**Structs**

- [`Ucs2CharFromUtf8`](#ucs2charfromutf8) - Value returned by `ucs2_from_utf8_at_offset`.

**Enums**

- [`Error`](#error) - Possible errors returned by the API.

**Functions**

- [`decode`](#decode) - Decode an input UCS-2 string into a UTF-8 string.
- [`decode_with`](#decode_with) - Decode UCS-2 string to UTF-8 with a custom callback function.
- [`encode`](#encode) - Encodes an input UTF-8 string into a UCS-2 string.
- [`encode_with`](#encode_with) - Encode UTF-8 string to UCS-2 with a custom callback function.
- [`ucs2_from_utf8_at_offset`](#ucs2_from_utf8_at_offset) - Get a UCS-2 character from a UTF-8 byte slice at the given offset.

**Type Aliases**

- [`Result`](#result)

---

## ucs2::Error

*Enum*

Possible errors returned by the API.

**Variants:**
- `BufferOverflow` - Not enough space left in the output buffer.
- `MultiByte` - Input contained a character which cannot be represented in UCS-2.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Error) -> $crate::cmp::Ordering`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Error) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## ucs2::Result

*Type Alias*: `core::result::Result<T, Error>`



## ucs2::Ucs2CharFromUtf8

*Struct*

Value returned by `ucs2_from_utf8_at_offset`.

**Fields:**
- `val: u16` - UCS-2 character.
- `num_bytes: u8` - Number of bytes needed to encode the character in UTF-8.



## ucs2::decode

*Function*

Decode an input UCS-2 string into a UTF-8 string.

The returned `usize` represents the length of the returned buffer,
in bytes. Due to the nature of UCS-2, the output buffer could end up with
three bytes for every character in the input buffer.

```rust
fn decode(input: &[u16], output: & mut [u8]) -> core::result::Result<usize, Error>
```



## ucs2::decode_with

*Function*

Decode UCS-2 string to UTF-8 with a custom callback function.

`output` is a function which receives every decoded character.
Due to the nature of UCS-2, the function can receive an UTF-8 character
of up to three bytes, for every input character.

```rust
fn decode_with<F>(input: &[u16], output: F) -> core::result::Result<usize, Error>
```



## ucs2::encode

*Function*

Encodes an input UTF-8 string into a UCS-2 string.

The returned `usize` represents the length of the returned buffer,
measured in 2-byte characters.

```rust
fn encode(input: &str, buffer: & mut [u16]) -> core::result::Result<usize, Error>
```



## ucs2::encode_with

*Function*

Encode UTF-8 string to UCS-2 with a custom callback function.

`output` is a function which receives every encoded character.

```rust
fn encode_with<F>(input: &str, output: F) -> core::result::Result<(), Error>
```



## Module: macros



## ucs2::ucs2_cstr

*Declarative Macro*

Encode a string as UCS-2 with a trailing null character.

The encoding is done at compile time, so the result can be used in a
`const` item. The type returned by the macro is a `[u16; N]` array;
to avoid having to specify what `N` is in a `const` item, take a
reference and store it as `&[u16]`.

# Example

```
use ucs2::ucs2_cstr;

const S: &[u16] = &ucs2_cstr!("abc");
assert_eq!(S, [97, 98, 99, 0]);
```

```rust
macro_rules! ucs2_cstr {
    ($s:literal) => { ... };
}
```



## ucs2::ucs2_from_utf8_at_offset

*Function*

Get a UCS-2 character from a UTF-8 byte slice at the given offset.

# Safety

The input `bytes` must be valid UTF-8.

```rust
fn ucs2_from_utf8_at_offset(bytes: &[u8], offset: usize) -> core::result::Result<Ucs2CharFromUtf8, Error>
```



