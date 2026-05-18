**webpki > rpk_entity**

# Module: rpk_entity

## Contents

**Structs**

- [`RawPublicKeyEntity`](#rawpublickeyentity) - A Raw Public Key, used for connections using raw public keys as specified

---

## webpki::rpk_entity::RawPublicKeyEntity

*Struct*

A Raw Public Key, used for connections using raw public keys as specified
in [RFC 7250](https://www.rfc-editor.org/rfc/rfc7250).

**Generic Parameters:**
- 'a

**Methods:**

- `fn verify_signature(self: &Self, signature_alg: &dyn SignatureVerificationAlgorithm, msg: &[u8], signature: &[u8]) -> Result<(), Error>` - Verifies the signature `signature` of message `msg` using a raw public key,

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryFrom**
  - `fn try_from(spki: &'a SubjectPublicKeyInfoDer<'a>) -> Result<Self, <Self as >::Error>` - Parse the ASN.1 DER-encoded SPKI encoding of the raw public key `spki`.



