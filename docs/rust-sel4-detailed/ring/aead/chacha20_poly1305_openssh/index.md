*[ring](../../index.md) / [aead](../index.md) / [chacha20_poly1305_openssh](index.md)*

---

# Module `chacha20_poly1305_openssh`

The [chacha20-poly1305@openssh.com] AEAD-ish construct.

This should only be used by SSH implementations. It has a similar, but
different API from `ring::aead` because the construct cannot use the same
API as `ring::aead` due to the way the construct handles the encrypted
packet length.

The concatenation of a and b is denoted `a||b`. `K_1` and `K_2` are defined
in the [chacha20-poly1305@openssh.com] specification. `packet_length`,
`padding_length`, `payload`, and `random padding` are defined in
[RFC 4253]. The term `plaintext` is used as a shorthand for
`padding_length||payload||random padding`.



## Contents

- [Structs](#structs)
  - [`SealingKey`](#sealingkey)
  - [`OpeningKey`](#openingkey)
  - [`Key`](#key)
- [Functions](#functions)
  - [`make_counter`](#make-counter)
  - [`verify`](#verify)
- [Constants](#constants)
  - [`KEY_LEN`](#key-len)
  - [`PACKET_LENGTH_LEN`](#packet-length-len)
  - [`TAG_LEN`](#tag-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SealingKey`](#sealingkey) | struct | A key for sealing packets. |
| [`OpeningKey`](#openingkey) | struct | A key for opening packets. |
| [`Key`](#key) | struct |  |
| [`make_counter`](#make-counter) | fn |  |
| [`verify`](#verify) | fn |  |
| [`KEY_LEN`](#key-len) | const | The length of key. |
| [`PACKET_LENGTH_LEN`](#packet-length-len) | const | The length in bytes of the `packet_length` field in a SSH packet. |
| [`TAG_LEN`](#tag-len) | const | The length in bytes of an authentication tag. |

## Structs

### `SealingKey`

```rust
struct SealingKey {
    key: Key,
}
```

A key for sealing packets.

#### Implementations

- <span id="sealingkey-new"></span>`fn new(key_material: &[u8; 64]) -> Self`

  Constructs a new `SealingKey`.

- <span id="sealingkey-seal-in-place"></span>`fn seal_in_place(&self, sequence_number: u32, plaintext_in_ciphertext_out: &mut [u8], tag_out: &mut [u8; 16])`

  Seals (encrypts and signs) a packet.

  

  On input, `plaintext_in_ciphertext_out` must contain the unencrypted

  `packet_length||plaintext` where `plaintext` is the

  `padding_length||payload||random padding`. It will be overwritten by

  `encrypted_packet_length||ciphertext`, where `encrypted_packet_length`

  is encrypted with `K_1` and `ciphertext` is encrypted by `K_2`.

### `OpeningKey`

```rust
struct OpeningKey {
    key: Key,
}
```

A key for opening packets.

#### Implementations

- <span id="openingkey-new"></span>`fn new(key_material: &[u8; 64]) -> Self`

  Constructs a new `OpeningKey`.

- <span id="openingkey-decrypt-packet-length"></span>`fn decrypt_packet_length(&self, sequence_number: u32, encrypted_packet_length: [u8; 4]) -> [u8; 4]`

  Returns the decrypted, but unauthenticated, packet length.

  

  Importantly, the result won't be authenticated until `open_in_place` is

  called.

- <span id="openingkey-open-in-place"></span>`fn open_in_place<'a>(&self, sequence_number: u32, ciphertext_in_plaintext_out: &'a mut [u8], tag: &[u8; 16]) -> Result<&'a [u8], error::Unspecified>` — [`Unspecified`](../../error/index.md#unspecified)

  Opens (authenticates and decrypts) a packet.

  

  `ciphertext_in_plaintext_out` must be of the form

  `encrypted_packet_length||ciphertext` where `ciphertext` is the

  encrypted `plaintext`. When the function succeeds the ciphertext is

  replaced by the plaintext and the result is `Ok(plaintext)`, where

  `plaintext` is `&ciphertext_in_plaintext_out[PACKET_LENGTH_LEN..]`;

  otherwise the contents of `ciphertext_in_plaintext_out` are unspecified

  and must not be used.

### `Key`

```rust
struct Key {
    k_1: chacha::Key,
    k_2: chacha::Key,
}
```

#### Implementations

- <span id="key-new"></span>`fn new(key_material: &[u8; 64]) -> Self`

## Functions

### `make_counter`

```rust
fn make_counter(sequence_number: u32) -> Counter
```

### `verify`

```rust
fn verify(key: poly1305::Key, msg: &[u8], tag: &[u8; 16]) -> Result<(), error::Unspecified>
```

## Constants

### `KEY_LEN`
```rust
const KEY_LEN: usize = 64usize;
```

The length of key.

### `PACKET_LENGTH_LEN`
```rust
const PACKET_LENGTH_LEN: usize = 4usize;
```

The length in bytes of the `packet_length` field in a SSH packet.

### `TAG_LEN`
```rust
const TAG_LEN: usize = 16usize;
```

The length in bytes of an authentication tag.

