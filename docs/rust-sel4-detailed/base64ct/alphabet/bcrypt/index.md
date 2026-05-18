*[base64ct](../../index.md) / [alphabet](../index.md) / [bcrypt](index.md)*

---

# Module `bcrypt`

bcrypt Base64 encoding.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Base64Bcrypt`](#base64bcrypt) | struct | bcrypt Base64 encoding. |

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

- <span id="base64bcrypt-clone"></span>`fn clone(&self) -> Base64Bcrypt` — [`Base64Bcrypt`](#base64bcrypt)

##### `impl Copy for Base64Bcrypt`

##### `impl Debug for Base64Bcrypt`

- <span id="base64bcrypt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Bcrypt`

- <span id="base64bcrypt-default"></span>`fn default() -> Base64Bcrypt` — [`Base64Bcrypt`](#base64bcrypt)

##### `impl Encoding for Base64Bcrypt`

- <span id="base64bcrypt-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64bcrypt-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](../../errors/index.md#invalidencodingerror)

- <span id="base64bcrypt-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64bcrypt-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](../../errors/index.md#invalidlengtherror)

- <span id="base64bcrypt-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64bcrypt-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Bcrypt`

##### `impl Hash for Base64Bcrypt`

- <span id="base64bcrypt-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Bcrypt`

- <span id="base64bcrypt-ord-cmp"></span>`fn cmp(&self, other: &Base64Bcrypt) -> cmp::Ordering` — [`Base64Bcrypt`](#base64bcrypt)

##### `impl PartialEq for Base64Bcrypt`

- <span id="base64bcrypt-partialeq-eq"></span>`fn eq(&self, other: &Base64Bcrypt) -> bool` — [`Base64Bcrypt`](#base64bcrypt)

##### `impl PartialOrd for Base64Bcrypt`

- <span id="base64bcrypt-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Bcrypt) -> option::Option<cmp::Ordering>` — [`Base64Bcrypt`](#base64bcrypt)

##### `impl StructuralPartialEq for Base64Bcrypt`

