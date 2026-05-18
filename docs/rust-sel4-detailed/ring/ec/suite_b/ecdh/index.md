*[ring](../../../index.md) / [ec](../../index.md) / [suite_b](../index.md) / [ecdh](index.md)*

---

# Module `ecdh`

ECDH key agreement using the P-256 and P-384 curves.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`p256_ecdh`](#p256-ecdh) | fn |  |
| [`p384_ecdh`](#p384-ecdh) | fn |  |
| [`ecdh`](#ecdh) | fn |  |
| [`ecdh!`](#ecdh) | macro | A key agreement algorithm. |

## Functions

### `p256_ecdh`

```rust
fn p256_ecdh(out: &mut [u8], my_private_key: &ec::Seed, peer_public_key: untrusted::Input<'_>) -> Result<(), error::Unspecified>
```

### `p384_ecdh`

```rust
fn p384_ecdh(out: &mut [u8], my_private_key: &ec::Seed, peer_public_key: untrusted::Input<'_>) -> Result<(), error::Unspecified>
```

### `ecdh`

```rust
fn ecdh(private_key_ops: &PrivateKeyOps, public_key_ops: &PublicKeyOps, out: &mut [u8], my_private_key: &ec::Seed, peer_public_key: untrusted::Input<'_>) -> Result<(), error::Unspecified>
```

## Macros

### `ecdh!`

A key agreement algorithm.

