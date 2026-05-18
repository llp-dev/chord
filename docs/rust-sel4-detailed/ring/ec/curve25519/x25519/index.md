*[ring](../../../index.md) / [ec](../../index.md) / [curve25519](../index.md) / [x25519](index.md)*

---

# Module `x25519`

X25519 Key agreement.

## Contents

- [Functions](#functions)
  - [`x25519_check_private_key_bytes`](#x25519-check-private-key-bytes)
  - [`x25519_generate_private_key`](#x25519-generate-private-key)
  - [`x25519_public_from_private`](#x25519-public-from-private)
  - [`x25519_ecdh`](#x25519-ecdh)
- [Type Aliases](#type-aliases)
  - [`PrivateKey`](#privatekey)
  - [`PublicKey`](#publickey)
  - [`SharedSecret`](#sharedsecret)
- [Constants](#constants)
  - [`ELEM_AND_SCALAR_LEN`](#elem-and-scalar-len)
  - [`PRIVATE_KEY_LEN`](#private-key-len)
  - [`PUBLIC_KEY_LEN`](#public-key-len)
  - [`SHARED_SECRET_LEN`](#shared-secret-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x25519_check_private_key_bytes`](#x25519-check-private-key-bytes) | fn |  |
| [`x25519_generate_private_key`](#x25519-generate-private-key) | fn |  |
| [`x25519_public_from_private`](#x25519-public-from-private) | fn |  |
| [`x25519_ecdh`](#x25519-ecdh) | fn |  |
| [`PrivateKey`](#privatekey) | type |  |
| [`PublicKey`](#publickey) | type |  |
| [`SharedSecret`](#sharedsecret) | type |  |
| [`ELEM_AND_SCALAR_LEN`](#elem-and-scalar-len) | const |  |
| [`PRIVATE_KEY_LEN`](#private-key-len) | const |  |
| [`PUBLIC_KEY_LEN`](#public-key-len) | const |  |
| [`SHARED_SECRET_LEN`](#shared-secret-len) | const |  |

## Functions

### `x25519_check_private_key_bytes`

```rust
fn x25519_check_private_key_bytes(bytes: &[u8]) -> Result<(), error::Unspecified>
```

### `x25519_generate_private_key`

```rust
fn x25519_generate_private_key(rng: &dyn rand::SecureRandom, out: &mut [u8]) -> Result<(), error::Unspecified>
```

### `x25519_public_from_private`

```rust
fn x25519_public_from_private(public_out: &mut [u8], private_key: &ec::Seed) -> Result<(), error::Unspecified>
```

### `x25519_ecdh`

```rust
fn x25519_ecdh(out: &mut [u8], my_private_key: &ec::Seed, peer_public_key: untrusted::Input<'_>) -> Result<(), error::Unspecified>
```

## Type Aliases

### `PrivateKey`

```rust
type PrivateKey = ops::MaskedScalar;
```

### `PublicKey`

```rust
type PublicKey = [u8; 32];
```

### `SharedSecret`

```rust
type SharedSecret = [u8; 32];
```

## Constants

### `ELEM_AND_SCALAR_LEN`
```rust
const ELEM_AND_SCALAR_LEN: usize = 32usize;
```

### `PRIVATE_KEY_LEN`
```rust
const PRIVATE_KEY_LEN: usize = 32usize;
```

### `PUBLIC_KEY_LEN`
```rust
const PUBLIC_KEY_LEN: usize = 32usize;
```

### `SHARED_SECRET_LEN`
```rust
const SHARED_SECRET_LEN: usize = 32usize;
```

