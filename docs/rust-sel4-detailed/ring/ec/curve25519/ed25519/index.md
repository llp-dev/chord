*[ring](../../../index.md) / [ec](../../index.md) / [curve25519](../index.md) / [ed25519](index.md)*

---

# Module `ed25519`

EdDSA Signatures.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`signing`](#signing) | mod | EdDSA Signatures. |
| [`verification`](#verification) | mod | EdDSA Signatures. |
| [`eddsa_digest`](#eddsa-digest) | fn |  |
| [`ED25519_PUBLIC_KEY_LEN`](#ed25519-public-key-len) | const | The length of an Ed25519 public key. |

## Modules

- [`signing`](signing/index.md) — EdDSA Signatures.
- [`verification`](verification/index.md) — EdDSA Signatures.

## Functions

### `eddsa_digest`

```rust
fn eddsa_digest(signature_r: &[u8], public_key: &[u8], msg: &[u8]) -> digest::Digest
```

## Constants

### `ED25519_PUBLIC_KEY_LEN`
```rust
const ED25519_PUBLIC_KEY_LEN: usize = 32usize;
```

The length of an Ed25519 public key.

