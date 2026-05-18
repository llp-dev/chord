*[ring](../../../index.md) / [ec](../../index.md) / [suite_b](../index.md) / [curve](index.md)*

---

# Module `curve`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`p256_check_private_key_bytes`](#p256-check-private-key-bytes) | fn |  |
| [`p256_generate_private_key`](#p256-generate-private-key) | fn |  |
| [`p256_public_from_private`](#p256-public-from-private) | fn |  |
| [`p384_check_private_key_bytes`](#p384-check-private-key-bytes) | fn |  |
| [`p384_generate_private_key`](#p384-generate-private-key) | fn |  |
| [`p384_public_from_private`](#p384-public-from-private) | fn |  |
| [`suite_b_curve!`](#suite-b-curve) | macro | A key agreement algorithm. |

## Functions

### `p256_check_private_key_bytes`

```rust
fn p256_check_private_key_bytes(bytes: &[u8]) -> Result<(), error::Unspecified>
```

### `p256_generate_private_key`

```rust
fn p256_generate_private_key(rng: &dyn rand::SecureRandom, out: &mut [u8]) -> Result<(), error::Unspecified>
```

### `p256_public_from_private`

```rust
fn p256_public_from_private(public_out: &mut [u8], private_key: &ec::Seed) -> Result<(), error::Unspecified>
```

### `p384_check_private_key_bytes`

```rust
fn p384_check_private_key_bytes(bytes: &[u8]) -> Result<(), error::Unspecified>
```

### `p384_generate_private_key`

```rust
fn p384_generate_private_key(rng: &dyn rand::SecureRandom, out: &mut [u8]) -> Result<(), error::Unspecified>
```

### `p384_public_from_private`

```rust
fn p384_public_from_private(public_out: &mut [u8], private_key: &ec::Seed) -> Result<(), error::Unspecified>
```

## Macros

### `suite_b_curve!`

A key agreement algorithm.

