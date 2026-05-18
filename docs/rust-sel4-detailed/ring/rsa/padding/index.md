*[ring](../../index.md) / [rsa](../index.md) / [padding](index.md)*

---

# Module `padding`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pkcs1`](#pkcs1) | mod |  |
| [`pss`](#pss) | mod |  |
| [`Padding`](#padding) | trait | Common features of both RSA padding encoding and RSA padding verification. |
| [`RsaEncoding`](#rsaencoding) | trait | An RSA signature encoding as described in [RFC 3447 Section 8]. |
| [`Verification`](#verification) | trait | Verification of an RSA signature encoding as described in [RFC 3447 Section 8]. |
| [`mgf1`](#mgf1) | fn |  |

## Modules

- [`pkcs1`](pkcs1/index.md)
- [`pss`](pss/index.md)

## Traits

### `Padding`

```rust
trait Padding: 'static + Sync + crate::sealed::Sealed + core::fmt::Debug { ... }
```

Common features of both RSA padding encoding and RSA padding verification.

#### Required Methods

- `fn digest_alg(&self) -> &'static digest::Algorithm`

#### Implementors

- [`PKCS1`](pkcs1/index.md#pkcs1)
- [`PSS`](pss/index.md#pss)

### `RsaEncoding`

```rust
trait RsaEncoding: Padding { ... }
```

An RSA signature encoding as described in [RFC 3447 Section 8].


#### Implementors

- [`PKCS1`](pkcs1/index.md#pkcs1)
- [`PSS`](pss/index.md#pss)

### `Verification`

```rust
trait Verification: Padding { ... }
```

Verification of an RSA signature encoding as described in
[RFC 3447 Section 8].


#### Required Methods

- `fn verify(&self, m_hash: digest::Digest, m: &mut untrusted::Reader<'_>, mod_bits: bits::BitLength) -> Result<(), error::Unspecified>`

#### Implementors

- [`PKCS1`](pkcs1/index.md#pkcs1)
- [`PSS`](pss/index.md#pss)

## Functions

### `mgf1`

```rust
fn mgf1(digest_alg: &'static digest::Algorithm, seed: &[u8], out: &mut [u8])
```

