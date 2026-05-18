*[ring](../../../index.md) / [rsa](../../index.md) / [padding](../index.md) / [pss](index.md)*

---

# Module `pss`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PSS`](#pss) | struct | RSA PSS padding as described in [RFC 3447 Section 8.1]. |
| [`PSSMetrics`](#pssmetrics) | struct |  |
| [`pss_digest`](#pss-digest) | fn |  |
| [`rsa_pss_padding!`](#rsa-pss-padding) | macro |  |

## Structs

### `PSS`

```rust
struct PSS {
    digest_alg: &'static digest::Algorithm,
}
```

RSA PSS padding as described in [RFC 3447 Section 8.1].

See "`RSA_PSS_*` Details\" in `ring::signature`'s module-level
documentation for more details.


#### Trait Implementations

##### `impl Debug for PSS`

- <span id="pss-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Padding for PSS`

- <span id="pss-padding-digest-alg"></span>`fn digest_alg(&self) -> &'static digest::Algorithm` — [`Algorithm`](../../../digest/index.md#algorithm)

##### `impl RsaEncoding for PSS`

##### `impl Sealed for PSS`

##### `impl Verification for PSS`

- <span id="pss-verification-verify"></span>`fn verify(&self, m_hash: digest::Digest, m: &mut untrusted::Reader<'_>, mod_bits: bits::BitLength) -> Result<(), error::Unspecified>` — [`Digest`](../../../digest/index.md#digest), [`BitLength`](../../../bits/index.md#bitlength), [`Unspecified`](../../../error/index.md#unspecified)

### `PSSMetrics`

```rust
struct PSSMetrics {
    em_len: usize,
    db_len: usize,
    ps_len: usize,
    s_len: usize,
    h_len: usize,
    top_byte_mask: u8,
}
```

#### Implementations

- <span id="pssmetrics-new"></span>`fn new(digest_alg: &'static digest::Algorithm, mod_bits: bits::BitLength) -> Result<Self, error::Unspecified>` — [`Algorithm`](../../../digest/index.md#algorithm), [`BitLength`](../../../bits/index.md#bitlength), [`Unspecified`](../../../error/index.md#unspecified)

## Functions

### `pss_digest`

```rust
fn pss_digest(digest_alg: &'static digest::Algorithm, m_hash: digest::Digest, salt: &[u8]) -> digest::Digest
```

## Macros

### `rsa_pss_padding!`

