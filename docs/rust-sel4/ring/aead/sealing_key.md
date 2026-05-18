**ring > aead > sealing_key**

# Module: aead::sealing_key

## Contents

**Structs**

- [`SealingKey`](#sealingkey) - An AEAD key for encrypting and signing ("sealing"), bound to a nonce

---

## ring::aead::sealing_key::SealingKey

*Struct*

An AEAD key for encrypting and signing ("sealing"), bound to a nonce
sequence.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the nonce sequence.

**Generic Parameters:**
- N

**Methods:**

- `fn seal_in_place_append_tag<A, InOut>(self: & mut Self, aad: Aad<A>, in_out: & mut InOut) -> Result<(), error::Unspecified>` - Encrypts and signs (“seals”) data in place, appending the tag to the
- `fn seal_in_place_separate_tag<A>(self: & mut Self, aad: Aad<A>, in_out: & mut [u8]) -> Result<Tag, error::Unspecified>` - Encrypts and signs (“seals”) data in place.

**Trait Implementations:**

- **BoundKey**
  - `fn new(key: UnboundKey, nonce_sequence: N) -> Self`
  - `fn algorithm(self: &Self) -> &'static Algorithm`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`



