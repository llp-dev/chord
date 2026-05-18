**ring > rsa > keypair_components**

# Module: rsa::keypair_components

## Contents

**Structs**

- [`KeyPairComponents`](#keypaircomponents) - RSA key pair components.

---

## ring::rsa::keypair_components::KeyPairComponents

*Struct*

RSA key pair components.

**Generic Parameters:**
- Public
- Private

**Fields:**
- `public_key: super::PublicKeyComponents<Public>` - The public key components.
- `d: Private` - The private exponent.
- `p: Private` - The first prime factor of `d`.
- `q: Private` - The second prime factor of `d`.
- `dP: Private` - `p`'s public Chinese Remainder Theorem exponent.
- `dQ: Private` - `q`'s public Chinese Remainder Theorem exponent.
- `qInv: Private` - `q**-1 mod p`.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> KeyPairComponents<Public, Private>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`



