*[base64ct](../../index.md) / [alphabet](../index.md) / [pbkdf2](index.md)*

---

# Module `pbkdf2`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Base64Pbkdf2`](#base64pbkdf2) | struct | PBKDF2 Base64: variant of unpadded standard Base64 with `.` instead of `+`. |
| [`DECODER`](#decoder) | const |  |
| [`ENCODER`](#encoder) | const |  |

## Structs

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

- <span id="base64pbkdf2-clone"></span>`fn clone(&self) -> Base64Pbkdf2` — [`Base64Pbkdf2`](#base64pbkdf2)

##### `impl Copy for Base64Pbkdf2`

##### `impl Debug for Base64Pbkdf2`

- <span id="base64pbkdf2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Base64Pbkdf2`

- <span id="base64pbkdf2-default"></span>`fn default() -> Base64Pbkdf2` — [`Base64Pbkdf2`](#base64pbkdf2)

##### `impl Encoding for Base64Pbkdf2`

- <span id="base64pbkdf2-encoding-decode"></span>`fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<&[u8], Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64pbkdf2-encoding-decode-in-place"></span>`fn decode_in_place(buf: &mut [u8]) -> Result<&[u8], InvalidEncodingError>` — [`InvalidEncodingError`](../../errors/index.md#invalidencodingerror)

- <span id="base64pbkdf2-encoding-decode-vec"></span>`fn decode_vec(input: &str) -> Result<Vec<u8>, Error>` — [`Error`](../../errors/index.md#error)

- <span id="base64pbkdf2-encoding-encode"></span>`fn encode<'a>(src: &[u8], dst: &'a mut [u8]) -> Result<&'a str, InvalidLengthError>` — [`InvalidLengthError`](../../errors/index.md#invalidlengtherror)

- <span id="base64pbkdf2-encoding-encode-string"></span>`fn encode_string(input: &[u8]) -> String`

- <span id="base64pbkdf2-encoding-encoded-len"></span>`fn encoded_len(bytes: &[u8]) -> usize`

##### `impl Eq for Base64Pbkdf2`

##### `impl Hash for Base64Pbkdf2`

- <span id="base64pbkdf2-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Base64Pbkdf2`

- <span id="base64pbkdf2-ord-cmp"></span>`fn cmp(&self, other: &Base64Pbkdf2) -> cmp::Ordering` — [`Base64Pbkdf2`](#base64pbkdf2)

##### `impl PartialEq for Base64Pbkdf2`

- <span id="base64pbkdf2-partialeq-eq"></span>`fn eq(&self, other: &Base64Pbkdf2) -> bool` — [`Base64Pbkdf2`](#base64pbkdf2)

##### `impl PartialOrd for Base64Pbkdf2`

- <span id="base64pbkdf2-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Base64Pbkdf2) -> option::Option<cmp::Ordering>` — [`Base64Pbkdf2`](#base64pbkdf2)

##### `impl StructuralPartialEq for Base64Pbkdf2`

## Constants

### `DECODER`
```rust
const DECODER: &[crate::alphabet::DecodeStep];
```

### `ENCODER`
```rust
const ENCODER: &[crate::alphabet::EncodeStep];
```

