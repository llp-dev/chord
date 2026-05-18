**ring > rsa > keypair**

# Module: rsa::keypair

## Contents

**Structs**

- [`KeyPair`](#keypair) - An RSA key pair, used for signing.

---

## ring::rsa::keypair::KeyPair

*Struct*

An RSA key pair, used for signing.

**Methods:**

- `fn sign(self: &Self, padding_alg: &'static dyn RsaEncoding, rng: &dyn rand::SecureRandom, msg: &[u8], signature: & mut [u8]) -> Result<(), error::Unspecified>` - Computes the signature of `msg` and writes it into `signature`.
- `fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, KeyRejected>` - Parses an unencrypted PKCS#8-encoded RSA private key.
- `fn from_der(input: &[u8]) -> Result<Self, KeyRejected>` - Parses an RSA private key that is not inside a PKCS#8 wrapper.
- `fn from_components<Public, Private>(components: &KeyPairComponents<Public, Private>) -> Result<Self, KeyRejected>` - Constructs an RSA private key from its big-endian-encoded components.
- `fn public(self: &Self) -> &PublicKey` - Returns a reference to the public key.
- `fn public_modulus_len(self: &Self) -> usize` - Returns the length in bytes of the key pair's public modulus.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`
- **KeyPair**
  - `fn public_key(self: &Self) -> &<Self as >::PublicKey`



