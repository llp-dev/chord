*[ring](../../../index.md) / [ec](../../index.md) / [suite_b](../index.md) / [ops](index.md)*

---

# Module `ops`

## Contents

- [Modules](#modules)
  - [`elem`](#elem)
  - [`p256`](#p256)
  - [`p384`](#p384)
- [Structs](#structs)
  - [`Point`](#point)
  - [`CommonOps`](#commonops)
  - [`Modulus`](#modulus)
  - [`PrivateKeyOps`](#privatekeyops)
  - [`PublicKeyOps`](#publickeyops)
  - [`ScalarOps`](#scalarops)
  - [`PublicScalarOps`](#publicscalarops)
  - [`PrivateScalarOps`](#privatescalarops)
  - [`Elem`](#elem)
- [Enums](#enums)
  - [`Q`](#q)
  - [`N`](#n)
- [Functions](#functions)
  - [`twin_mul_inefficient`](#twin-mul-inefficient)
  - [`elem_reduced_to_scalar`](#elem-reduced-to-scalar)
  - [`scalar_sum`](#scalar-sum)
  - [`elem_sqr_mul`](#elem-sqr-mul)
  - [`elem_sqr_mul_acc`](#elem-sqr-mul-acc)
  - [`elem_parse_big_endian_fixed_consttime`](#elem-parse-big-endian-fixed-consttime)
  - [`scalar_parse_big_endian_fixed_consttime`](#scalar-parse-big-endian-fixed-consttime)
  - [`scalar_parse_big_endian_variable`](#scalar-parse-big-endian-variable)
  - [`scalar_parse_big_endian_partially_reduced_variable_consttime`](#scalar-parse-big-endian-partially-reduced-variable-consttime)
  - [`parse_big_endian_fixed_consttime`](#parse-big-endian-fixed-consttime)
  - [`mul_mont`](#mul-mont)
  - [`binary_op`](#binary-op)
  - [`binary_op_assign`](#binary-op-assign)
  - [`unary_op`](#unary-op)
  - [`unary_op_assign`](#unary-op-assign)
  - [`unary_op_from_binary_op_assign`](#unary-op-from-binary-op-assign)
- [Type Aliases](#type-aliases)
  - [`Elem`](#elem)
  - [`Scalar`](#scalar)
- [Constants](#constants)
  - [`MAX_LIMBS`](#max-limbs)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`elem`](#elem) | mod |  |
| [`p256`](#p256) | mod |  |
| [`p384`](#p384) | mod |  |
| [`Point`](#point) | struct |  |
| [`CommonOps`](#commonops) | struct | Operations and values needed by all curve operations. |
| [`Modulus`](#modulus) | struct |  |
| [`PrivateKeyOps`](#privatekeyops) | struct | Operations on private keys, for ECDH and ECDSA signing. |
| [`PublicKeyOps`](#publickeyops) | struct | Operations and values needed by all operations on public keys (ECDH agreement and ECDSA verification). |
| [`ScalarOps`](#scalarops) | struct |  |
| [`PublicScalarOps`](#publicscalarops) | struct | Operations on public scalars needed by ECDSA signature verification. |
| [`PrivateScalarOps`](#privatescalarops) | struct |  |
| [`Elem`](#elem) | struct | Elements of ℤ/mℤ for some modulus *m*. |
| [`Q`](#q) | enum | Represents the (prime) order *q* of the curve's prime field. |
| [`N`](#n) | enum | Represents the prime order *n* of the curve's group. |
| [`twin_mul_inefficient`](#twin-mul-inefficient) | fn |  |
| [`elem_reduced_to_scalar`](#elem-reduced-to-scalar) | fn |  |
| [`scalar_sum`](#scalar-sum) | fn |  |
| [`elem_sqr_mul`](#elem-sqr-mul) | fn |  |
| [`elem_sqr_mul_acc`](#elem-sqr-mul-acc) | fn |  |
| [`elem_parse_big_endian_fixed_consttime`](#elem-parse-big-endian-fixed-consttime) | fn |  |
| [`scalar_parse_big_endian_fixed_consttime`](#scalar-parse-big-endian-fixed-consttime) | fn |  |
| [`scalar_parse_big_endian_variable`](#scalar-parse-big-endian-variable) | fn |  |
| [`scalar_parse_big_endian_partially_reduced_variable_consttime`](#scalar-parse-big-endian-partially-reduced-variable-consttime) | fn |  |
| [`parse_big_endian_fixed_consttime`](#parse-big-endian-fixed-consttime) | fn |  |
| [`mul_mont`](#mul-mont) | fn |  |
| [`binary_op`](#binary-op) | fn |  |
| [`binary_op_assign`](#binary-op-assign) | fn |  |
| [`unary_op`](#unary-op) | fn |  |
| [`unary_op_assign`](#unary-op-assign) | fn |  |
| [`unary_op_from_binary_op_assign`](#unary-op-from-binary-op-assign) | fn |  |
| [`Elem`](#elem) | type | A field element, i.e. an element of ℤ/qℤ for the curve's field modulus *q*. |
| [`Scalar`](#scalar) | type | A scalar. |
| [`MAX_LIMBS`](#max-limbs) | const |  |

## Modules

- [`elem`](elem/index.md)
- [`p256`](p256/index.md)
- [`p384`](p384/index.md)

## Structs

### `Point`

```rust
struct Point {
    xyz: [u64; 18],
}
```

#### Implementations

- <span id="point-new-at-infinity"></span>`fn new_at_infinity() -> Self`

### `CommonOps`

```rust
struct CommonOps {
    num_limbs: usize,
    q: Modulus,
    n: elem::Elem<Q, Unencoded>,
    pub a: elem::Elem<Q, R>,
    pub b: elem::Elem<Q, R>,
    elem_mul_mont: fn(*mut u64, *const u64, *const u64),
    elem_sqr_mont: fn(*mut u64, *const u64),
    point_add_jacobian_impl: fn(*mut u64, *const u64, *const u64),
}
```

Operations and values needed by all curve operations.

#### Implementations

- <span id="commonops-len"></span>`fn len(&self) -> usize`

- <span id="commonops-elem-add"></span>`fn elem_add<E: Encoding>(&self, a: &mut elem::Elem<Q, E>, b: &elem::Elem<Q, E>)` — [`Elem`](#elem), [`Q`](#q)

- <span id="commonops-elems-are-equal"></span>`fn elems_are_equal(&self, a: &elem::Elem<Q, R>, b: &elem::Elem<Q, R>) -> LimbMask` — [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r), [`LimbMask`](../../../limb/index.md#limbmask)

- <span id="commonops-elem-unencoded"></span>`fn elem_unencoded(&self, a: &elem::Elem<Q, R>) -> elem::Elem<Q, Unencoded>` — [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded)

- <span id="commonops-elem-mul"></span>`fn elem_mul(&self, a: &mut elem::Elem<Q, R>, b: &elem::Elem<Q, R>)` — [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r)

- <span id="commonops-elem-product"></span>`fn elem_product<EA: Encoding, EB: Encoding>(&self, a: &elem::Elem<Q, EA>, b: &elem::Elem<Q, EB>) -> elem::Elem<Q, <(EA, EB) as ProductEncoding>::Output>` — [`Elem`](#elem), [`Q`](#q), [`ProductEncoding`](../../../arithmetic/montgomery/index.md#productencoding)

- <span id="commonops-elem-square"></span>`fn elem_square(&self, a: &mut elem::Elem<Q, R>)` — [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r)

- <span id="commonops-elem-squared"></span>`fn elem_squared(&self, a: &elem::Elem<Q, R>) -> elem::Elem<Q, R>` — [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r)

- <span id="commonops-is-zero"></span>`fn is_zero<M, E: Encoding>(&self, a: &elem::Elem<M, E>) -> bool` — [`Elem`](#elem)

- <span id="commonops-elem-verify-is-not-zero"></span>`fn elem_verify_is_not_zero(&self, a: &elem::Elem<Q, R>) -> Result<(), error::Unspecified>` — [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r), [`Unspecified`](../../../error/index.md#unspecified)

- <span id="commonops-point-sum"></span>`fn point_sum(&self, a: &Point, b: &Point) -> Point` — [`Point`](#point)

- <span id="commonops-point-x"></span>`fn point_x(&self, p: &Point) -> elem::Elem<Q, R>` — [`Point`](#point), [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r)

- <span id="commonops-point-y"></span>`fn point_y(&self, p: &Point) -> elem::Elem<Q, R>` — [`Point`](#point), [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r)

- <span id="commonops-point-z"></span>`fn point_z(&self, p: &Point) -> elem::Elem<Q, R>` — [`Point`](#point), [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r)

### `Modulus`

```rust
struct Modulus {
    p: [u64; 6],
    rr: [u64; 6],
}
```

### `PrivateKeyOps`

```rust
struct PrivateKeyOps {
    pub common: &'static CommonOps,
    elem_inv_squared: fn(&elem::Elem<Q, R>) -> elem::Elem<Q, R>,
    point_mul_base_impl: fn(&elem::Elem<N, Unencoded>) -> Point,
    point_mul_impl: fn(*mut u64, *const u64, *const u64, *const u64),
}
```

Operations on private keys, for ECDH and ECDSA signing.

#### Implementations

- <span id="privatekeyops-leak-limbs"></span>`fn leak_limbs<'a>(&self, a: &'a elem::Elem<Q, Unencoded>) -> &'a [u64]` — [`Elem`](#elem), [`Q`](#q), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded)

- <span id="privatekeyops-point-mul-base"></span>`fn point_mul_base(&self, a: &elem::Elem<N, Unencoded>) -> Point` — [`Elem`](#elem), [`N`](#n), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded), [`Point`](#point)

- <span id="privatekeyops-point-mul"></span>`fn point_mul(&self, p_scalar: &elem::Elem<N, Unencoded>, (p_x, p_y): &(elem::Elem<Q, R>, elem::Elem<Q, R>)) -> Point` — [`Elem`](#elem), [`N`](#n), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r), [`Point`](#point)

- <span id="privatekeyops-elem-inverse-squared"></span>`fn elem_inverse_squared(&self, a: &elem::Elem<Q, R>) -> elem::Elem<Q, R>` — [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r)

### `PublicKeyOps`

```rust
struct PublicKeyOps {
    pub common: &'static CommonOps,
}
```

Operations and values needed by all operations on public keys (ECDH
agreement and ECDSA verification).

#### Implementations

- <span id="publickeyops-elem-parse"></span>`fn elem_parse(&self, input: &mut untrusted::Reader<'_>) -> Result<elem::Elem<Q, R>, error::Unspecified>` — [`Elem`](#elem), [`Q`](#q), [`R`](../../../arithmetic/montgomery/index.md#r), [`Unspecified`](../../../error/index.md#unspecified)

### `ScalarOps`

```rust
struct ScalarOps {
    pub common: &'static CommonOps,
    scalar_mul_mont: fn(*mut u64, *const u64, *const u64),
}
```

#### Implementations

- <span id="scalarops-scalar-bytes-len"></span>`fn scalar_bytes_len(&self) -> usize`

- <span id="scalarops-leak-limbs"></span>`fn leak_limbs<'s>(&self, s: &'s elem::Elem<N, Unencoded>) -> &'s [u64]` — [`Elem`](#elem), [`N`](#n), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded)

- <span id="scalarops-scalar-product"></span>`fn scalar_product<EA: Encoding, EB: Encoding>(&self, a: &elem::Elem<N, EA>, b: &elem::Elem<N, EB>) -> elem::Elem<N, <(EA, EB) as ProductEncoding>::Output>` — [`Elem`](#elem), [`N`](#n), [`ProductEncoding`](../../../arithmetic/montgomery/index.md#productencoding)

### `PublicScalarOps`

```rust
struct PublicScalarOps {
    pub scalar_ops: &'static ScalarOps,
    pub public_key_ops: &'static PublicKeyOps,
    pub twin_mul: fn(&elem::Elem<N, Unencoded>, &elem::Elem<N, Unencoded>, &(elem::Elem<Q, R>, elem::Elem<Q, R>)) -> Point,
    pub scalar_inv_to_mont_vartime: fn(&elem::Elem<N, Unencoded>) -> elem::Elem<N, R>,
    pub q_minus_n: elem::Elem<Q, Unencoded>,
}
```

Operations on public scalars needed by ECDSA signature verification.

#### Implementations

- <span id="publicscalarops-n"></span>`fn n(&self) -> &elem::Elem<Q, Unencoded>` — [`Elem`](#elem), [`Q`](#q), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded)

- <span id="publicscalarops-scalar-as-elem"></span>`fn scalar_as_elem(&self, a: &elem::Elem<N, Unencoded>) -> elem::Elem<Q, Unencoded>` — [`Elem`](#elem), [`N`](#n), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded), [`Q`](#q)

- <span id="publicscalarops-elem-equals-vartime"></span>`fn elem_equals_vartime(&self, a: &elem::Elem<Q, Unencoded>, b: &elem::Elem<Q, Unencoded>) -> bool` — [`Elem`](#elem), [`Q`](#q), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded)

- <span id="publicscalarops-elem-less-than"></span>`fn elem_less_than(&self, a: &elem::Elem<Q, Unencoded>, b: &elem::Elem<Q, Unencoded>) -> bool` — [`Elem`](#elem), [`Q`](#q), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded)

- <span id="publicscalarops-scalar-inv-to-mont-vartime"></span>`fn scalar_inv_to_mont_vartime(&self, s: &elem::Elem<N, Unencoded>) -> elem::Elem<N, R>` — [`Elem`](#elem), [`N`](#n), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded), [`R`](../../../arithmetic/montgomery/index.md#r)

### `PrivateScalarOps`

```rust
struct PrivateScalarOps {
    pub scalar_ops: &'static ScalarOps,
    oneRR_mod_n: elem::Elem<N, RR>,
    scalar_inv_to_mont: fn(elem::Elem<N, R>) -> elem::Elem<N, R>,
}
```

#### Implementations

- <span id="privatescalarops-to-mont"></span>`fn to_mont(&self, s: &elem::Elem<N, Unencoded>) -> elem::Elem<N, R>` — [`Elem`](#elem), [`N`](#n), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded), [`R`](../../../arithmetic/montgomery/index.md#r)

- <span id="privatescalarops-scalar-inv-to-mont"></span>`fn scalar_inv_to_mont(&self, a: &elem::Elem<N, Unencoded>) -> elem::Elem<N, R>` — [`Elem`](#elem), [`N`](#n), [`Unencoded`](../../../arithmetic/montgomery/index.md#unencoded), [`R`](../../../arithmetic/montgomery/index.md#r)

  Returns the modular inverse of `a` (mod `n`). Panics if `a` is zero.

### `Elem<M, E: Encoding>`

```rust
struct Elem<M, E: Encoding> {
    limbs: [u64; 6],
    m: core::marker::PhantomData<M>,
    encoding: core::marker::PhantomData<E>,
}
```

Elements of ℤ/mℤ for some modulus *m*. Elements are always fully reduced
with respect to *m*; i.e. the 0 <= x < m for every value x.

#### Fields

- **`m`**: `core::marker::PhantomData<M>`

  The modulus *m* for the ring ℤ/mℤ for which this element is a value.

- **`encoding`**: `core::marker::PhantomData<E>`

  The number of Montgomery factors that need to be canceled out from
  `value` to get the actual value.

#### Implementations

- <span id="elem-zero"></span>`fn zero() -> Self`

- <span id="elem-from-hex"></span>`const fn from_hex(hex: &str) -> Self`

#### Trait Implementations

##### `impl<M: clone::Clone, E: clone::Clone + Encoding> Clone for Elem<M, E>`

- <span id="elem-clone"></span>`fn clone(&self) -> Elem<M, E>` — [`Elem`](#elem)

##### `impl<M: marker::Copy, E: marker::Copy + Encoding> Copy for Elem<M, E>`

## Enums

### `Q`

```rust
enum Q {
}
```

Represents the (prime) order *q* of the curve's prime field.

#### Trait Implementations

##### `impl Clone for Q`

- <span id="q-clone"></span>`fn clone(&self) -> Q` — [`Q`](#q)

##### `impl Copy for Q`

### `N`

```rust
enum N {
}
```

Represents the prime order *n* of the curve's group.

#### Trait Implementations

##### `impl Clone for N`

- <span id="n-clone"></span>`fn clone(&self) -> N` — [`N`](#n)

##### `impl Copy for N`

## Functions

### `twin_mul_inefficient`

```rust
fn twin_mul_inefficient(ops: &PrivateKeyOps, g_scalar: &elem::Elem<N, Unencoded>, p_scalar: &elem::Elem<N, Unencoded>, p_xy: &(elem::Elem<Q, R>, elem::Elem<Q, R>)) -> Point
```

### `elem_reduced_to_scalar`

```rust
fn elem_reduced_to_scalar(ops: &CommonOps, elem: &elem::Elem<Q, Unencoded>) -> elem::Elem<N, Unencoded>
```

### `scalar_sum`

```rust
fn scalar_sum(ops: &CommonOps, a: &elem::Elem<N, Unencoded>, b: elem::Elem<N, Unencoded>) -> elem::Elem<N, Unencoded>
```

### `elem_sqr_mul`

```rust
fn elem_sqr_mul(ops: &CommonOps, a: &elem::Elem<Q, R>, squarings: usize, b: &elem::Elem<Q, R>) -> elem::Elem<Q, R>
```

### `elem_sqr_mul_acc`

```rust
fn elem_sqr_mul_acc(ops: &CommonOps, acc: &mut elem::Elem<Q, R>, squarings: usize, b: &elem::Elem<Q, R>)
```

### `elem_parse_big_endian_fixed_consttime`

```rust
fn elem_parse_big_endian_fixed_consttime(ops: &CommonOps, bytes: untrusted::Input<'_>) -> Result<elem::Elem<Q, Unencoded>, error::Unspecified>
```

### `scalar_parse_big_endian_fixed_consttime`

```rust
fn scalar_parse_big_endian_fixed_consttime(ops: &CommonOps, bytes: untrusted::Input<'_>) -> Result<elem::Elem<N, Unencoded>, error::Unspecified>
```

### `scalar_parse_big_endian_variable`

```rust
fn scalar_parse_big_endian_variable(ops: &CommonOps, allow_zero: AllowZero, bytes: untrusted::Input<'_>) -> Result<elem::Elem<N, Unencoded>, error::Unspecified>
```

### `scalar_parse_big_endian_partially_reduced_variable_consttime`

```rust
fn scalar_parse_big_endian_partially_reduced_variable_consttime(ops: &CommonOps, bytes: untrusted::Input<'_>) -> Result<elem::Elem<N, Unencoded>, error::Unspecified>
```

### `parse_big_endian_fixed_consttime`

```rust
fn parse_big_endian_fixed_consttime<M>(ops: &CommonOps, bytes: untrusted::Input<'_>, allow_zero: AllowZero, max_exclusive: &[u64]) -> Result<elem::Elem<M, Unencoded>, error::Unspecified>
```

### `mul_mont`

```rust
fn mul_mont<M, EA: Encoding, EB: Encoding>(f: fn(*mut u64, *const u64, *const u64), a: &Elem<M, EA>, b: &Elem<M, EB>) -> Elem<M, <(EA, EB) as ProductEncoding>::Output>
where
    (EA, EB): ProductEncoding
```

### `binary_op`

```rust
fn binary_op<M, EA: Encoding, EB: Encoding, ER: Encoding>(f: fn(*mut u64, *const u64, *const u64), a: &Elem<M, EA>, b: &Elem<M, EB>) -> Elem<M, ER>
```

### `binary_op_assign`

```rust
fn binary_op_assign<M, EA: Encoding, EB: Encoding>(f: fn(*mut u64, *const u64, *const u64), a: &mut Elem<M, EA>, b: &Elem<M, EB>)
```

### `unary_op`

```rust
fn unary_op<M, E: Encoding>(f: fn(*mut u64, *const u64), a: &Elem<M, E>) -> Elem<M, E>
```

### `unary_op_assign`

```rust
fn unary_op_assign<M, E: Encoding>(f: fn(*mut u64, *const u64), a: &mut Elem<M, E>)
```

### `unary_op_from_binary_op_assign`

```rust
fn unary_op_from_binary_op_assign<M, E: Encoding>(f: fn(*mut u64, *const u64, *const u64), a: &mut Elem<M, E>)
```

## Type Aliases

### `Elem<E>`

```rust
type Elem<E> = elem::Elem<Q, E>;
```

A field element, i.e. an element of ℤ/qℤ for the curve's field modulus
*q*.

### `Scalar<E>`

```rust
type Scalar<E> = elem::Elem<N, E>;
```

A scalar. Its value is in [0, n). Zero-valued scalars are forbidden in most
contexts.

## Constants

### `MAX_LIMBS`
```rust
const MAX_LIMBS: usize = 6usize;
```

