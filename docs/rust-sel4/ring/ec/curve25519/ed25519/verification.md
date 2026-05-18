**ring > ec > curve25519 > ed25519 > verification**

# Module: ec::curve25519::ed25519::verification

## Contents

**Structs**

- [`EdDSAParameters`](#eddsaparameters) - Parameters for EdDSA signing and verification.

**Statics**

- [`ED25519`](#ed25519) - Verification of [Ed25519] signatures.

---

## ring::ec::curve25519::ed25519::verification::ED25519

*Static*

Verification of [Ed25519] signatures.

Ed25519 uses SHA-512 as the digest algorithm.

[Ed25519]: https://ed25519.cr.yp.to/

```rust
static ED25519: EdDSAParameters
```



## ring::ec::curve25519::ed25519::verification::EdDSAParameters

*Struct*

Parameters for EdDSA signing and verification.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`
- **VerificationAlgorithm**
  - `fn verify(self: &Self, public_key: untrusted::Input, msg: untrusted::Input, signature: untrusted::Input) -> Result<(), error::Unspecified>`



