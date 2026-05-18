**ring > rsa > public_key**

# Module: rsa::public_key

## Contents

**Structs**

- [`PublicKey`](#publickey) - An RSA Public Key.

---

## ring::rsa::public_key::PublicKey

*Struct*

An RSA Public Key.

**Methods:**

- `fn modulus_len(self: &Self) -> usize` - The length, in bytes, of the public modulus.

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Clone**
  - `fn clone(self: &Self) -> PublicKey`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`



