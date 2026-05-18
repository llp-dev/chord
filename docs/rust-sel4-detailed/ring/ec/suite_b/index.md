*[ring](../../index.md) / [ec](../index.md) / [suite_b](index.md)*

---

# Module `suite_b`

Elliptic curve operations on P-256 & P-384.

## Contents

- [Modules](#modules)
  - [`curve`](#curve)
  - [`ecdh`](#ecdh)
  - [`ecdsa`](#ecdsa)
  - [`ops`](#ops)
  - [`private_key`](#private-key)
  - [`public_key`](#public-key)
- [Functions](#functions)
  - [`verify_affine_point_is_on_the_curve`](#verify-affine-point-is-on-the-curve)
  - [`verify_jacobian_point_is_on_the_curve`](#verify-jacobian-point-is-on-the-curve)
  - [`verify_affine_point_is_on_the_curve_scaled`](#verify-affine-point-is-on-the-curve-scaled)
  - [`key_pair_from_pkcs8`](#key-pair-from-pkcs8)
  - [`key_pair_from_pkcs8_`](#key-pair-from-pkcs8)
  - [`key_pair_from_bytes`](#key-pair-from-bytes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`curve`](#curve) | mod |  |
| [`ecdh`](#ecdh) | mod | ECDH key agreement using the P-256 and P-384 curves. |
| [`ecdsa`](#ecdsa) | mod |  |
| [`ops`](#ops) | mod |  |
| [`private_key`](#private-key) | mod | Functionality shared by operations on private keys (ECC keygen and ECDSA signing). |
| [`public_key`](#public-key) | mod | Functionality shared by operations on public keys (ECDSA verification and ECDH agreement). |
| [`verify_affine_point_is_on_the_curve`](#verify-affine-point-is-on-the-curve) | fn |  |
| [`verify_jacobian_point_is_on_the_curve`](#verify-jacobian-point-is-on-the-curve) | fn |  |
| [`verify_affine_point_is_on_the_curve_scaled`](#verify-affine-point-is-on-the-curve-scaled) | fn |  |
| [`key_pair_from_pkcs8`](#key-pair-from-pkcs8) | fn |  |
| [`key_pair_from_pkcs8_`](#key-pair-from-pkcs8) | fn |  |
| [`key_pair_from_bytes`](#key-pair-from-bytes) | fn |  |

## Modules

- [`curve`](curve/index.md)
- [`ecdh`](ecdh/index.md) — ECDH key agreement using the P-256 and P-384 curves.
- [`ecdsa`](ecdsa/index.md)
- [`ops`](ops/index.md)
- [`private_key`](private_key/index.md) — Functionality shared by operations on private keys (ECC keygen and
- [`public_key`](public_key/index.md) — Functionality shared by operations on public keys (ECDSA verification and

## Functions

### `verify_affine_point_is_on_the_curve`

```rust
fn verify_affine_point_is_on_the_curve(ops: &CommonOps, (x, y): (&elem::Elem<Q, R>, &elem::Elem<Q, R>)) -> Result<(), error::Unspecified>
```

### `verify_jacobian_point_is_on_the_curve`

```rust
fn verify_jacobian_point_is_on_the_curve(ops: &CommonOps, p: &Point) -> Result<elem::Elem<Q, R>, error::Unspecified>
```

### `verify_affine_point_is_on_the_curve_scaled`

```rust
fn verify_affine_point_is_on_the_curve_scaled(ops: &CommonOps, (x, y): (&elem::Elem<Q, R>, &elem::Elem<Q, R>), a_scaled: &elem::Elem<Q, R>, b_scaled: &elem::Elem<Q, R>) -> Result<(), error::Unspecified>
```

### `key_pair_from_pkcs8`

```rust
fn key_pair_from_pkcs8(curve: &'static ec::Curve, template: &pkcs8::Template, input: untrusted::Input<'_>, cpu_features: cpu::Features) -> Result<ec::KeyPair, error::KeyRejected>
```

### `key_pair_from_pkcs8_`

```rust
fn key_pair_from_pkcs8_<'a>(template: &pkcs8::Template, input: &mut untrusted::Reader<'a>) -> Result<(untrusted::Input<'a>, untrusted::Input<'a>), error::KeyRejected>
```

### `key_pair_from_bytes`

```rust
fn key_pair_from_bytes(curve: &'static ec::Curve, private_key_bytes: untrusted::Input<'_>, public_key_bytes: untrusted::Input<'_>, cpu_features: cpu::Features) -> Result<ec::KeyPair, error::KeyRejected>
```

