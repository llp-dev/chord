*[base64ct](../../index.md) / [alphabet](../index.md) / [crypt](index.md)*

---

# Module `crypt`

`crypt(3)` Base64 encoding.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Base64Crypt`](#base64crypt) | struct | DEPRECATED: non-standard big endian variant of the `crypt(3)` Base64 encoding. |

## Structs

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
used via `crypt(3)` use the [`Base64ShaCrypt`][`crate::Base64ShaCrypt`](../shacrypt/index.md) encoding.
</div>

#### Trait Implementations

##### `impl Alphabet for Base64Crypt`

- <span id="base64crypt-alphabet-const-base"></span>`const BASE: u8`

- <span id="base64crypt-alphabet-const-decoder"></span>`const DECODER: &'static [DecodeStep]`

- <span id="base64crypt-alphabet-const-encoder"></span>`const ENCODER: &'static [EncodeStep]`

- <span id="base64crypt-alphabet-const-padded"></span>`const PADDED: bool`

- <span id="base64crypt-alphabet-type-unpadded"></span>`type Unpadded = Base64Crypt`

##### `impl Clone for Base64Crypt`

- <span id="base64crypt-clone"></span>`fn clone(&self) -> Base64Crypt` — [`Base64Crypt`](#base64crypt)

##### `impl Copy for Base64Crypt`

##### `impl Debug for Base64Crypt`

- <span id="base64crypt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Crypt`

- <span id="base64crypt-default"></span>`fn default() -> Base64Crypt` — [`Base64Crypt`](#base64crypt)

##### `impl Encoding for Base64Crypt`

- <span id="base64crypt-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64crypt-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](../../errors/index.md#invalidencodingerror)

- <span id="base64crypt-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64crypt-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](../../errors/index.md#invalidlengtherror)

- <span id="base64crypt-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64crypt-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Crypt`

##### `impl Hash for Base64Crypt`

- <span id="base64crypt-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Crypt`

- <span id="base64crypt-ord-cmp"></span>`fn cmp(&self, other: &Base64Crypt) -> cmp::Ordering` — [`Base64Crypt`](#base64crypt)

##### `impl PartialEq for Base64Crypt`

- <span id="base64crypt-partialeq-eq"></span>`fn eq(&self, other: &Base64Crypt) -> bool` — [`Base64Crypt`](#base64crypt)

##### `impl PartialOrd for Base64Crypt`

- <span id="base64crypt-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Crypt) -> option::Option<cmp::Ordering>` — [`Base64Crypt`](#base64crypt)

##### `impl StructuralPartialEq for Base64Crypt`

