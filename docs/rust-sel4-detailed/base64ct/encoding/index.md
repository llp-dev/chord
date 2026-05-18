*[base64ct](../index.md) / [encoding](index.md)*

---

# Module `encoding`

Base64 encodings

## Contents

- [Traits](#traits)
  - [`Encoding`](#encoding)
- [Functions](#functions)
  - [`decode_padding`](#decode-padding)
  - [`validate_last_block`](#validate-last-block)
  - [`decoded_len`](#decoded-len)
  - [`is_pad_ct`](#is-pad-ct)
  - [`encoded_len_inner`](#encoded-len-inner)
- [Constants](#constants)
  - [`PAD`](#pad)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Encoding`](#encoding) | trait | Base64 encoding trait. |
| [`decode_padding`](#decode-padding) | fn | Validate padding is of the expected length compute unpadded length. |
| [`validate_last_block`](#validate-last-block) | fn | Validate that the last block of the decoded data round-trips back to the encoded data. |
| [`decoded_len`](#decoded-len) | fn | Get the length of the output from decoding the provided *unpadded* Base64-encoded input. |
| [`is_pad_ct`](#is-pad-ct) | fn | Branchless match that a given byte is the `PAD` character |
| [`encoded_len_inner`](#encoded-len-inner) | fn |  |
| [`PAD`](#pad) | const | Padding character |

## Traits

### `Encoding`

```rust
trait Encoding: Alphabet { ... }
```

Base64 encoding trait.

This trait must be imported to make use of any Base64 alphabet defined
in this crate.

The following encoding types impl this trait:

- [`Base64`](../alphabet/standard/index.md): standard Base64 encoding with `=` padding.
- [`Base64Bcrypt`](../alphabet/bcrypt/index.md): bcrypt Base64 encoding.
- [`Base64Crypt`](../alphabet/crypt/index.md): `crypt(3)` Base64 encoding.
- [`Base64Unpadded`](../alphabet/standard/index.md): standard Base64 encoding *without* padding.
- [`Base64Url`](../alphabet/url/index.md): URL-safe Base64 encoding with `=` padding.
- [`Base64UrlUnpadded`](../alphabet/url/index.md): URL-safe Base64 encoding *without* padding.

#### Required Methods

- `fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>`

  Decode a Base64 string into the provided destination buffer.

- `fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>`

  Decode a Base64 string in-place.

- `fn decode_vec(input: &str) -> Result<Vec<u8>, Error>`

  Decode a Base64 string into a byte vector.

- `fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>`

  Encode the input byte slice as Base64.

- `fn encode_string(input: &[u8]) -> String`

  Encode input byte slice into a [`String`](../../clap_builder/index.md) containing Base64.

- `fn encoded_len(bytes: &[u8]) -> usize`

  Get the length of Base64 produced by encoding the given bytes.

#### Implementors

- `T`

## Functions

### `decode_padding`

```rust
fn decode_padding(input: &[u8]) -> Result<(usize, i16), crate::errors::InvalidEncodingError>
```

Validate padding is of the expected length compute unpadded length.

Note that this method does not explicitly check that the padded data
is valid in and of itself: that is performed by `validate_last_block` as a
final step.

Returns length-related errors eagerly as a [`Result`](../../sel4/error/index.md), and data-dependent
errors (i.e. malformed padding bytes) as `i16` to be combined with other
encoding-related errors prior to branching.

### `validate_last_block`

```rust
fn validate_last_block<T: Alphabet>(encoded: &[u8], decoded: &[u8]) -> Result<(), crate::errors::Error>
```

Validate that the last block of the decoded data round-trips back to the
encoded data.

### `decoded_len`

```rust
fn decoded_len(input_len: usize) -> usize
```

Get the length of the output from decoding the provided *unpadded*
Base64-encoded input.

Note that this function does not fully validate the Base64 is well-formed
and may return incorrect results for malformed Base64.

### `is_pad_ct`

```rust
fn is_pad_ct(input: u8) -> i16
```

Branchless match that a given byte is the `PAD` character

### `encoded_len_inner`

```rust
const fn encoded_len_inner(n: usize, padded: bool) -> Option<usize>
```

## Constants

### `PAD`
```rust
const PAD: u8 = 61u8;
```

Padding character

