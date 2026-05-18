**ring > ec > curve25519 > ed25519 > signing**

# Module: ec::curve25519::ed25519::signing

## Contents

**Structs**

- [`Ed25519KeyPair`](#ed25519keypair) - An Ed25519 key pair, for signing.
- [`PublicKey`](#publickey)

---

## ring::ec::curve25519::ed25519::signing::Ed25519KeyPair

*Struct*

An Ed25519 key pair, for signing.

**Methods:**

- `fn generate_pkcs8(rng: &dyn rand::SecureRandom) -> Result<pkcs8::Document, error::Unspecified>` - Generates a new key pair and returns the key pair serialized as a
- `fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, error::KeyRejected>` - Constructs an Ed25519 key pair by parsing an unencrypted PKCS#8 v2
- `fn from_pkcs8_maybe_unchecked(pkcs8: &[u8]) -> Result<Self, error::KeyRejected>` - Constructs an Ed25519 key pair by parsing an unencrypted PKCS#8 v1 or v2
- `fn from_seed_and_public_key(seed: &[u8], public_key: &[u8]) -> Result<Self, error::KeyRejected>` - Constructs an Ed25519 key pair from the private key seed `seed` and its
- `fn from_seed_unchecked(seed: &[u8]) -> Result<Self, error::KeyRejected>` - Constructs a Ed25519 key pair from the private key seed `seed`.
- `fn sign(self: &Self, msg: &[u8]) -> signature::Signature` - Returns the signature of the message `msg`.

**Trait Implementations:**

- **KeyPair**
  - `fn public_key(self: &Self) -> &<Self as >::PublicKey`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`



## ring::ec::curve25519::ed25519::signing::PublicKey

*Struct*

**Tuple Struct**: `()`



