**webpki > ring_algs**

# Module: ring_algs

## Contents

**Statics**

- [`ECDSA_P256_SHA256`](#ecdsa_p256_sha256) - ECDSA signatures using the P-256 curve and SHA-256.
- [`ECDSA_P256_SHA384`](#ecdsa_p256_sha384) - ECDSA signatures using the P-256 curve and SHA-384. Deprecated.
- [`ECDSA_P384_SHA256`](#ecdsa_p384_sha256) - ECDSA signatures using the P-384 curve and SHA-256. Deprecated.
- [`ECDSA_P384_SHA384`](#ecdsa_p384_sha384) - ECDSA signatures using the P-384 curve and SHA-384.
- [`ED25519`](#ed25519) - ED25519 signatures according to RFC 8410
- [`RSA_PKCS1_2048_8192_SHA256`](#rsa_pkcs1_2048_8192_sha256) - RSA PKCS#1 1.5 signatures using SHA-256 for keys of 2048-8192 bits.
- [`RSA_PKCS1_2048_8192_SHA256_ABSENT_PARAMS`](#rsa_pkcs1_2048_8192_sha256_absent_params) - RSA PKCS#1 1.5 signatures using SHA-256 for keys of 2048-8192 bits,
- [`RSA_PKCS1_2048_8192_SHA384`](#rsa_pkcs1_2048_8192_sha384) - RSA PKCS#1 1.5 signatures using SHA-384 for keys of 2048-8192 bits.
- [`RSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS`](#rsa_pkcs1_2048_8192_sha384_absent_params) - RSA PKCS#1 1.5 signatures using SHA-384 for keys of 2048-8192 bits,
- [`RSA_PKCS1_2048_8192_SHA512`](#rsa_pkcs1_2048_8192_sha512) - RSA PKCS#1 1.5 signatures using SHA-512 for keys of 2048-8192 bits.
- [`RSA_PKCS1_2048_8192_SHA512_ABSENT_PARAMS`](#rsa_pkcs1_2048_8192_sha512_absent_params) - RSA PKCS#1 1.5 signatures using SHA-512 for keys of 2048-8192 bits,
- [`RSA_PKCS1_3072_8192_SHA384`](#rsa_pkcs1_3072_8192_sha384) - RSA PKCS#1 1.5 signatures using SHA-384 for keys of 3072-8192 bits.
- [`RSA_PSS_2048_8192_SHA256_LEGACY_KEY`](#rsa_pss_2048_8192_sha256_legacy_key) - RSA PSS signatures using SHA-256 for keys of 2048-8192 bits and of
- [`RSA_PSS_2048_8192_SHA384_LEGACY_KEY`](#rsa_pss_2048_8192_sha384_legacy_key) - RSA PSS signatures using SHA-384 for keys of 2048-8192 bits and of
- [`RSA_PSS_2048_8192_SHA512_LEGACY_KEY`](#rsa_pss_2048_8192_sha512_legacy_key) - RSA PSS signatures using SHA-512 for keys of 2048-8192 bits and of

---

## webpki::ring_algs::ECDSA_P256_SHA256

*Static*

ECDSA signatures using the P-256 curve and SHA-256.

```rust
static ECDSA_P256_SHA256: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::ECDSA_P256_SHA384

*Static*

ECDSA signatures using the P-256 curve and SHA-384. Deprecated.

```rust
static ECDSA_P256_SHA384: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::ECDSA_P384_SHA256

*Static*

ECDSA signatures using the P-384 curve and SHA-256. Deprecated.

```rust
static ECDSA_P384_SHA256: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::ECDSA_P384_SHA384

*Static*

ECDSA signatures using the P-384 curve and SHA-384.

```rust
static ECDSA_P384_SHA384: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::ED25519

*Static*

ED25519 signatures according to RFC 8410

```rust
static ED25519: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PKCS1_2048_8192_SHA256

*Static*

RSA PKCS#1 1.5 signatures using SHA-256 for keys of 2048-8192 bits.

```rust
static RSA_PKCS1_2048_8192_SHA256: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PKCS1_2048_8192_SHA256_ABSENT_PARAMS

*Static*

RSA PKCS#1 1.5 signatures using SHA-256 for keys of 2048-8192 bits,
with illegally absent AlgorithmIdentifier parameters.

RFC4055 says on sha256WithRSAEncryption and company:

>   When any of these four object identifiers appears within an
>   AlgorithmIdentifier, the parameters MUST be NULL.  Implementations
>   MUST accept the parameters being absent as well as present.

This algorithm covers the absent case, [`RSA_PKCS1_2048_8192_SHA256`] covers
the present case.

```rust
static RSA_PKCS1_2048_8192_SHA256_ABSENT_PARAMS: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PKCS1_2048_8192_SHA384

*Static*

RSA PKCS#1 1.5 signatures using SHA-384 for keys of 2048-8192 bits.

```rust
static RSA_PKCS1_2048_8192_SHA384: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS

*Static*

RSA PKCS#1 1.5 signatures using SHA-384 for keys of 2048-8192 bits,
with illegally absent AlgorithmIdentifier parameters.

RFC4055 says on sha256WithRSAEncryption and company:

>   When any of these four object identifiers appears within an
>   AlgorithmIdentifier, the parameters MUST be NULL.  Implementations
>   MUST accept the parameters being absent as well as present.

This algorithm covers the absent case, [`RSA_PKCS1_2048_8192_SHA384`] covers
the present case.

```rust
static RSA_PKCS1_2048_8192_SHA384_ABSENT_PARAMS: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PKCS1_2048_8192_SHA512

*Static*

RSA PKCS#1 1.5 signatures using SHA-512 for keys of 2048-8192 bits.

```rust
static RSA_PKCS1_2048_8192_SHA512: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PKCS1_2048_8192_SHA512_ABSENT_PARAMS

*Static*

RSA PKCS#1 1.5 signatures using SHA-512 for keys of 2048-8192 bits,
with illegally absent AlgorithmIdentifier parameters.

RFC4055 says on sha256WithRSAEncryption and company:

>   When any of these four object identifiers appears within an
>   AlgorithmIdentifier, the parameters MUST be NULL.  Implementations
>   MUST accept the parameters being absent as well as present.

This algorithm covers the absent case, [`RSA_PKCS1_2048_8192_SHA512`] covers
the present case.

```rust
static RSA_PKCS1_2048_8192_SHA512_ABSENT_PARAMS: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PKCS1_3072_8192_SHA384

*Static*

RSA PKCS#1 1.5 signatures using SHA-384 for keys of 3072-8192 bits.

```rust
static RSA_PKCS1_3072_8192_SHA384: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PSS_2048_8192_SHA256_LEGACY_KEY

*Static*

RSA PSS signatures using SHA-256 for keys of 2048-8192 bits and of
type rsaEncryption; see [RFC 4055 Section 1.2].

[RFC 4055 Section 1.2]: https://tools.ietf.org/html/rfc4055#section-1.2

```rust
static RSA_PSS_2048_8192_SHA256_LEGACY_KEY: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PSS_2048_8192_SHA384_LEGACY_KEY

*Static*

RSA PSS signatures using SHA-384 for keys of 2048-8192 bits and of
type rsaEncryption; see [RFC 4055 Section 1.2].

[RFC 4055 Section 1.2]: https://tools.ietf.org/html/rfc4055#section-1.2

```rust
static RSA_PSS_2048_8192_SHA384_LEGACY_KEY: &dyn SignatureVerificationAlgorithm
```



## webpki::ring_algs::RSA_PSS_2048_8192_SHA512_LEGACY_KEY

*Static*

RSA PSS signatures using SHA-512 for keys of 2048-8192 bits and of
type rsaEncryption; see [RFC 4055 Section 1.2].

[RFC 4055 Section 1.2]: https://tools.ietf.org/html/rfc4055#section-1.2

```rust
static RSA_PSS_2048_8192_SHA512_LEGACY_KEY: &dyn SignatureVerificationAlgorithm
```



