**rustls > crypto > ring > sign**

# Module: crypto::ring::sign

## Contents

**Functions**

- [`any_ecdsa_type`](#any_ecdsa_type) - Parse `der` as any ECDSA key type, returning the first which works.
- [`any_eddsa_type`](#any_eddsa_type) - Parse `der` as any EdDSA key type, returning the first which works.
- [`any_supported_type`](#any_supported_type) - Parse `der` as any supported key encoding/type, returning

---

## rustls::crypto::ring::sign::any_ecdsa_type

*Function*

Parse `der` as any ECDSA key type, returning the first which works.

Both SEC1 (PEM section starting with 'BEGIN EC PRIVATE KEY') and PKCS8
(PEM section starting with 'BEGIN PRIVATE KEY') encodings are supported.

```rust
fn any_ecdsa_type(der: &pki_types::PrivateKeyDer) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error>
```



## rustls::crypto::ring::sign::any_eddsa_type

*Function*

Parse `der` as any EdDSA key type, returning the first which works.

Note that, at the time of writing, Ed25519 does not have wide support
in browsers.  It is also not supported by the WebPKI, because the
CA/Browser Forum Baseline Requirements do not support it for publicly
trusted certificates.

```rust
fn any_eddsa_type(der: &pki_types::PrivatePkcs8KeyDer) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error>
```



## rustls::crypto::ring::sign::any_supported_type

*Function*

Parse `der` as any supported key encoding/type, returning
the first which works.

```rust
fn any_supported_type(der: &pki_types::PrivateKeyDer) -> Result<alloc::sync::Arc<dyn SigningKey>, crate::error::Error>
```



