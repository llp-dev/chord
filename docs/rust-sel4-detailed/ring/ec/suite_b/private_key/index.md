*[ring](../../../index.md) / [ec](../../index.md) / [suite_b](../index.md) / [private_key](index.md)*

---

# Module `private_key`

Functionality shared by operations on private keys (ECC keygen and
ECDSA signing).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`random_scalar`](#random-scalar) | fn | Generates a random scalar in the range [1, n). |
| [`generate_private_scalar_bytes`](#generate-private-scalar-bytes) | fn |  |
| [`private_key_as_scalar`](#private-key-as-scalar) | fn |  |
| [`check_scalar_big_endian_bytes`](#check-scalar-big-endian-bytes) | fn |  |
| [`scalar_from_big_endian_bytes`](#scalar-from-big-endian-bytes) | fn |  |
| [`public_from_private`](#public-from-private) | fn |  |
| [`affine_from_jacobian`](#affine-from-jacobian) | fn |  |
| [`big_endian_affine_from_jacobian`](#big-endian-affine-from-jacobian) | fn |  |

## Functions

### `random_scalar`

```rust
fn random_scalar(ops: &PrivateKeyOps, rng: &dyn rand::SecureRandom) -> Result<elem::Elem<N, Unencoded>, error::Unspecified>
```

Generates a random scalar in the range [1, n).

### `generate_private_scalar_bytes`

```rust
fn generate_private_scalar_bytes(ops: &PrivateKeyOps, rng: &dyn rand::SecureRandom, out: &mut [u8]) -> Result<(), error::Unspecified>
```

### `private_key_as_scalar`

```rust
fn private_key_as_scalar(ops: &PrivateKeyOps, private_key: &ec::Seed) -> elem::Elem<N, Unencoded>
```

### `check_scalar_big_endian_bytes`

```rust
fn check_scalar_big_endian_bytes(ops: &PrivateKeyOps, bytes: &[u8]) -> Result<(), error::Unspecified>
```

### `scalar_from_big_endian_bytes`

```rust
fn scalar_from_big_endian_bytes(ops: &PrivateKeyOps, bytes: &[u8]) -> Result<elem::Elem<N, Unencoded>, error::Unspecified>
```

### `public_from_private`

```rust
fn public_from_private(ops: &PrivateKeyOps, public_out: &mut [u8], my_private_key: &ec::Seed) -> Result<(), error::Unspecified>
```

### `affine_from_jacobian`

```rust
fn affine_from_jacobian(ops: &PrivateKeyOps, p: &Point) -> Result<(elem::Elem<Q, crate::arithmetic::montgomery::R>, elem::Elem<Q, crate::arithmetic::montgomery::R>), error::Unspecified>
```

### `big_endian_affine_from_jacobian`

```rust
fn big_endian_affine_from_jacobian(ops: &PrivateKeyOps, x_out: Option<&mut [u8]>, y_out: Option<&mut [u8]>, p: &Point) -> Result<(), error::Unspecified>
```

