*[ring](../../../index.md) / [ec](../../index.md) / [curve25519](../index.md) / [scalar](index.md)*

---

# Module `scalar`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Scalar`](#scalar) | struct |  |
| [`MaskedScalar`](#maskedscalar) | struct |  |
| [`UnreducedScalar`](#unreducedscalar) | type |  |
| [`SCALAR_LEN`](#scalar-len) | const |  |
| [`UNREDUCED_SCALAR_LEN`](#unreduced-scalar-len) | const |  |

## Structs

### `Scalar`

```rust
struct Scalar([u8; 32]);
```

#### Implementations

- <span id="scalar-from-bytes-checked"></span>`fn from_bytes_checked(bytes: [u8; 32]) -> Result<Self, error::Unspecified>` — [`Unspecified`](../../../error/index.md#unspecified)

- <span id="scalar-from-sha512-digest-reduced"></span>`fn from_sha512_digest_reduced(digest: digest::Digest) -> Self` — [`Digest`](../../../digest/index.md#digest)

### `MaskedScalar`

```rust
struct MaskedScalar([u8; 32]);
```

#### Implementations

- <span id="maskedscalar-from-bytes-masked"></span>`fn from_bytes_masked(bytes: [u8; 32]) -> Self`

## Type Aliases

### `UnreducedScalar`

```rust
type UnreducedScalar = [u8; 64];
```

## Constants

### `SCALAR_LEN`
```rust
const SCALAR_LEN: usize = 32usize;
```

### `UNREDUCED_SCALAR_LEN`
```rust
const UNREDUCED_SCALAR_LEN: usize = 64usize;
```

