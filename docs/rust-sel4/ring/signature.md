**ring > signature**

# Module: signature

## Contents

**Structs**

- [`Signature`](#signature) - A public key signature returned from a signing operation.
- [`UnparsedPublicKey`](#unparsedpublickey) - An unparsed, possibly malformed, public key for signature verification.

**Traits**

- [`KeyPair`](#keypair) - Key pairs for signing messages (private key and public key).
- [`VerificationAlgorithm`](#verificationalgorithm) - A signature verification algorithm.

**Type Aliases**

- [`RsaKeyPair`](#rsakeypair) - An RSA key pair, used for signing.

---

## ring::signature::KeyPair

*Trait*

Key pairs for signing messages (private key and public key).

**Methods:**

- `PublicKey`: The type of the public key.
- `public_key`: The public key for the key pair.



## ring::signature::RsaKeyPair

*Type Alias*: `crate::rsa::KeyPair`

An RSA key pair, used for signing.



## ring::signature::Signature

*Struct*

A public key signature returned from a signing operation.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Signature`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## ring::signature::UnparsedPublicKey

*Struct*

An unparsed, possibly malformed, public key for signature verification.

**Generic Parameters:**
- B

**Methods:**

- `fn new(algorithm: &'static dyn VerificationAlgorithm, bytes: B) -> Self` - Construct a new `UnparsedPublicKey`.
- `fn verify(self: &Self, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>` - Parses the public key and verifies `signature` is a valid signature of

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UnparsedPublicKey<B>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`



## ring::signature::VerificationAlgorithm

*Trait*

A signature verification algorithm.

**Methods:**

- `verify`: Verify the signature `signature` of message `msg` with the public key



