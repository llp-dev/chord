*[base64ct](../../index.md) / [alphabet](../index.md) / [shacrypt](index.md)*

---

# Module `shacrypt`

`crypt(3)` Base64 encoding for sha* family.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Base64ShaCrypt`](#base64shacrypt) | struct | `crypt(3)` Base64 encoding. |

## Structs

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

- <span id="base64shacrypt-clone"></span>`fn clone(&self) -> Base64ShaCrypt` — [`Base64ShaCrypt`](#base64shacrypt)

##### `impl Copy for Base64ShaCrypt`

##### `impl Debug for Base64ShaCrypt`

- <span id="base64shacrypt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64ShaCrypt`

- <span id="base64shacrypt-default"></span>`fn default() -> Base64ShaCrypt` — [`Base64ShaCrypt`](#base64shacrypt)

##### `impl Encoding for Base64ShaCrypt`

- <span id="base64shacrypt-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64shacrypt-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](../../errors/index.md#invalidencodingerror)

- <span id="base64shacrypt-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64shacrypt-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](../../errors/index.md#invalidlengtherror)

- <span id="base64shacrypt-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64shacrypt-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64ShaCrypt`

##### `impl Hash for Base64ShaCrypt`

- <span id="base64shacrypt-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64ShaCrypt`

- <span id="base64shacrypt-ord-cmp"></span>`fn cmp(&self, other: &Base64ShaCrypt) -> cmp::Ordering` — [`Base64ShaCrypt`](#base64shacrypt)

##### `impl PartialEq for Base64ShaCrypt`

- <span id="base64shacrypt-partialeq-eq"></span>`fn eq(&self, other: &Base64ShaCrypt) -> bool` — [`Base64ShaCrypt`](#base64shacrypt)

##### `impl PartialOrd for Base64ShaCrypt`

- <span id="base64shacrypt-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64ShaCrypt) -> option::Option<cmp::Ordering>` — [`Base64ShaCrypt`](#base64shacrypt)

##### `impl StructuralPartialEq for Base64ShaCrypt`

