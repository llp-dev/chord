**ring > rsa > padding > pkcs1**

# Module: rsa::padding::pkcs1

## Contents

**Structs**

- [`PKCS1`](#pkcs1) - PKCS#1 1.5 padding as described in [RFC 3447 Section 8.2].

**Statics**

- [`RSA_PKCS1_SHA256`](#rsa_pkcs1_sha256) - PKCS#1 1.5 padding using SHA-256 for RSA signatures.
- [`RSA_PKCS1_SHA384`](#rsa_pkcs1_sha384) - PKCS#1 1.5 padding using SHA-384 for RSA signatures.
- [`RSA_PKCS1_SHA512`](#rsa_pkcs1_sha512) - PKCS#1 1.5 padding using SHA-512 for RSA signatures.

---

## ring::rsa::padding::pkcs1::PKCS1

*Struct*

PKCS#1 1.5 padding as described in [RFC 3447 Section 8.2].

See "`RSA_PSS_*` Details\" in `ring::signature`'s module-level
documentation for more details.

[RFC 3447 Section 8.2]: https://tools.ietf.org/html/rfc3447#section-8.2



## ring::rsa::padding::pkcs1::RSA_PKCS1_SHA256

*Static*

PKCS#1 1.5 padding using SHA-256 for RSA signatures.

```rust
static RSA_PKCS1_SHA256: PKCS1
```



## ring::rsa::padding::pkcs1::RSA_PKCS1_SHA384

*Static*

PKCS#1 1.5 padding using SHA-384 for RSA signatures.

```rust
static RSA_PKCS1_SHA384: PKCS1
```



## ring::rsa::padding::pkcs1::RSA_PKCS1_SHA512

*Static*

PKCS#1 1.5 padding using SHA-512 for RSA signatures.

```rust
static RSA_PKCS1_SHA512: PKCS1
```



