**rustls > crypto > ring > kx**

# Module: crypto::ring::kx

## Contents

**Statics**

- [`SECP256R1`](#secp256r1) - Ephemeral ECDH on secp256r1 (aka NIST-P256)
- [`SECP384R1`](#secp384r1) - Ephemeral ECDH on secp384r1 (aka NIST-P384)
- [`X25519`](#x25519) - Ephemeral ECDH on curve25519 (see RFC7748)

---

## rustls::crypto::ring::kx::SECP256R1

*Static*

Ephemeral ECDH on secp256r1 (aka NIST-P256)

```rust
static SECP256R1: &dyn SupportedKxGroup
```



## rustls::crypto::ring::kx::SECP384R1

*Static*

Ephemeral ECDH on secp384r1 (aka NIST-P384)

```rust
static SECP384R1: &dyn SupportedKxGroup
```



## rustls::crypto::ring::kx::X25519

*Static*

Ephemeral ECDH on curve25519 (see RFC7748)

```rust
static X25519: &dyn SupportedKxGroup
```



