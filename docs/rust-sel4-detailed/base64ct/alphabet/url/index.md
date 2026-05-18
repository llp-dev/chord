*[base64ct](../../index.md) / [alphabet](../index.md) / [url](index.md)*

---

# Module `url`

URL-safe Base64 encoding.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Base64Url`](#base64url) | struct | URL-safe Base64 encoding with `=` padding. |
| [`Base64UrlUnpadded`](#base64urlunpadded) | struct | URL-safe Base64 encoding *without* padding. |
| [`DECODER`](#decoder) | const | URL-safe Base64 decoder |
| [`ENCODER`](#encoder) | const | URL-safe Base64 encoder |

## Structs

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

- <span id="base64url-clone"></span>`fn clone(&self) -> Base64Url` — [`Base64Url`](#base64url)

##### `impl Copy for Base64Url`

##### `impl Debug for Base64Url`

- <span id="base64url-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Url`

- <span id="base64url-default"></span>`fn default() -> Base64Url` — [`Base64Url`](#base64url)

##### `impl Encoding for Base64Url`

- <span id="base64url-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64url-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](../../errors/index.md#invalidencodingerror)

- <span id="base64url-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64url-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](../../errors/index.md#invalidlengtherror)

- <span id="base64url-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64url-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Url`

##### `impl Hash for Base64Url`

- <span id="base64url-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Url`

- <span id="base64url-ord-cmp"></span>`fn cmp(&self, other: &Base64Url) -> cmp::Ordering` — [`Base64Url`](#base64url)

##### `impl PartialEq for Base64Url`

- <span id="base64url-partialeq-eq"></span>`fn eq(&self, other: &Base64Url) -> bool` — [`Base64Url`](#base64url)

##### `impl PartialOrd for Base64Url`

- <span id="base64url-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Url) -> option::Option<cmp::Ordering>` — [`Base64Url`](#base64url)

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

- <span id="base64urlunpadded-clone"></span>`fn clone(&self) -> Base64UrlUnpadded` — [`Base64UrlUnpadded`](#base64urlunpadded)

##### `impl Copy for Base64UrlUnpadded`

##### `impl Debug for Base64UrlUnpadded`

- <span id="base64urlunpadded-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64UrlUnpadded`

- <span id="base64urlunpadded-default"></span>`fn default() -> Base64UrlUnpadded` — [`Base64UrlUnpadded`](#base64urlunpadded)

##### `impl Encoding for Base64UrlUnpadded`

- <span id="base64urlunpadded-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64urlunpadded-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](../../errors/index.md#invalidencodingerror)

- <span id="base64urlunpadded-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64urlunpadded-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](../../errors/index.md#invalidlengtherror)

- <span id="base64urlunpadded-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64urlunpadded-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64UrlUnpadded`

##### `impl Hash for Base64UrlUnpadded`

- <span id="base64urlunpadded-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64UrlUnpadded`

- <span id="base64urlunpadded-ord-cmp"></span>`fn cmp(&self, other: &Base64UrlUnpadded) -> cmp::Ordering` — [`Base64UrlUnpadded`](#base64urlunpadded)

##### `impl PartialEq for Base64UrlUnpadded`

- <span id="base64urlunpadded-partialeq-eq"></span>`fn eq(&self, other: &Base64UrlUnpadded) -> bool` — [`Base64UrlUnpadded`](#base64urlunpadded)

##### `impl PartialOrd for Base64UrlUnpadded`

- <span id="base64urlunpadded-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64UrlUnpadded) -> option::Option<cmp::Ordering>` — [`Base64UrlUnpadded`](#base64urlunpadded)

##### `impl StructuralPartialEq for Base64UrlUnpadded`

## Constants

### `DECODER`
```rust
const DECODER: &[super::DecodeStep];
```

URL-safe Base64 decoder

### `ENCODER`
```rust
const ENCODER: &[super::EncodeStep];
```

URL-safe Base64 encoder

