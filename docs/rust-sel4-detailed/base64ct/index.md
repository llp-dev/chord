# Crate `base64ct`

# [RustCrypto]: Constant-Time Base64

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

Pure Rust implementation of Base64 ([RFC 4648]).

Implements multiple Base64 alphabets without data-dependent branches or lookup
tables, thereby providing portable "best effort" constant-time operation.

Supports `no_std` environments and avoids heap allocations in the core API
(but also provides optional `alloc` support for convenience).

[Documentation][docs-link]

## About

This crate implements several Base64 alphabets in constant-time for sidechannel
resistance, aimed at purposes like encoding/decoding the "PEM" format used to
store things like cryptographic private keys (i.e. in the `pem-rfc7468` crate).

The paper `Util::Lookup: Exploiting key decoding in cryptographic libraries`
demonstrates how the leakage from non-constant-time Base64 parsers can be used
to practically extract RSA private keys from SGX enclaves.

The padded variants require (`=`) padding. Unpadded variants expressly
reject such padding.

Whitespace is expressly disallowed, with the exception of the
`Decoder::new_wrapped` and `Encoder::new_wrapped` modes which provide
fixed-width line wrapping.

## Supported Base64 variants

- Standard Base64: `[A-Z]`, `[a-z]`, `[0-9]`, `+`, `/`
- URL-safe Base64: `[A-Z]`, `[a-z]`, `[0-9]`, `-`, `_`
- bcrypt Base64: `.`, `/`, `[A-Z]`, `[a-z]`, `[0-9]`
- `crypt(3)` Base64: `.`, `-`, `[0-9]`, `[A-Z]`, `[a-z]`
- PBKDF2 Base64: `[A-Z]`, `[a-z]`, `[0-9]`, `.`, `/`

## Minimum Supported Rust Version (MSRV) Policy

MSRV increases are not considered breaking changes and can happen in patch releases.

The crate MSRV accounts for all supported targets and crate feature combinations, excluding
explicitly unstable features.

## License

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)










