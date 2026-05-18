**rustls_pki_types > alg_id**

# Module: alg_id

## Contents

**Structs**

- [`AlgorithmIdentifier`](#algorithmidentifier) - A DER encoding of the PKIX AlgorithmIdentifier type:

**Constants**

- [`ECDSA_P256`](#ecdsa_p256) - AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256r1`.
- [`ECDSA_P256K1`](#ecdsa_p256k1) - AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256k1`.
- [`ECDSA_P384`](#ecdsa_p384) - AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp384r1`.
- [`ECDSA_P521`](#ecdsa_p521) - AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp521r1`.
- [`ECDSA_SHA256`](#ecdsa_sha256) - AlgorithmIdentifier for `ecdsa-with-SHA256`.
- [`ECDSA_SHA384`](#ecdsa_sha384) - AlgorithmIdentifier for `ecdsa-with-SHA384`.
- [`ECDSA_SHA512`](#ecdsa_sha512) - AlgorithmIdentifier for `ecdsa-with-SHA512`.
- [`ED25519`](#ed25519) - AlgorithmIdentifier for `ED25519`.
- [`ED448`](#ed448) - AlgorithmIdentifier for `ED448`.
- [`ML_DSA_44`](#ml_dsa_44) - AlgorithmIdentifier for `id-ml-dsa-44`.
- [`ML_DSA_65`](#ml_dsa_65) - AlgorithmIdentifier for `id-ml-dsa-65`.
- [`ML_DSA_87`](#ml_dsa_87) - AlgorithmIdentifier for `id-ml-dsa-87`.
- [`RSA_ENCRYPTION`](#rsa_encryption) - AlgorithmIdentifier for `rsaEncryption`.
- [`RSA_PKCS1_SHA256`](#rsa_pkcs1_sha256) - AlgorithmIdentifier for `sha256WithRSAEncryption`.
- [`RSA_PKCS1_SHA384`](#rsa_pkcs1_sha384) - AlgorithmIdentifier for `sha384WithRSAEncryption`.
- [`RSA_PKCS1_SHA512`](#rsa_pkcs1_sha512) - AlgorithmIdentifier for `sha512WithRSAEncryption`.
- [`RSA_PSS_SHA256`](#rsa_pss_sha256) - AlgorithmIdentifier for `rsassaPss` with:
- [`RSA_PSS_SHA384`](#rsa_pss_sha384) - AlgorithmIdentifier for `rsassaPss` with:
- [`RSA_PSS_SHA512`](#rsa_pss_sha512) - AlgorithmIdentifier for `rsassaPss` with:

---

## rustls_pki_types::alg_id::AlgorithmIdentifier

*Struct*

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
[`RSA_ENCRYPTION`] instead):

```
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

**Tuple Struct**: `()`

**Methods:**

- `fn from_slice(bytes: &'static [u8]) -> Self` - Makes a new `AlgorithmIdentifier` from a static octet slice.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &AlgorithmIdentifier) -> bool`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AlgorithmIdentifier`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## rustls_pki_types::alg_id::ECDSA_P256

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp256r1
OBJECT_IDENTIFIER { 1.2.840.10045.3.1.7 }
```



## rustls_pki_types::alg_id::ECDSA_P256K1

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp256k1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp256k1
OBJECT_IDENTIFIER { 1.3.132.0.10 }
```



## rustls_pki_types::alg_id::ECDSA_P384

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp384r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp384r1
OBJECT_IDENTIFIER { 1.3.132.0.34 }
```



## rustls_pki_types::alg_id::ECDSA_P521

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `id-ecPublicKey` with named curve `secp521r1`.

This is:

```text
# ecPublicKey
OBJECT_IDENTIFIER { 1.2.840.10045.2.1 }
# secp521r1
OBJECT_IDENTIFIER { 1.3.132.0.35 }
```



## rustls_pki_types::alg_id::ECDSA_SHA256

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `ecdsa-with-SHA256`.

This is:

```text
# ecdsa-with-SHA256
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.2 }
```



## rustls_pki_types::alg_id::ECDSA_SHA384

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `ecdsa-with-SHA384`.

This is:

```text
# ecdsa-with-SHA384
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.3 }
```



## rustls_pki_types::alg_id::ECDSA_SHA512

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `ecdsa-with-SHA512`.

This is:

```text
# ecdsa-with-SHA512
OBJECT_IDENTIFIER { 1.2.840.10045.4.3.4 }
```



## rustls_pki_types::alg_id::ED25519

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `ED25519`.

This is:

```text
# ed25519
OBJECT_IDENTIFIER { 1.3.101.112 }
```



## rustls_pki_types::alg_id::ED448

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `ED448`.

This is:

```text
# ed448
OBJECT_IDENTIFIER { 1.3.101.113 }
```



## rustls_pki_types::alg_id::ML_DSA_44

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `id-ml-dsa-44`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.17 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>



## rustls_pki_types::alg_id::ML_DSA_65

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `id-ml-dsa-65`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.18 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>



## rustls_pki_types::alg_id::ML_DSA_87

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `id-ml-dsa-87`.

This is:

```text
OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.3.19 }
```

<https://www.ietf.org/archive/id/draft-ietf-lamps-dilithium-certificates-07.html#name-identifiers>



## rustls_pki_types::alg_id::RSA_ENCRYPTION

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `rsaEncryption`.

This is:

```text
# rsaEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.1 }
NULL {}
```



## rustls_pki_types::alg_id::RSA_PKCS1_SHA256

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `sha256WithRSAEncryption`.

This is:

```text
# sha256WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.11 }
NULL {}
```



## rustls_pki_types::alg_id::RSA_PKCS1_SHA384

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `sha384WithRSAEncryption`.

This is:

```text
# sha384WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.12 }
NULL {}
```



## rustls_pki_types::alg_id::RSA_PKCS1_SHA512

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `sha512WithRSAEncryption`.

This is:

```text
# sha512WithRSAEncryption
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.13 }
NULL {}
```



## rustls_pki_types::alg_id::RSA_PSS_SHA256

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha256
- maskGenAlgorithm: mgf1 with sha256
- saltLength: 32

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha256
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.1 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha256
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.1 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 32 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.



## rustls_pki_types::alg_id::RSA_PSS_SHA384

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha384
- maskGenAlgorithm: mgf1 with sha384
- saltLength: 48

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha384
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha384
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.2 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 48 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.



## rustls_pki_types::alg_id::RSA_PSS_SHA512

*Constant*: `AlgorithmIdentifier`

AlgorithmIdentifier for `rsassaPss` with:

- hashAlgorithm: sha512
- maskGenAlgorithm: mgf1 with sha512
- saltLength: 64

This is:

```text
# rsassa-pss
OBJECT_IDENTIFIER { 1.2.840.113549.1.1.10 }
SEQUENCE {
  # hashAlgorithm:
  [0] {
    SEQUENCE {
      # sha512
      OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.3 }
      NULL {}
    }
  }
  # maskGenAlgorithm:
  [1] {
    SEQUENCE {
      # mgf1
      OBJECT_IDENTIFIER { 1.2.840.113549.1.1.8 }
      SEQUENCE {
        # sha512
        OBJECT_IDENTIFIER { 2.16.840.1.101.3.4.2.3 }
        NULL {}
      }
    }
  }
  # saltLength:
  [2] {
    INTEGER { 64 }
  }
}
```

See <https://datatracker.ietf.org/doc/html/rfc4055#section-3.1> for
the meaning of the context-specific tags.



