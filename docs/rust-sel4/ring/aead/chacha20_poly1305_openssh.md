**ring > aead > chacha20_poly1305_openssh**

# Module: aead::chacha20_poly1305_openssh

## Contents

**Structs**

- [`OpeningKey`](#openingkey) - A key for opening packets.
- [`SealingKey`](#sealingkey) - A key for sealing packets.

**Constants**

- [`KEY_LEN`](#key_len) - The length of key.
- [`PACKET_LENGTH_LEN`](#packet_length_len) - The length in bytes of the `packet_length` field in a SSH packet.
- [`TAG_LEN`](#tag_len) - The length in bytes of an authentication tag.

---

## ring::aead::chacha20_poly1305_openssh::KEY_LEN

*Constant*: `usize`

The length of key.



## ring::aead::chacha20_poly1305_openssh::OpeningKey

*Struct*

A key for opening packets.

**Methods:**

- `fn new(key_material: &[u8; 64]) -> Self` - Constructs a new `OpeningKey`.
- `fn decrypt_packet_length(self: &Self, sequence_number: u32, encrypted_packet_length: [u8; 4]) -> [u8; 4]` - Returns the decrypted, but unauthenticated, packet length.
- `fn open_in_place<'a>(self: &Self, sequence_number: u32, ciphertext_in_plaintext_out: &'a  mut [u8], tag: &[u8; 16]) -> Result<&'a [u8], error::Unspecified>` - Opens (authenticates and decrypts) a packet.



## ring::aead::chacha20_poly1305_openssh::PACKET_LENGTH_LEN

*Constant*: `usize`

The length in bytes of the `packet_length` field in a SSH packet.



## ring::aead::chacha20_poly1305_openssh::SealingKey

*Struct*

A key for sealing packets.

**Methods:**

- `fn new(key_material: &[u8; 64]) -> Self` - Constructs a new `SealingKey`.
- `fn seal_in_place(self: &Self, sequence_number: u32, plaintext_in_ciphertext_out: & mut [u8], tag_out: & mut [u8; 16])` - Seals (encrypts and signs) a packet.



## ring::aead::chacha20_poly1305_openssh::TAG_LEN

*Constant*: `usize`

The length in bytes of an authentication tag.