[//]: # (links)






# Usage

## Allocating (enable `alloc` crate feature)

```rust
#[cfg(feature = "alloc")]
{
use base64ct::{Base64, Encoding};

let bytes = b"example bytestring!";
let encoded = Base64::encode_string(bytes);
assert_eq!(encoded, "ZXhhbXBsZSBieXRlc3RyaW5nIQ==");

let decoded = Base64::decode_vec(&encoded).unwrap();
assert_eq!(decoded, bytes);
}
```

## Heapless `no_std` usage

```rust
use base64ct::{Base64, Encoding};

const BUF_SIZE: usize = 128;

let bytes = b"example bytestring!";
assert!(Base64::encoded_len(bytes) <= BUF_SIZE);

let mut enc_buf = [0u8; BUF_SIZE];
let encoded = Base64::encode(bytes, &mut enc_buf).unwrap();
assert_eq!(encoded, "ZXhhbXBsZSBieXRlc3RyaW5nIQ==");

let mut dec_buf = [0u8; BUF_SIZE];
let decoded = Base64::decode(encoded, &mut dec_buf).unwrap();
assert_eq!(decoded, bytes);
```

# Implementation

Implemented using integer arithmetic alone without any lookup tables or
data-dependent branches, thereby providing portable "best effort"
constant-time operation.

Not constant-time with respect to message length (only data).

Adapted from the following constant-time C++ implementation of Base64:

<https://github.com/Sc00bz/ConstTimeEncoding/blob/master/base64.cpp>

Copyright (c) 2014 Steve "Sc00bz" Thomas (steve at tobtu dot com).
Derived code is dual licensed MIT + Apache 2 (with permission from Sc00bz).

## Contents

- [Modules](#modules)
  - [`alphabet`](#alphabet)
  - [`decoder`](#decoder)
  - [`encoder`](#encoder)
  - [`encoding`](#encoding)
  - [`errors`](#errors)
  - [`line_ending`](#line-ending)
- [Structs](#structs)
  - [`Base64Bcrypt`](#base64bcrypt)
  - [`Base64Pbkdf2`](#base64pbkdf2)
  - [`Base64ShaCrypt`](#base64shacrypt)
  - [`Base64`](#base64)
  - [`Base64Unpadded`](#base64unpadded)
  - [`Base64Url`](#base64url)
  - [`Base64UrlUnpadded`](#base64urlunpadded)
  - [`Decoder`](#decoder)
  - [`Encoder`](#encoder)
  - [`InvalidEncodingError`](#invalidencodingerror)
  - [`InvalidLengthError`](#invalidlengtherror)
  - [`Base64Crypt`](#base64crypt)
- [Enums](#enums)
  - [`Error`](#error)
  - [`LineEnding`](#lineending)
- [Traits](#traits)
  - [`Encoding`](#encoding)
- [Constants](#constants)
  - [`MIN_LINE_WIDTH`](#min-line-width)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alphabet`](#alphabet) | mod | Base64 alphabets. |
| [`decoder`](#decoder) | mod | Buffered Base64 decoder. |
| [`encoder`](#encoder) | mod | Buffered Base64 encoder. |
| [`encoding`](#encoding) | mod | Base64 encodings |
| [`errors`](#errors) | mod | Error types |
| [`line_ending`](#line-ending) | mod | Line endings. |
| [`Base64Bcrypt`](#base64bcrypt) | struct |  |
| [`Base64Pbkdf2`](#base64pbkdf2) | struct |  |
| [`Base64ShaCrypt`](#base64shacrypt) | struct |  |
| [`Base64`](#base64) | struct |  |
| [`Base64Unpadded`](#base64unpadded) | struct |  |
| [`Base64Url`](#base64url) | struct |  |
| [`Base64UrlUnpadded`](#base64urlunpadded) | struct |  |
| [`Decoder`](#decoder) | struct |  |
| [`Encoder`](#encoder) | struct |  |
| [`InvalidEncodingError`](#invalidencodingerror) | struct |  |
| [`InvalidLengthError`](#invalidlengtherror) | struct |  |
| [`Base64Crypt`](#base64crypt) | struct |  |
| [`Error`](#error) | enum |  |
| [`LineEnding`](#lineending) | enum |  |
| [`Encoding`](#encoding) | trait |  |
| [`MIN_LINE_WIDTH`](#min-line-width) | const | Minimum supported line width. |

## Modules

- [`alphabet`](alphabet/index.md) — Base64 alphabets.
- [`decoder`](decoder/index.md) — Buffered Base64 decoder.
- [`encoder`](encoder/index.md) — Buffered Base64 encoder.
- [`encoding`](encoding/index.md) — Base64 encodings
- [`errors`](errors/index.md) — Error types
- [`line_ending`](line_ending/index.md) — Line endings.

## Structs

### `Base64Bcrypt`

```rust
struct Base64Bcrypt;
```

bcrypt Base64 encoding.

```text
./         [A-Z]      [a-z]     [0-9]
0x2e-0x2f, 0x41-0x5a, 0x61-0x7a, 0x30-0x39
```

#### Trait Implementations

##### `impl Alphabet for Base64Bcrypt`

- <span id="base64bcrypt-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64bcrypt-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64bcrypt-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64bcrypt-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64bcrypt-alphabet-type-unpadded"></span>`type Unpadded = Base64Bcrypt`

##### `impl Clone for Base64Bcrypt`

- <span id="base64bcrypt-clone"></span>`fn clone(&self) -> Base64Bcrypt` — [`Base64Bcrypt`](alphabet/bcrypt/index.md#base64bcrypt)

##### `impl Copy for Base64Bcrypt`

##### `impl Debug for Base64Bcrypt`

- <span id="base64bcrypt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Bcrypt`

- <span id="base64bcrypt-default"></span>`fn default() -> Base64Bcrypt` — [`Base64Bcrypt`](alphabet/bcrypt/index.md#base64bcrypt)

##### `impl Encoding for Base64Bcrypt`

- <span id="base64bcrypt-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](errors/index.md#error)

- <span id="base64bcrypt-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

- <span id="base64bcrypt-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](errors/index.md#error)

- <span id="base64bcrypt-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

- <span id="base64bcrypt-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64bcrypt-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Bcrypt`

##### `impl Hash for Base64Bcrypt`

- <span id="base64bcrypt-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Bcrypt`

- <span id="base64bcrypt-ord-cmp"></span>`fn cmp(&self, other: &Base64Bcrypt) -> cmp::Ordering` — [`Base64Bcrypt`](alphabet/bcrypt/index.md#base64bcrypt)

##### `impl PartialEq for Base64Bcrypt`

- <span id="base64bcrypt-partialeq-eq"></span>`fn eq(&self, other: &Base64Bcrypt) -> bool` — [`Base64Bcrypt`](alphabet/bcrypt/index.md#base64bcrypt)

##### `impl PartialOrd for Base64Bcrypt`

- <span id="base64bcrypt-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Bcrypt) -> option::Option<cmp::Ordering>` — [`Base64Bcrypt`](alphabet/bcrypt/index.md#base64bcrypt)

##### `impl StructuralPartialEq for Base64Bcrypt`

### `Base64Pbkdf2`

```rust
struct Base64Pbkdf2;
```

PBKDF2 Base64: variant of unpadded standard Base64 with `.` instead of `+`.

```text
[A-Z]      [a-z]      [0-9]      .     /
0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2e, 0x2f
```

#### Trait Implementations

##### `impl Alphabet for Base64Pbkdf2`

- <span id="base64pbkdf2-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64pbkdf2-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64pbkdf2-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64pbkdf2-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64pbkdf2-alphabet-type-unpadded"></span>`type Unpadded = Base64Pbkdf2`

##### `impl Clone for Base64Pbkdf2`

- <span id="base64pbkdf2-clone"></span>`fn clone(&self) -> Base64Pbkdf2` — [`Base64Pbkdf2`](alphabet/pbkdf2/index.md#base64pbkdf2)

##### `impl Copy for Base64Pbkdf2`

##### `impl Debug for Base64Pbkdf2`

- <span id="base64pbkdf2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Pbkdf2`

- <span id="base64pbkdf2-default"></span>`fn default() -> Base64Pbkdf2` — [`Base64Pbkdf2`](alphabet/pbkdf2/index.md#base64pbkdf2)

##### `impl Encoding for Base64Pbkdf2`

- <span id="base64pbkdf2-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](errors/index.md#error)

- <span id="base64pbkdf2-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

- <span id="base64pbkdf2-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](errors/index.md#error)

- <span id="base64pbkdf2-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

- <span id="base64pbkdf2-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64pbkdf2-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Pbkdf2`

##### `impl Hash for Base64Pbkdf2`

- <span id="base64pbkdf2-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Pbkdf2`

- <span id="base64pbkdf2-ord-cmp"></span>`fn cmp(&self, other: &Base64Pbkdf2) -> cmp::Ordering` — [`Base64Pbkdf2`](alphabet/pbkdf2/index.md#base64pbkdf2)

##### `impl PartialEq for Base64Pbkdf2`

- <span id="base64pbkdf2-partialeq-eq"></span>`fn eq(&self, other: &Base64Pbkdf2) -> bool` — [`Base64Pbkdf2`](alphabet/pbkdf2/index.md#base64pbkdf2)

##### `impl PartialOrd for Base64Pbkdf2`

- <span id="base64pbkdf2-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Pbkdf2) -> option::Option<cmp::Ordering>` — [`Base64Pbkdf2`](alphabet/pbkdf2/index.md#base64pbkdf2)

##### `impl StructuralPartialEq for Base64Pbkdf2`

### `Base64ShaCrypt`

```rust
struct Base64ShaCrypt;
```

`crypt(3)` Base64 encoding.

This is the standard Base64 encoding used by password hashes for the following schemes:
- MD5-Crypt
- scrypt
- SHA1-Crypt
- SHA256-Crypt
- SHA512-Crypt
- yescrypt

```text
[.-9]      [A-Z]      [a-z]
0x2e-0x39, 0x41-0x5a, 0x61-0x7a
```

#### Trait Implementations

##### `impl Alphabet for Base64ShaCrypt`

- <span id="base64shacrypt-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64shacrypt-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64shacrypt-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64shacrypt-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64shacrypt-alphabet-type-unpadded"></span>`type Unpadded = Base64ShaCrypt`

- <span id="base64shacrypt-alphabet-decode-3bytes"></span>`fn decode_3bytes(src: &[u8], dst: &mut [u8]) -> i16`

- <span id="base64shacrypt-alphabet-encode-3bytes"></span>`fn encode_3bytes(src: &[u8], dst: &mut [u8])`

##### `impl Clone for Base64ShaCrypt`

- <span id="base64shacrypt-clone"></span>`fn clone(&self) -> Base64ShaCrypt` — [`Base64ShaCrypt`](alphabet/shacrypt/index.md#base64shacrypt)

##### `impl Copy for Base64ShaCrypt`

##### `impl Debug for Base64ShaCrypt`

- <span id="base64shacrypt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64ShaCrypt`

- <span id="base64shacrypt-default"></span>`fn default() -> Base64ShaCrypt` — [`Base64ShaCrypt`](alphabet/shacrypt/index.md#base64shacrypt)

##### `impl Encoding for Base64ShaCrypt`

- <span id="base64shacrypt-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](errors/index.md#error)

- <span id="base64shacrypt-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

- <span id="base64shacrypt-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](errors/index.md#error)

- <span id="base64shacrypt-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

- <span id="base64shacrypt-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64shacrypt-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64ShaCrypt`

##### `impl Hash for Base64ShaCrypt`

- <span id="base64shacrypt-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64ShaCrypt`

- <span id="base64shacrypt-ord-cmp"></span>`fn cmp(&self, other: &Base64ShaCrypt) -> cmp::Ordering` — [`Base64ShaCrypt`](alphabet/shacrypt/index.md#base64shacrypt)

##### `impl PartialEq for Base64ShaCrypt`

- <span id="base64shacrypt-partialeq-eq"></span>`fn eq(&self, other: &Base64ShaCrypt) -> bool` — [`Base64ShaCrypt`](alphabet/shacrypt/index.md#base64shacrypt)

##### `impl PartialOrd for Base64ShaCrypt`

- <span id="base64shacrypt-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64ShaCrypt) -> option::Option<cmp::Ordering>` — [`Base64ShaCrypt`](alphabet/shacrypt/index.md#base64shacrypt)

##### `impl StructuralPartialEq for Base64ShaCrypt`

### `Base64`

```rust
struct Base64;
```

Standard Base64 encoding with `=` padding.

```text
[A-Z]      [a-z]      [0-9]      +     /
0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2b, 0x2f
```

#### Trait Implementations

##### `impl Alphabet for Base64`

- <span id="base64-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64-alphabet-type-unpadded"></span>`type Unpadded = Base64Unpadded`

##### `impl Clone for Base64`

- <span id="base64-clone"></span>`fn clone(&self) -> Base64` — [`Base64`](alphabet/standard/index.md#base64)

##### `impl Copy for Base64`

##### `impl Debug for Base64`

- <span id="base64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64`

- <span id="base64-default"></span>`fn default() -> Base64` — [`Base64`](alphabet/standard/index.md#base64)

##### `impl Encoding for Base64`

- <span id="base64-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](errors/index.md#error)

- <span id="base64-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

- <span id="base64-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](errors/index.md#error)

- <span id="base64-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

- <span id="base64-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64`

##### `impl Hash for Base64`

- <span id="base64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64`

- <span id="base64-ord-cmp"></span>`fn cmp(&self, other: &Base64) -> cmp::Ordering` — [`Base64`](alphabet/standard/index.md#base64)

##### `impl PartialEq for Base64`

- <span id="base64-partialeq-eq"></span>`fn eq(&self, other: &Base64) -> bool` — [`Base64`](alphabet/standard/index.md#base64)

##### `impl PartialOrd for Base64`

- <span id="base64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64) -> option::Option<cmp::Ordering>` — [`Base64`](alphabet/standard/index.md#base64)

##### `impl StructuralPartialEq for Base64`

### `Base64Unpadded`

```rust
struct Base64Unpadded;
```

Standard Base64 encoding *without* padding.

```text
[A-Z]      [a-z]      [0-9]      +     /
0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2b, 0x2f
```

#### Trait Implementations

##### `impl Alphabet for Base64Unpadded`

- <span id="base64unpadded-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64unpadded-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64unpadded-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64unpadded-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64unpadded-alphabet-type-unpadded"></span>`type Unpadded = Base64Unpadded`

##### `impl Clone for Base64Unpadded`

- <span id="base64unpadded-clone"></span>`fn clone(&self) -> Base64Unpadded` — [`Base64Unpadded`](alphabet/standard/index.md#base64unpadded)

##### `impl Copy for Base64Unpadded`

##### `impl Debug for Base64Unpadded`

- <span id="base64unpadded-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Unpadded`

- <span id="base64unpadded-default"></span>`fn default() -> Base64Unpadded` — [`Base64Unpadded`](alphabet/standard/index.md#base64unpadded)

##### `impl Encoding for Base64Unpadded`

- <span id="base64unpadded-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](errors/index.md#error)

- <span id="base64unpadded-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

- <span id="base64unpadded-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](errors/index.md#error)

- <span id="base64unpadded-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

- <span id="base64unpadded-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64unpadded-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Unpadded`

##### `impl Hash for Base64Unpadded`

- <span id="base64unpadded-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Unpadded`

- <span id="base64unpadded-ord-cmp"></span>`fn cmp(&self, other: &Base64Unpadded) -> cmp::Ordering` — [`Base64Unpadded`](alphabet/standard/index.md#base64unpadded)

##### `impl PartialEq for Base64Unpadded`

- <span id="base64unpadded-partialeq-eq"></span>`fn eq(&self, other: &Base64Unpadded) -> bool` — [`Base64Unpadded`](alphabet/standard/index.md#base64unpadded)

##### `impl PartialOrd for Base64Unpadded`

- <span id="base64unpadded-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Unpadded) -> option::Option<cmp::Ordering>` — [`Base64Unpadded`](alphabet/standard/index.md#base64unpadded)

##### `impl StructuralPartialEq for Base64Unpadded`

### `Base64Url`

```rust
struct Base64Url;
```

URL-safe Base64 encoding with `=` padding.

```text
[A-Z]      [a-z]      [0-9]      -     _
0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2d, 0x5f
```

#### Trait Implementations

##### `impl Alphabet for Base64Url`

- <span id="base64url-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64url-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64url-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64url-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64url-alphabet-type-unpadded"></span>`type Unpadded = Base64UrlUnpadded`

##### `impl Clone for Base64Url`

- <span id="base64url-clone"></span>`fn clone(&self) -> Base64Url` — [`Base64Url`](alphabet/url/index.md#base64url)

##### `impl Copy for Base64Url`

##### `impl Debug for Base64Url`

- <span id="base64url-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Url`

- <span id="base64url-default"></span>`fn default() -> Base64Url` — [`Base64Url`](alphabet/url/index.md#base64url)

##### `impl Encoding for Base64Url`

- <span id="base64url-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](errors/index.md#error)

- <span id="base64url-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

- <span id="base64url-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](errors/index.md#error)

- <span id="base64url-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

- <span id="base64url-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64url-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Url`

##### `impl Hash for Base64Url`

- <span id="base64url-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Url`

- <span id="base64url-ord-cmp"></span>`fn cmp(&self, other: &Base64Url) -> cmp::Ordering` — [`Base64Url`](alphabet/url/index.md#base64url)

##### `impl PartialEq for Base64Url`

- <span id="base64url-partialeq-eq"></span>`fn eq(&self, other: &Base64Url) -> bool` — [`Base64Url`](alphabet/url/index.md#base64url)

##### `impl PartialOrd for Base64Url`

- <span id="base64url-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Url) -> option::Option<cmp::Ordering>` — [`Base64Url`](alphabet/url/index.md#base64url)

##### `impl StructuralPartialEq for Base64Url`

### `Base64UrlUnpadded`

```rust
struct Base64UrlUnpadded;
```

URL-safe Base64 encoding *without* padding.

```text
[A-Z]      [a-z]      [0-9]      -     _
0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2d, 0x5f
```

#### Trait Implementations

##### `impl Alphabet for Base64UrlUnpadded`

- <span id="base64urlunpadded-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64urlunpadded-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64urlunpadded-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64urlunpadded-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64urlunpadded-alphabet-type-unpadded"></span>`type Unpadded = Base64UrlUnpadded`

##### `impl Clone for Base64UrlUnpadded`

- <span id="base64urlunpadded-clone"></span>`fn clone(&self) -> Base64UrlUnpadded` — [`Base64UrlUnpadded`](alphabet/url/index.md#base64urlunpadded)

##### `impl Copy for Base64UrlUnpadded`

##### `impl Debug for Base64UrlUnpadded`

- <span id="base64urlunpadded-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64UrlUnpadded`

- <span id="base64urlunpadded-default"></span>`fn default() -> Base64UrlUnpadded` — [`Base64UrlUnpadded`](alphabet/url/index.md#base64urlunpadded)

##### `impl Encoding for Base64UrlUnpadded`

- <span id="base64urlunpadded-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](errors/index.md#error)

- <span id="base64urlunpadded-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

- <span id="base64urlunpadded-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](errors/index.md#error)

- <span id="base64urlunpadded-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

- <span id="base64urlunpadded-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64urlunpadded-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64UrlUnpadded`

##### `impl Hash for Base64UrlUnpadded`

- <span id="base64urlunpadded-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64UrlUnpadded`

- <span id="base64urlunpadded-ord-cmp"></span>`fn cmp(&self, other: &Base64UrlUnpadded) -> cmp::Ordering` — [`Base64UrlUnpadded`](alphabet/url/index.md#base64urlunpadded)

##### `impl PartialEq for Base64UrlUnpadded`

- <span id="base64urlunpadded-partialeq-eq"></span>`fn eq(&self, other: &Base64UrlUnpadded) -> bool` — [`Base64UrlUnpadded`](alphabet/url/index.md#base64urlunpadded)

##### `impl PartialOrd for Base64UrlUnpadded`

- <span id="base64urlunpadded-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64UrlUnpadded) -> option::Option<cmp::Ordering>` — [`Base64UrlUnpadded`](alphabet/url/index.md#base64urlunpadded)

##### `impl StructuralPartialEq for Base64UrlUnpadded`

### `Decoder<'i, E: Encoding>`

```rust
struct Decoder<'i, E: Encoding> {
    line: Line<'i>,
    line_reader: LineReader<'i>,
    remaining_len: usize,
    block_buffer: BlockBuffer,
    encoding: core::marker::PhantomData<E>,
}
```

Stateful Base64 decoder with support for buffered, incremental decoding.

The `E` type parameter can be any type which impls [`Encoding`](encoding/index.md) such as
[`Base64`](alphabet/standard/index.md) or [`Base64Unpadded`](alphabet/standard/index.md).

#### Fields

- **`line`**: `Line<'i>`

  Current line being processed.

- **`line_reader`**: `LineReader<'i>`

  Base64 input data reader.

- **`remaining_len`**: `usize`

  Length of the remaining data after Base64 decoding.

- **`block_buffer`**: `BlockBuffer`

  Block buffer used for non-block-aligned data.

- **`encoding`**: `core::marker::PhantomData<E>`

  Phantom parameter for the Base64 encoding in use.

#### Implementations

- <span id="decoder-new"></span>`fn new(input: &'i [u8]) -> Result<Self, Error>` — [`Error`](errors/index.md#error)

  Create a new decoder for a byte slice containing contiguous

  (non-newline-delimited) Base64-encoded data.

  

  # Returns

  - `Ok(decoder)` on success.

  - `Err(Error::InvalidLength)` if the input buffer is empty.

- <span id="decoder-new-wrapped"></span>`fn new_wrapped(input: &'i [u8], line_width: usize) -> Result<Self, Error>` — [`Error`](errors/index.md#error)

  Create a new decoder for a byte slice containing Base64 which

  line wraps at the given line length.

  

  Trailing newlines are not supported and must be removed in advance.

  

  Newlines are handled according to what are roughly [RFC7468] conventions:

  

  ```text

  [parsers] MUST handle different newline conventions

  ```

  

  RFC7468 allows any of the following as newlines, and allows a mixture

  of different types of newlines:

  

  ```text

  eol        = CRLF / CR / LF

  ```

  

  # Returns

  - `Ok(decoder)` on success.

  - `Err(Error::InvalidLength)` if the input buffer is empty or the line

    width is zero.

- <span id="decoder-decode"></span>`fn decode<'o>(&mut self, out: &'o mut [u8]) -> Result<&'o [u8], Error>` — [`Error`](errors/index.md#error)

  Fill the provided buffer with data decoded from Base64.

  

  Enough Base64 input data must remain to fill the entire buffer.

  

  # Returns

  - `Ok(bytes)` if the expected amount of data was read

  - `Err(Error::InvalidLength)` if the exact amount of data couldn't be read

- <span id="decoder-decode-to-end"></span>`fn decode_to_end<'o>(&mut self, buf: &'o mut Vec<u8>) -> Result<&'o [u8], Error>` — [`Error`](errors/index.md#error)

  Decode all remaining Base64 data, placing the result into `buf`.

  

  If successful, this function will return the total number of bytes

  decoded into `buf`.

- <span id="decoder-remaining-len"></span>`fn remaining_len(&self) -> usize`

  Get the length of the remaining data after Base64 decoding.

  

  Decreases every time data is decoded.

- <span id="decoder-is-finished"></span>`fn is_finished(&self) -> bool`

  Has all of the input data been decoded?

- <span id="decoder-fill-block-buffer"></span>`fn fill_block_buffer(&mut self) -> Result<(), Error>` — [`Error`](errors/index.md#error)

  Fill the block buffer with data.

- <span id="decoder-advance-line"></span>`fn advance_line(&mut self) -> Result<(), Error>` — [`Error`](errors/index.md#error)

  Advance the internal buffer to the next line.

- <span id="decoder-perform-decode"></span>`fn perform_decode<'o>(&self, src: &[u8], dst: &'o mut [u8]) -> Result<&'o [u8], Error>` — [`Error`](errors/index.md#error)

  Perform Base64 decoding operation.

#### Trait Implementations

##### `impl<E: clone::Clone + Encoding> Clone for Decoder<'i, E>`

- <span id="decoder-clone"></span>`fn clone(&self) -> Decoder<'i, E>` — [`Decoder`](decoder/index.md#decoder)

### `Encoder<'o, E: Encoding>`

```rust
struct Encoder<'o, E: Encoding> {
    output: &'o mut [u8],
    position: usize,
    block_buffer: BlockBuffer,
    line_wrapper: Option<LineWrapper>,
    encoding: core::marker::PhantomData<E>,
}
```

Stateful Base64 encoder with support for buffered, incremental encoding.

The `E` type parameter can be any type which impls [`Encoding`](encoding/index.md) such as
[`Base64`](alphabet/standard/index.md) or [`Base64Unpadded`](alphabet/standard/index.md).

#### Fields

- **`output`**: `&'o mut [u8]`

  Output buffer.

- **`position`**: `usize`

  Cursor within the output buffer.

- **`block_buffer`**: `BlockBuffer`

  Block buffer used for non-block-aligned data.

- **`line_wrapper`**: `Option<LineWrapper>`

  Configuration and state for line-wrapping the output at a specified
  column.

- **`encoding`**: `core::marker::PhantomData<E>`

  Phantom parameter for the Base64 encoding in use.

#### Implementations

- <span id="encoder-new"></span>`fn new(output: &'o mut [u8]) -> Result<Self, Error>` — [`Error`](errors/index.md#error)

  Create a new encoder which writes output to the given byte slice.

  

  Output constructed using this method is not line-wrapped.

- <span id="encoder-new-wrapped"></span>`fn new_wrapped(output: &'o mut [u8], width: usize, ending: LineEnding) -> Result<Self, Error>` — [`LineEnding`](line_ending/index.md#lineending), [`Error`](errors/index.md#error)

  Create a new encoder which writes line-wrapped output to the given byte

  slice.

  

  Output will be wrapped at the specified interval, using the provided

  line ending. Use `LineEnding::default()` to use the conventional line

  ending for the target OS.

  

  Minimum allowed line width is 4.

- <span id="encoder-encode"></span>`fn encode(&mut self, input: &[u8]) -> Result<(), Error>` — [`Error`](errors/index.md#error)

  Encode the provided buffer as Base64, writing it to the output buffer.

  

  # Returns

  - `Ok(())` if the expected amount of data was read

  - `Err(Error::InvalidLength)` if there is insufficient space in the output buffer

- <span id="encoder-position"></span>`fn position(&self) -> usize`

  Get the position inside of the output buffer where the write cursor

  is currently located.

- <span id="encoder-finish"></span>`fn finish(self) -> Result<&'o str, Error>` — [`Error`](errors/index.md#error)

  Finish encoding data, returning the resulting Base64 as a `str`.

- <span id="encoder-finish-with-remaining"></span>`fn finish_with_remaining(self) -> Result<(&'o str, &'o mut [u8]), Error>` — [`Error`](errors/index.md#error)

  Finish encoding data, returning the resulting Base64 as a `str`

  along with the remaining space in the output buffer.

- <span id="encoder-remaining"></span>`fn remaining(&mut self) -> &mut [u8]`

  Borrow the remaining data in the buffer.

- <span id="encoder-process-buffer"></span>`fn process_buffer(&mut self, input: &mut &[u8]) -> Result<(), Error>` — [`Error`](errors/index.md#error)

  Fill the block buffer with data, consuming and encoding it when the

  buffer is full.

- <span id="encoder-perform-encode"></span>`fn perform_encode(&mut self, input: &[u8]) -> Result<usize, Error>` — [`Error`](errors/index.md#error)

  Perform Base64 encoding operation.

### `InvalidEncodingError`

```rust
struct InvalidEncodingError;
```

Invalid encoding of provided Base64 string.

#### Trait Implementations

##### `impl Clone for InvalidEncodingError`

- <span id="invalidencodingerror-clone"></span>`fn clone(&self) -> InvalidEncodingError` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

##### `impl Copy for InvalidEncodingError`

##### `impl Debug for InvalidEncodingError`

- <span id="invalidencodingerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for InvalidEncodingError`

- <span id="invalidencodingerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for InvalidEncodingError`

##### `impl Error for InvalidEncodingError`

##### `impl PartialEq for InvalidEncodingError`

- <span id="invalidencodingerror-partialeq-eq"></span>`fn eq(&self, other: &InvalidEncodingError) -> bool` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

##### `impl StructuralPartialEq for InvalidEncodingError`

##### `impl ToString for InvalidEncodingError`

- <span id="invalidencodingerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `InvalidLengthError`

```rust
struct InvalidLengthError;
```

Insufficient output buffer length.

#### Trait Implementations

##### `impl Clone for InvalidLengthError`

- <span id="invalidlengtherror-clone"></span>`fn clone(&self) -> InvalidLengthError` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

##### `impl Copy for InvalidLengthError`

##### `impl Debug for InvalidLengthError`

- <span id="invalidlengtherror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for InvalidLengthError`

- <span id="invalidlengtherror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for InvalidLengthError`

##### `impl Error for InvalidLengthError`

##### `impl PartialEq for InvalidLengthError`

- <span id="invalidlengtherror-partialeq-eq"></span>`fn eq(&self, other: &InvalidLengthError) -> bool` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

##### `impl StructuralPartialEq for InvalidLengthError`

##### `impl ToString for InvalidLengthError`

- <span id="invalidlengtherror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Base64Crypt`

```rust
struct Base64Crypt;
```

DEPRECATED: non-standard big endian variant of the `crypt(3)` Base64 encoding.

```text
[.-9]      [A-Z]      [a-z]
0x2e-0x39, 0x41-0x5a, 0x61-0x7a
```

<div class="warning">
This encodes using a big endian variant of Base64. Most modern algorithms which can be
used via `crypt(3)` use the [`Base64ShaCrypt`][`crate::Base64ShaCrypt`](alphabet/shacrypt/index.md) encoding.
</div>

#### Trait Implementations

##### `impl Alphabet for Base64Crypt`

- <span id="base64crypt-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64crypt-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64crypt-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64crypt-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64crypt-alphabet-type-unpadded"></span>`type Unpadded = Base64Crypt`

##### `impl Clone for Base64Crypt`

- <span id="base64crypt-clone"></span>`fn clone(&self) -> Base64Crypt` — [`Base64Crypt`](alphabet/crypt/index.md#base64crypt)

##### `impl Copy for Base64Crypt`

##### `impl Debug for Base64Crypt`

- <span id="base64crypt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Crypt`

- <span id="base64crypt-default"></span>`fn default() -> Base64Crypt` — [`Base64Crypt`](alphabet/crypt/index.md#base64crypt)

##### `impl Encoding for Base64Crypt`

- <span id="base64crypt-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](errors/index.md#error)

- <span id="base64crypt-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](errors/index.md#invalidencodingerror)

- <span id="base64crypt-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](errors/index.md#error)

- <span id="base64crypt-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](errors/index.md#invalidlengtherror)

- <span id="base64crypt-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64crypt-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Crypt`

##### `impl Hash for Base64Crypt`

- <span id="base64crypt-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Crypt`

- <span id="base64crypt-ord-cmp"></span>`fn cmp(&self, other: &Base64Crypt) -> cmp::Ordering` — [`Base64Crypt`](alphabet/crypt/index.md#base64crypt)

##### `impl PartialEq for Base64Crypt`

- <span id="base64crypt-partialeq-eq"></span>`fn eq(&self, other: &Base64Crypt) -> bool` — [`Base64Crypt`](alphabet/crypt/index.md#base64crypt)

##### `impl PartialOrd for Base64Crypt`

- <span id="base64crypt-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Crypt) -> option::Option<cmp::Ordering>` — [`Base64Crypt`](alphabet/crypt/index.md#base64crypt)

##### `impl StructuralPartialEq for Base64Crypt`

## Enums

### `Error`

```rust
enum Error {
    InvalidEncoding,
    InvalidLength,
}
```

Generic error, union of [`InvalidLengthError`](errors/index.md) and [`InvalidEncodingError`](errors/index.md).

#### Variants

- **`InvalidEncoding`**

  Invalid encoding of provided Base64 string.

- **`InvalidLength`**

  Insufficient output buffer length.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](errors/index.md#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](errors/index.md#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

### `LineEnding`

```rust
enum LineEnding {
    CR,
    LF,
    CRLF,
}
```

Line endings: variants of newline characters that can be used with Base64.

Use `LineEnding::default` to get an appropriate line ending for the
current operating system.

#### Variants

- **`CR`**

  Carriage return: `\r` (Pre-OS X Macintosh)

- **`LF`**

  Line feed: `\n` (Unix OSes)

- **`CRLF`**

  Carriage return + line feed: `\r\n` (Windows)

#### Implementations

- <span id="lineending-as-bytes"></span>`fn as_bytes(self) -> &'static [u8]`

  Get the byte serialization of this [`LineEnding`](line_ending/index.md).

- <span id="lineending-len"></span>`fn len(self) -> usize`

  Get the encoded length of this [`LineEnding`](line_ending/index.md).

#### Trait Implementations

##### `impl Clone for LineEnding`

- <span id="lineending-clone"></span>`fn clone(&self) -> LineEnding` — [`LineEnding`](line_ending/index.md#lineending)

##### `impl Copy for LineEnding`

##### `impl Debug for LineEnding`

- <span id="lineending-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LineEnding`

- <span id="lineending-default"></span>`fn default() -> LineEnding` — [`LineEnding`](line_ending/index.md#lineending)

##### `impl Eq for LineEnding`

##### `impl Ord for LineEnding`

- <span id="lineending-ord-cmp"></span>`fn cmp(&self, other: &LineEnding) -> cmp::Ordering` — [`LineEnding`](line_ending/index.md#lineending)

##### `impl PartialEq for LineEnding`

- <span id="lineending-partialeq-eq"></span>`fn eq(&self, other: &LineEnding) -> bool` — [`LineEnding`](line_ending/index.md#lineending)

##### `impl PartialOrd for LineEnding`

- <span id="lineending-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LineEnding) -> option::Option<cmp::Ordering>` — [`LineEnding`](line_ending/index.md#lineending)

##### `impl StructuralPartialEq for LineEnding`

## Traits

### `Encoding`

```rust
trait Encoding: Alphabet { ... }
```

Base64 encoding trait.

This trait must be imported to make use of any Base64 alphabet defined
in this crate.

The following encoding types impl this trait:

- [`Base64`](alphabet/standard/index.md): standard Base64 encoding with `=` padding.
- [`Base64Bcrypt`](alphabet/bcrypt/index.md): bcrypt Base64 encoding.
- [`Base64Crypt`](alphabet/crypt/index.md): `crypt(3)` Base64 encoding.
- [`Base64Unpadded`](alphabet/standard/index.md): standard Base64 encoding *without* padding.
- [`Base64Url`](alphabet/url/index.md): URL-safe Base64 encoding with `=` padding.
- [`Base64UrlUnpadded`](alphabet/url/index.md): URL-safe Base64 encoding *without* padding.

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

  Encode input byte slice into a [`String`](../clap_builder/index.md) containing Base64.

- `fn encoded_len(bytes: &[u8]) -> usize`

  Get the length of Base64 produced by encoding the given bytes.

#### Implementors

- `T`

## Constants

### `MIN_LINE_WIDTH`
```rust
const MIN_LINE_WIDTH: usize = 4usize;
```

Minimum supported line width.

