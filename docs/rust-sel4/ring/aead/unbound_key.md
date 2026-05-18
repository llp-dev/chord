**ring > aead > unbound_key**

# Module: aead::unbound_key

## Contents

**Structs**

- [`UnboundKey`](#unboundkey) - An AEAD key without a designated role or nonce sequence.

---

## ring::aead::unbound_key::UnboundKey

*Struct*

An AEAD key without a designated role or nonce sequence.

**Methods:**

- `fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>` - Constructs a `UnboundKey`.
- `fn algorithm(self: &Self) -> &'static Algorithm` - The key's AEAD algorithm.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`
- **From**
  - `fn from(okm: hkdf::Okm<&'static Algorithm>) -> Self`



