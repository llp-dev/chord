*[ring](../../../index.md) / [rsa](../../index.md) / [padding](../index.md) / [pkcs1](index.md)*

---

# Module `pkcs1`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PKCS1`](#pkcs1) | struct | PKCS#1 1.5 padding as described in [RFC 3447 Section 8.2]. |
| [`pkcs1_encode`](#pkcs1-encode) | fn |  |
| [`rsa_pkcs1_padding!`](#rsa-pkcs1-padding) | macro |  |
| [`pkcs1_digestinfo_prefix!`](#pkcs1-digestinfo-prefix) | macro |  |

## Structs

### `PKCS1`

```rust
struct PKCS1 {
    digest_alg: &'static digest::Algorithm,
    digestinfo_prefix: &'static [u8],
}
```

PKCS#1 1.5 padding as described in [RFC 3447 Section 8.2].

See "`RSA_PSS_*` Details\" in `ring::signature`'s module-level
documentation for more details.


#### Trait Implementations

##### `impl Debug for PKCS1`

- <span id="pkcs1-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Padding for PKCS1`

- <span id="pkcs1-padding-digest-alg"></span>`fn digest_alg(&self) -> &'static digest::Algorithm` — [`Algorithm`](../../../digest/index.md#algorithm)

##### `impl RsaEncoding for PKCS1`

##### `impl Sealed for PKCS1`

##### `impl Verification for PKCS1`

- <span id="pkcs1-verification-verify"></span>`fn verify(&self, m_hash: digest::Digest, m: &mut untrusted::Reader<'_>, mod_bits: bits::BitLength) -> Result<(), error::Unspecified>` — [`Digest`](../../../digest/index.md#digest), [`BitLength`](../../../bits/index.md#bitlength), [`Unspecified`](../../../error/index.md#unspecified)

## Functions

### `pkcs1_encode`

```rust
fn pkcs1_encode(pkcs1: &PKCS1, m_hash: digest::Digest, m_out: &mut [u8])
```

## Macros

### `rsa_pkcs1_padding!`

### `pkcs1_digestinfo_prefix!`

