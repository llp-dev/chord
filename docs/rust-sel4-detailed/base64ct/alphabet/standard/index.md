*[base64ct](../../index.md) / [alphabet](../index.md) / [standard](index.md)*

---

# Module `standard`

Standard Base64 encoding.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Base64`](#base64) | struct | Standard Base64 encoding with `=` padding. |
| [`Base64Unpadded`](#base64unpadded) | struct | Standard Base64 encoding *without* padding. |
| [`DECODER`](#decoder) | const | Standard Base64 decoder |
| [`ENCODER`](#encoder) | const | Standard Base64 encoder |

## Structs

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

- <span id="base64-clone"></span>`fn clone(&self) -> Base64` — [`Base64`](#base64)

##### `impl Copy for Base64`

##### `impl Debug for Base64`

- <span id="base64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64`

- <span id="base64-default"></span>`fn default() -> Base64` — [`Base64`](#base64)

##### `impl Encoding for Base64`

- <span id="base64-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](../../errors/index.md#invalidencodingerror)

- <span id="base64-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](../../errors/index.md#invalidlengtherror)

- <span id="base64-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64`

##### `impl Hash for Base64`

- <span id="base64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64`

- <span id="base64-ord-cmp"></span>`fn cmp(&self, other: &Base64) -> cmp::Ordering` — [`Base64`](#base64)

##### `impl PartialEq for Base64`

- <span id="base64-partialeq-eq"></span>`fn eq(&self, other: &Base64) -> bool` — [`Base64`](#base64)

##### `impl PartialOrd for Base64`

- <span id="base64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64) -> option::Option<cmp::Ordering>` — [`Base64`](#base64)

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

- <span id="base64unpadded-clone"></span>`fn clone(&self) -> Base64Unpadded` — [`Base64Unpadded`](#base64unpadded)

##### `impl Copy for Base64Unpadded`

##### `impl Debug for Base64Unpadded`

- <span id="base64unpadded-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Unpadded`

- <span id="base64unpadded-default"></span>`fn default() -> Base64Unpadded` — [`Base64Unpadded`](#base64unpadded)

##### `impl Encoding for Base64Unpadded`

- <span id="base64unpadded-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64unpadded-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](../../errors/index.md#invalidencodingerror)

- <span id="base64unpadded-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64unpadded-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](../../errors/index.md#invalidlengtherror)

- <span id="base64unpadded-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64unpadded-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Unpadded`

##### `impl Hash for Base64Unpadded`

- <span id="base64unpadded-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Unpadded`

- <span id="base64unpadded-ord-cmp"></span>`fn cmp(&self, other: &Base64Unpadded) -> cmp::Ordering` — [`Base64Unpadded`](#base64unpadded)

##### `impl PartialEq for Base64Unpadded`

- <span id="base64unpadded-partialeq-eq"></span>`fn eq(&self, other: &Base64Unpadded) -> bool` — [`Base64Unpadded`](#base64unpadded)

##### `impl PartialOrd for Base64Unpadded`

- <span id="base64unpadded-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Unpadded) -> option::Option<cmp::Ordering>` — [`Base64Unpadded`](#base64unpadded)

##### `impl StructuralPartialEq for Base64Unpadded`

## Constants

### `DECODER`
```rust
const DECODER: &[super::DecodeStep];
```

Standard Base64 decoder

### `ENCODER`
```rust
const ENCODER: &[super::EncodeStep];
```

Standard Base64 encoder

