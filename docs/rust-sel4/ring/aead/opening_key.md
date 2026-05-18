**ring > aead > opening_key**

# Module: aead::opening_key

## Contents

**Structs**

- [`OpeningKey`](#openingkey) - An AEAD key for authenticating and decrypting ("opening"), bound to a nonce

---

## ring::aead::opening_key::OpeningKey

*Struct*

An AEAD key for authenticating and decrypting ("opening"), bound to a nonce
sequence.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the nonce sequence.

**Generic Parameters:**
- N

**Methods:**

- `fn open_in_place<'in_out, A>(self: & mut Self, aad: Aad<A>, in_out: &'in_out  mut [u8]) -> Result<&'in_out  mut [u8], error::Unspecified>` - Authenticates and decrypts (“opens”) data in place.
- `fn open_within<'in_out, A>(self: & mut Self, aad: Aad<A>, in_out: &'in_out  mut [u8], ciphertext_and_tag: RangeFrom<usize>) -> Result<&'in_out  mut [u8], error::Unspecified>` - Authenticates and decrypts (“opens”) data in place, with a shift.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`
- **BoundKey**
  - `fn new(key: UnboundKey, nonce_sequence: N) -> Self`
  - `fn algorithm(self: &Self) -> &'static Algorithm`



