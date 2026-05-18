**ring > rsa > public_key_components**

# Module: rsa::public_key_components

## Contents

**Structs**

- [`PublicKeyComponents`](#publickeycomponents) - RSA public key components.

---

## ring::rsa::public_key_components::PublicKeyComponents

*Struct*

RSA public key components.

`B` must implement `AsRef<[u8]>` like `&[u8]` or `Vec<u8>`.

**Generic Parameters:**
- B

**Fields:**
- `n: B` - The public modulus, encoded in big-endian bytes without leading zeros.
- `e: B` - The public exponent, encoded in big-endian bytes without leading zeros.

**Methods:**

- `fn verify(self: &Self, params: &RsaParameters, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>` - Verifies that `signature` is a valid signature of `message` using `self`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PublicKeyComponents<B>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`
- **From**
  - `fn from(public_key: &PublicKey) -> Self`



