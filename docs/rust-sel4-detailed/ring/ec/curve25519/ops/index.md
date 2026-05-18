*[ring](../../../index.md) / [ec](../../index.md) / [curve25519](../index.md) / [ops](index.md)*

---

# Module `ops`

Elliptic curve operations on the birationally equivalent curves Curve25519
and Edwards25519.

## Contents

- [Structs](#structs)
  - [`MaskedScalar`](#maskedscalar)
  - [`Scalar`](#scalar)
  - [`Elem`](#elem)
  - [`T`](#t)
  - [`ExtPoint`](#extpoint)
  - [`Point`](#point)
- [Traits](#traits)
  - [`Encoding`](#encoding)
- [Functions](#functions)
  - [`x25519_fe_invert`](#x25519-fe-invert)
  - [`x25519_fe_isnegative`](#x25519-fe-isnegative)
  - [`x25519_fe_mul_ttt`](#x25519-fe-mul-ttt)
  - [`x25519_fe_neg`](#x25519-fe-neg)
  - [`x25519_fe_tobytes`](#x25519-fe-tobytes)
  - [`x25519_ge_frombytes_vartime`](#x25519-ge-frombytes-vartime)
  - [`encode_point`](#encode-point)
  - [`has_fe25519_adx`](#has-fe25519-adx)
- [Type Aliases](#type-aliases)
  - [`EncodedPoint`](#encodedpoint)
- [Constants](#constants)
  - [`SCALAR_LEN`](#scalar-len)
  - [`ELEM_LIMBS`](#elem-limbs)
  - [`ELEM_LEN`](#elem-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MaskedScalar`](#maskedscalar) | struct |  |
| [`Scalar`](#scalar) | struct |  |
| [`Elem`](#elem) | struct |  |
| [`T`](#t) | struct |  |
| [`ExtPoint`](#extpoint) | struct |  |
| [`Point`](#point) | struct |  |
| [`Encoding`](#encoding) | trait |  |
| [`x25519_fe_invert`](#x25519-fe-invert) | fn |  |
| [`x25519_fe_isnegative`](#x25519-fe-isnegative) | fn |  |
| [`x25519_fe_mul_ttt`](#x25519-fe-mul-ttt) | fn |  |
| [`x25519_fe_neg`](#x25519-fe-neg) | fn |  |
| [`x25519_fe_tobytes`](#x25519-fe-tobytes) | fn |  |
| [`x25519_ge_frombytes_vartime`](#x25519-ge-frombytes-vartime) | fn |  |
| [`encode_point`](#encode-point) | fn |  |
| [`has_fe25519_adx`](#has-fe25519-adx) | fn |  |
| [`EncodedPoint`](#encodedpoint) | type |  |
| [`SCALAR_LEN`](#scalar-len) | const |  |
| [`ELEM_LIMBS`](#elem-limbs) | const |  |
| [`ELEM_LEN`](#elem-len) | const |  |

## Structs

### `MaskedScalar`

```rust
struct MaskedScalar([u8; 32]);
```

#### Implementations

- <span id="maskedscalar-from-bytes-masked"></span>`fn from_bytes_masked(bytes: [u8; 32]) -> Self`

### `Scalar`

```rust
struct Scalar([u8; 32]);
```

#### Implementations

- <span id="scalar-from-bytes-checked"></span>`fn from_bytes_checked(bytes: [u8; 32]) -> Result<Self, error::Unspecified>` — [`Unspecified`](../../../error/index.md#unspecified)

- <span id="scalar-from-sha512-digest-reduced"></span>`fn from_sha512_digest_reduced(digest: digest::Digest) -> Self` — [`Digest`](../../../digest/index.md#digest)

### `Elem<E: Encoding>`

```rust
struct Elem<E: Encoding> {
    limbs: [u64; 5],
    encoding: core::marker::PhantomData<E>,
}
```

#### Implementations

- <span id="elem-zero"></span>`fn zero() -> Self`

### `T`

```rust
struct T;
```

#### Trait Implementations

##### `impl Encoding for T`

### `ExtPoint`

```rust
struct ExtPoint {
    x: Elem<T>,
    y: Elem<T>,
    z: Elem<T>,
    t: Elem<T>,
}
```

#### Implementations

- <span id="extpoint-from-scalarmult-base-consttime"></span>`fn from_scalarmult_base_consttime(scalar: &Scalar, cpu: cpu::Features) -> Self` — [`Scalar`](../scalar/index.md#scalar), [`Features`](../../../cpu/index.md#features)

- <span id="extpoint-from-encoded-point-vartime"></span>`fn from_encoded_point_vartime(encoded: &[u8; 32]) -> Result<Self, error::Unspecified>` — [`Unspecified`](../../../error/index.md#unspecified)

- <span id="extpoint-into-encoded-point"></span>`fn into_encoded_point(self) -> [u8; 32]`

- <span id="extpoint-invert-vartime"></span>`fn invert_vartime(&mut self)`

### `Point`

```rust
struct Point {
    x: Elem<T>,
    y: Elem<T>,
    z: Elem<T>,
}
```

#### Implementations

- <span id="point-new-at-infinity"></span>`fn new_at_infinity() -> Self`

- <span id="point-into-encoded-point"></span>`fn into_encoded_point(self) -> [u8; 32]`

## Traits

### `Encoding`

```rust
trait Encoding { ... }
```

#### Implementors

- [`T`](#t)

## Functions

### `x25519_fe_invert`

```rust
unsafe fn x25519_fe_invert(out: &mut Elem<T>, z: &Elem<T>)
```

### `x25519_fe_isnegative`

```rust
unsafe fn x25519_fe_isnegative(elem: &Elem<T>) -> u8
```

### `x25519_fe_mul_ttt`

```rust
unsafe fn x25519_fe_mul_ttt(h: &mut Elem<T>, f: &Elem<T>, g: &Elem<T>)
```

### `x25519_fe_neg`

```rust
unsafe fn x25519_fe_neg(f: &mut Elem<T>)
```

### `x25519_fe_tobytes`

```rust
unsafe fn x25519_fe_tobytes(bytes: &mut [u8; 32], elem: &Elem<T>)
```

### `x25519_ge_frombytes_vartime`

```rust
unsafe fn x25519_ge_frombytes_vartime(h: &mut ExtPoint, s: &[u8; 32]) -> bssl::Result
```

### `encode_point`

```rust
fn encode_point(x: Elem<T>, y: Elem<T>, z: Elem<T>) -> [u8; 32]
```

### `has_fe25519_adx`

```rust
fn has_fe25519_adx(cpu: cpu::Features) -> bool
```

## Type Aliases

### `EncodedPoint`

```rust
type EncodedPoint = [u8; 32];
```

## Constants

### `SCALAR_LEN`
```rust
const SCALAR_LEN: usize = 32usize;
```

### `ELEM_LIMBS`
```rust
const ELEM_LIMBS: usize = 5usize;
```

### `ELEM_LEN`
```rust
const ELEM_LEN: usize = 32usize;
```

