*[base64ct](../index.md) / [alphabet](index.md)*

---

# Module `alphabet`

Base64 alphabets.

## Contents

- [Modules](#modules)
  - [`bcrypt`](#bcrypt)
  - [`crypt`](#crypt)
  - [`pbkdf2`](#pbkdf2)
  - [`shacrypt`](#shacrypt)
  - [`standard`](#standard)
  - [`url`](#url)
- [Enums](#enums)
  - [`DecodeStep`](#decodestep)
  - [`EncodeStep`](#encodestep)
- [Traits](#traits)
  - [`Alphabet`](#alphabet)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bcrypt`](#bcrypt) | mod | bcrypt Base64 encoding. |
| [`crypt`](#crypt) | mod | `crypt(3)` Base64 encoding. |
| [`pbkdf2`](#pbkdf2) | mod |  |
| [`shacrypt`](#shacrypt) | mod | `crypt(3)` Base64 encoding for sha* family. |
| [`standard`](#standard) | mod | Standard Base64 encoding. |
| [`url`](#url) | mod | URL-safe Base64 encoding. |
| [`DecodeStep`](#decodestep) | enum | Constant-time decoder step. |
| [`EncodeStep`](#encodestep) | enum | Constant-time encoder step. |
| [`Alphabet`](#alphabet) | trait | Core encoder/decoder functions for a particular Base64 alphabet. |

## Modules

- [`bcrypt`](bcrypt/index.md) — bcrypt Base64 encoding.
- [`crypt`](crypt/index.md) — `crypt(3)` Base64 encoding.
- [`pbkdf2`](pbkdf2/index.md)
- [`shacrypt`](shacrypt/index.md) — `crypt(3)` Base64 encoding for sha* family.
- [`standard`](standard/index.md) — Standard Base64 encoding.
- [`url`](url/index.md) — URL-safe Base64 encoding.

## Enums

### `DecodeStep`

```rust
enum DecodeStep {
    Range(core::ops::RangeInclusive<u8>, i16),
    Eq(u8, i16),
}
```

Constant-time decoder step.

#### Variants

- **`Range`**

  Match the given range, offsetting the input on match.

- **`Eq`**

  Match the given value, returning the associated offset on match.

#### Trait Implementations

##### `impl Debug for DecodeStep`

- <span id="decodestep-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EncodeStep`

```rust
enum EncodeStep {
    Apply(u8, i16),
    Diff(u8, i16),
}
```

Constant-time encoder step.

#### Variants

- **`Apply`**

  Apply the given offset to the cumulative result on match.

- **`Diff`**

  Compute a difference using the given offset on match.

#### Trait Implementations

##### `impl Clone for EncodeStep`

- <span id="encodestep-clone"></span>`fn clone(&self) -> EncodeStep` — [`EncodeStep`](#encodestep)

##### `impl Copy for EncodeStep`

##### `impl Debug for EncodeStep`

- <span id="encodestep-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Alphabet`

```rust
trait Alphabet: 'static + Copy + Debug + Eq + Send + Sized + Sync { ... }
```

Core encoder/decoder functions for a particular Base64 alphabet.

#### Associated Types

- `type Unpadded: 1`

#### Associated Constants

- `const BASE: u8`

- `const DECODER: &'static [DecodeStep]`

- `const ENCODER: &'static [EncodeStep]`

- `const PADDED: bool`

#### Provided Methods

- `fn decode_3bytes(src: &[u8], dst: &mut [u8]) -> i16`

  Decode 3 bytes of a Base64 message.

- `fn decode_6bits(src: u8) -> i16`

  Decode 6-bits of a Base64 message.

- `fn encode_3bytes(src: &[u8], dst: &mut [u8])`

  Encode 3-bytes of a Base64 message.

- `fn encode_6bits(src: i16) -> u8`

  Encode 6-bits of a Base64 message.

#### Implementors

- [`Base64Bcrypt`](bcrypt/index.md#base64bcrypt)
- [`Base64Crypt`](crypt/index.md#base64crypt)
- [`Base64Pbkdf2`](pbkdf2/index.md#base64pbkdf2)
- [`Base64ShaCrypt`](shacrypt/index.md#base64shacrypt)
- [`Base64Unpadded`](standard/index.md#base64unpadded)
- [`Base64UrlUnpadded`](url/index.md#base64urlunpadded)
- [`Base64Url`](url/index.md#base64url)
- [`Base64`](standard/index.md#base64)

