*[rustls_pki_types](../index.md) / [alg_id](index.md)*

---

# Module `alg_id`

The PKIX [`AlgorithmIdentifier`](#algorithmidentifier) type, and common values.

If you need to use an [`AlgorithmIdentifier`](#algorithmidentifier) not defined here,
you can define it locally.

## Contents

- [Structs](#structs)
  - [`AlgorithmIdentifier`](#algorithmidentifier)
- [Constants](#constants)
  - [`ML_DSA_44`](#ml-dsa-44)
  - [`ML_DSA_65`](#ml-dsa-65)
  - [`ML_DSA_87`](#ml-dsa-87)
  - [`ECDSA_P256K1`](#ecdsa-p256k1)
  - [`ECDSA_P256`](#ecdsa-p256)
  - [`ECDSA_P384`](#ecdsa-p384)
  - [`ECDSA_P521`](#ecdsa-p521)
  - [`ECDSA_SHA256`](#ecdsa-sha256)
  - [`ECDSA_SHA384`](#ecdsa-sha384)
  - [`ECDSA_SHA512`](#ecdsa-sha512)
  - [`RSA_ENCRYPTION`](#rsa-encryption)
  - [`RSA_PKCS1_SHA256`](#rsa-pkcs1-sha256)
  - [`RSA_PKCS1_SHA384`](#rsa-pkcs1-sha384)
  - [`RSA_PKCS1_SHA512`](#rsa-pkcs1-sha512)
  - [`RSA_PSS_SHA256`](#rsa-pss-sha256)
  - [`RSA_PSS_SHA384`](#rsa-pss-sha384)
  - [`RSA_PSS_SHA512`](#rsa-pss-sha512)
  - [`ED25519`](#ed25519)
  - [`ED448`](#ed448)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AlgorithmIdentifier`](#algorithmidentifier) | struct | A DER encoding of the PKIX AlgorithmIdentifier type |
| [`ML_DSA_44`](#ml-dsa-44) | const | AlgorithmIdentifier for `id-ml-dsa-44`. |
| [`ML_DSA_65`](#ml-dsa-65) | const | AlgorithmIdentifier for `id-ml-dsa-65`. |
| [`ML_DSA_87`](#ml-dsa-87) | const | AlgorithmIdentifier for `id-ml-dsa-87`. |
| [`ECDSA_P256K1`](#ecdsa-p256k1) | const | AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256k1`. |
| [`ECDSA_P256`](#ecdsa-p256) | const | AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256r1`. |
| [`ECDSA_P384`](#ecdsa-p384) | const | AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp384r1`. |
| [`ECDSA_P521`](#ecdsa-p521) | const | AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp521r1`. |
| [`ECDSA_SHA256`](#ecdsa-sha256) | const | AlgorithmIdentifier for `ecdsa-with-SHA256`. |
| [`ECDSA_SHA384`](#ecdsa-sha384) | const | AlgorithmIdentifier for `ecdsa-with-SHA384`. |
| [`ECDSA_SHA512`](#ecdsa-sha512) | const | AlgorithmIdentifier for `ecdsa-with-SHA512`. |
| [`RSA_ENCRYPTION`](#rsa-encryption) | const | AlgorithmIdentifier for `rsaEncryption`. |
| [`RSA_PKCS1_SHA256`](#rsa-pkcs1-sha256) | const | AlgorithmIdentifier for `sha256WithRSAEncryption`. |
| [`RSA_PKCS1_SHA384`](#rsa-pkcs1-sha384) | const | AlgorithmIdentifier for `sha384WithRSAEncryption`. |
| [`RSA_PKCS1_SHA512`](#rsa-pkcs1-sha512) | const | AlgorithmIdentifier for `sha512WithRSAEncryption`. |
| [`RSA_PSS_SHA256`](#rsa-pss-sha256) | const | AlgorithmIdentifier for `rsassaPss` with |
| [`RSA_PSS_SHA384`](#rsa-pss-sha384) | const | AlgorithmIdentifier for `rsassaPss` with |
| [`RSA_PSS_SHA512`](#rsa-pss-sha512) | const | AlgorithmIdentifier for `rsassaPss` with |
| [`ED25519`](#ed25519) | const | AlgorithmIdentifier for `ED25519`. |
| [`ED448`](#ed448) | const | AlgorithmIdentifier for `ED448`. |

## Structs

### `AlgorithmIdentifier`

```rust
struct AlgorithmIdentifier(&'static [u8]);
```

A DER encoding of the PKIX AlgorithmIdentifier type:

```ASN.1
AlgorithmIdentifier  ::=  SEQUENCE  {
    algorithm               OBJECT IDENTIFIER,
    parameters              ANY DEFINED BY algorithm OPTIONAL  }
                               -- contains a value of the type
                               -- registered for use with the
                               -- algorithm object identifier value
```
(from <https://www.rfc-editor.org/rfc/rfc5280#section-4.1.1.2>)

The outer sequence encoding is *not included*, so this is the DER encoding
of an OID for `algorithm` plus the `parameters` value.

For example, this is the `rsaEncryption` algorithm (but prefer to use the constant
[`RSA_ENCRYPTION`](#rsa-encryption) instead):

```rust
let rsa_encryption = rustls_pki_types::AlgorithmIdentifier::from_slice(
    &[
        // algorithm: 1.2.840.113549.1.1.1
        0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01,
        // parameters: NULL
        0x05, 0x00
    ]
);
assert_eq!(rustls_pki_types::alg_id::RSA_ENCRYPTION, rsa_encryption);
```

Common values for this type are provided in this module.

#### Implementations

- <span id="algorithmidentifier-from-slice"></span>`const fn from_slice(bytes: &'static [u8]) -> Self`

  Makes a new `AlgorithmIdentifier` from a static octet slice.

  

  This does not validate the contents of the slice.

#### Trait Implementations

##### `impl AsRef for AlgorithmIdentifier`

- <span id="algorithmidentifier-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for AlgorithmIdentifier`

- <span id="algorithmidentifier-clone"></span>`fn clone(&self) -> AlgorithmIdentifier` — [`AlgorithmIdentifier`](#algorithmidentifier)

##### `impl Copy for AlgorithmIdentifier`

##### `impl Debug for AlgorithmIdentifier`

- <span id="algorithmidentifier-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for AlgorithmIdentifier`

- <span id="algorithmidentifier-deref-type-target"></span>`type Target = [u8]`

- <span id="algorithmidentifier-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Eq for AlgorithmIdentifier`

##### `impl PartialEq for AlgorithmIdentifier`

- <span id="algorithmidentifier-partialeq-eq"></span>`fn eq(&self, other: &AlgorithmIdentifier) -> bool` — [`AlgorithmIdentifier`](#algorithmidentifier)

##### `impl Receiver for AlgorithmIdentifier`

- <span id="algorithmidentifier-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for AlgorithmIdentifier`

## Constants

### `ML_DSA_44`
```rust
const ML_DSA_44: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ml-dsa-44`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.17 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>

### `ML_DSA_65`
```rust
const ML_DSA_65: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ml-dsa-65`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.18 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>

### `ML_DSA_87`
```rust
const ML_DSA_87: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ml-dsa-87`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.19 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>

### `ECDSA_P256K1`
```rust
const ECDSA_P256K1: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256k1`.

This is:

```text
ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
secp256k1
OBJECT_IDENTIFIER { 1.3.132.0.10 }
```

### `ECDSA_P256`
```rust
const ECDSA_P256: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256r1`.

This is:

```text
ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
secp256r1
OBJECT_IDENTIFIER { 1.2.840.10045.3.1.7 }
```

### `ECDSA_P384`
```rust
const ECDSA_P384: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp384r1`.

This is:

```text
ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
secp384r1
OBJECT_IDENTIFIER { 1.3.132.0.34 }
```

### `ECDSA_P521`
```rust
const ECDSA_P521: AlgorithmIdentifier;
```

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp521r1`.

This is:

```text
ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
secp521r1
OBJECT_IDENTIFIER { 1.3.132.0.35 }
```

### `ECDSA_SHA256`
```rust
const ECDSA_SHA256: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ecdsa-with-SHA256`.

This is:

```text
ecdsa-with-SHA256
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.2 }
```

### `ECDSA_SHA384`
```rust
const ECDSA_SHA384: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ecdsa-with-SHA384`.

This is:

```text
ecdsa-with-SHA384
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.3 }
```

### `ECDSA_SHA512`
```rust
const ECDSA_SHA512: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ecdsa-with-SHA512`.

This is:

```text
ecdsa-with-SHA512
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.4 }
```

### `RSA_ENCRYPTION`
```rust
const RSA_ENCRYPTION: AlgorithmIdentifier;
```

AlgorithmIdentifier for `rsaEncryption`.

This is:

```text
rsaEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.1 }
NULL {}
```

### `RSA_PKCS1_SHA256`
```rust
const RSA_PKCS1_SHA256: AlgorithmIdentifier;
```

AlgorithmIdentifier for `sha256WithRSAEncryption`.

This is:

```text
sha256WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.11 }
NULL {}
```

### `RSA_PKCS1_SHA384`
```rust
const RSA_PKCS1_SHA384: AlgorithmIdentifier;
```

AlgorithmIdentifier for `sha384WithRSAEncryption`.

This is:

```text
sha384WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.12 }
NULL {}
```

### `RSA_PKCS1_SHA512`
```rust
const RSA_PKCS1_SHA512: AlgorithmIdentifier;
```

AlgorithmIdentifier for `sha512WithRSAEncryption`.

This is:

```text
sha512WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.13 }
NULL {}
```

### `RSA_PSS_SHA256`
```rust
const RSA_PSS_SHA256: AlgorithmIdentifier;
```

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha256
- maskGenAlgorithm: mgf1 with sha256
- saltLength: 32

This is:

```text
rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  hashAlgorithm:
  [0] {
    SEQUENCE {
      sha256
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.1 }
      NULL {}
    }
  }
  maskGenAlgorithm:
  [1] {
    SEQUENCE {
      mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        sha256
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.1 }
        NULL {}
      }
    }
  }
  saltLength:
  [2] {
    INTEGER { 32 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

### `RSA_PSS_SHA384`
```rust
const RSA_PSS_SHA384: AlgorithmIdentifier;
```

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha384
- maskGenAlgorithm: mgf1 with sha384
- saltLength: 48

This is:

```text
rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  hashAlgorithm:
  [0] {
    SEQUENCE {
      sha384
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }
      NULL {}
    }
  }
  maskGenAlgorithm:
  [1] {
    SEQUENCE {
      mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        sha384
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }
        NULL {}
      }
    }
  }
  saltLength:
  [2] {
    INTEGER { 48 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

### `RSA_PSS_SHA512`
```rust
const RSA_PSS_SHA512: AlgorithmIdentifier;
```

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha512
- maskGenAlgorithm: mgf1 with sha512
- saltLength: 64

This is:

```text
rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  hashAlgorithm:
  [0] {
    SEQUENCE {
      sha512
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.3 }
      NULL {}
    }
  }
  maskGenAlgorithm:
  [1] {
    SEQUENCE {
      mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        sha512
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.3 }
        NULL {}
      }
    }
  }
  saltLength:
  [2] {
    INTEGER { 64 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.

### `ED25519`
```rust
const ED25519: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ED25519`.

This is:

```text
ed25519
OBJECT_IDENTIFIER { 1.3.101.112 }
```

### `ED448`
```rust
const ED448: AlgorithmIdentifier;
```

AlgorithmIdentifier for `ED448`.

This is:

```text
ed448
OBJECT_IDENTIFIER { 1.3.101.113 }
```

