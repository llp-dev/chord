**ring > ec > suite_b > ecdsa > verification**

# Module: ec::suite_b::ecdsa::verification

## Contents

**Structs**

- [`EcdsaVerificationAlgorithm`](#ecdsaverificationalgorithm) - An ECDSA verification algorithm.

**Statics**

- [`ECDSA_P256_SHA256_ASN1`](#ecdsa_p256_sha256_asn1) - Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve
- [`ECDSA_P256_SHA256_FIXED`](#ecdsa_p256_sha256_fixed) - Verification of fixed-length (PKCS#11 style) ECDSA signatures using the
- [`ECDSA_P256_SHA384_ASN1`](#ecdsa_p256_sha384_asn1) - *Not recommended*. Verification of ASN.1 DER-encoded ECDSA signatures using
- [`ECDSA_P384_SHA256_ASN1`](#ecdsa_p384_sha256_asn1) - *Not recommended*. Verification of ASN.1 DER-encoded ECDSA signatures using
- [`ECDSA_P384_SHA384_ASN1`](#ecdsa_p384_sha384_asn1) - Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve
- [`ECDSA_P384_SHA384_FIXED`](#ecdsa_p384_sha384_fixed) - Verification of fixed-length (PKCS#11 style) ECDSA signatures using the

---

## ring::ec::suite_b::ecdsa::verification::ECDSA_P256_SHA256_ASN1

*Static*

Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve
and SHA-256.

See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
documentation for more details.

```rust
static ECDSA_P256_SHA256_ASN1: EcdsaVerificationAlgorithm
```



## ring::ec::suite_b::ecdsa::verification::ECDSA_P256_SHA256_FIXED

*Static*

Verification of fixed-length (PKCS#11 style) ECDSA signatures using the
P-256 curve and SHA-256.

See "`ECDSA_*_FIXED` Details" in `ring::signature`'s module-level
documentation for more details.

```rust
static ECDSA_P256_SHA256_FIXED: EcdsaVerificationAlgorithm
```



## ring::ec::suite_b::ecdsa::verification::ECDSA_P256_SHA384_ASN1

*Static*

*Not recommended*. Verification of ASN.1 DER-encoded ECDSA signatures using
the P-256 curve and SHA-384.

In most situations, P-256 should be used only with SHA-256 and P-384
should be used only with SHA-384. However, in some cases, particularly TLS
on the web, it is necessary to support P-256 with SHA-384 for compatibility
with widely-deployed implementations that do not follow these guidelines.

See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
documentation for more details.

```rust
static ECDSA_P256_SHA384_ASN1: EcdsaVerificationAlgorithm
```



## ring::ec::suite_b::ecdsa::verification::ECDSA_P384_SHA256_ASN1

*Static*

*Not recommended*. Verification of ASN.1 DER-encoded ECDSA signatures using
the P-384 curve and SHA-256.

In most situations, P-256 should be used only with SHA-256 and P-384
should be used only with SHA-384. However, in some cases, particularly TLS
on the web, it is necessary to support P-256 with SHA-384 for compatibility
with widely-deployed implementations that do not follow these guidelines.

See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
documentation for more details.

```rust
static ECDSA_P384_SHA256_ASN1: EcdsaVerificationAlgorithm
```



## ring::ec::suite_b::ecdsa::verification::ECDSA_P384_SHA384_ASN1

*Static*

Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve
and SHA-384.

See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
documentation for more details.

```rust
static ECDSA_P384_SHA384_ASN1: EcdsaVerificationAlgorithm
```



## ring::ec::suite_b::ecdsa::verification::ECDSA_P384_SHA384_FIXED

*Static*

Verification of fixed-length (PKCS#11 style) ECDSA signatures using the
P-384 curve and SHA-384.

See "`ECDSA_*_FIXED` Details" in `ring::signature`'s module-level
documentation for more details.

```rust
static ECDSA_P384_SHA384_FIXED: EcdsaVerificationAlgorithm
```



## ring::ec::suite_b::ecdsa::verification::EcdsaVerificationAlgorithm

*Struct*

An ECDSA verification algorithm.

**Trait Implementations:**

- **VerificationAlgorithm**
  - `fn verify(self: &Self, public_key: untrusted::Input, msg: untrusted::Input, signature: untrusted::Input) -> Result<(), error::Unspecified>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`



