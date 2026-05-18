*[ring](../../index.md) / [arithmetic](../index.md) / [bigint](index.md)*

---

# Module `bigint`

Multi-precision integers.

# Modular Arithmetic.

Modular arithmetic is done in finite commutative rings ℤ/mℤ for some
modulus *m*. We work in finite commutative rings instead of finite fields
because the RSA public modulus *n* is not prime, which means ℤ/nℤ contains
nonzero elements that have no multiplicative inverse, so ℤ/nℤ is not a
finite field.

In some calculations we need to deal with multiple rings at once. For
example, RSA private key operations operate in the rings ℤ/nℤ, ℤ/pℤ, and
ℤ/qℤ. Types and functions dealing with such rings are all parameterized
over a type `M` to ensure that we don't wrongly mix up the math, e.g. by
multiplying an element of ℤ/pℤ by an element of ℤ/qℤ modulo q. This follows
the "unit" pattern described in [Static checking of units in Servo].

`Elem` also uses the static unit checking pattern to statically track the
Montgomery factors that need to be canceled out in each value using it's
`E` parameter.


## Contents

- [Modules](#modules)
  - [`boxed_limbs`](#boxed-limbs)
  - [`modulus`](#modulus)
  - [`private_exponent`](#private-exponent)
- [Structs](#structs)
  - [`Elem`](#elem)
  - [`One`](#one)
- [Traits](#traits)
  - [`PublicModulus`](#publicmodulus)
- [Functions](#functions)
  - [`from_montgomery_amm`](#from-montgomery-amm)
  - [`elem_mul`](#elem-mul)
  - [`elem_double`](#elem-double)
  - [`elem_reduced_once`](#elem-reduced-once)
  - [`elem_reduced`](#elem-reduced)
  - [`elem_squared`](#elem-squared)
  - [`elem_widen`](#elem-widen)
  - [`elem_add`](#elem-add)
  - [`elem_sub`](#elem-sub)
  - [`elem_exp_vartime`](#elem-exp-vartime)
  - [`elem_exp_consttime`](#elem-exp-consttime)
  - [`verify_inverses_consttime`](#verify-inverses-consttime)
  - [`elem_verify_equal_consttime`](#elem-verify-equal-consttime)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`boxed_limbs`](#boxed-limbs) | mod |  |
| [`modulus`](#modulus) | mod |  |
| [`private_exponent`](#private-exponent) | mod |  |
| [`Elem`](#elem) | struct | Elements of ℤ/mℤ for some modulus *m*. |
| [`One`](#one) | struct |  |
| [`PublicModulus`](#publicmodulus) | trait |  |
| [`from_montgomery_amm`](#from-montgomery-amm) | fn | Does a Montgomery reduction on `limbs` assuming they are Montgomery-encoded ('R') and assuming they are the same size as `m`, but perhaps not reduced mod `m`. |
| [`elem_mul`](#elem-mul) | fn |  |
| [`elem_double`](#elem-double) | fn |  |
| [`elem_reduced_once`](#elem-reduced-once) | fn |  |
| [`elem_reduced`](#elem-reduced) | fn |  |
| [`elem_squared`](#elem-squared) | fn |  |
| [`elem_widen`](#elem-widen) | fn |  |
| [`elem_add`](#elem-add) | fn |  |
| [`elem_sub`](#elem-sub) | fn |  |
| [`elem_exp_vartime`](#elem-exp-vartime) | fn | Calculates base**exponent (mod m). |
| [`elem_exp_consttime`](#elem-exp-consttime) | fn |  |
| [`verify_inverses_consttime`](#verify-inverses-consttime) | fn | Verified a == b**-1 (mod m), i.e. a**-1 == b (mod m). |
| [`elem_verify_equal_consttime`](#elem-verify-equal-consttime) | fn |  |

## Modules

- [`boxed_limbs`](boxed_limbs/index.md)
- [`modulus`](modulus/index.md)
- [`private_exponent`](private_exponent/index.md)

## Structs

### `Elem<M, E>`

```rust
struct Elem<M, E> {
    limbs: self::boxed_limbs::BoxedLimbs<M>,
    encoding: core::marker::PhantomData<E>,
}
```

Elements of ℤ/mℤ for some modulus *m*.

#### Fields

- **`encoding`**: `core::marker::PhantomData<E>`

  The number of Montgomery factors that need to be canceled out from
  `value` to get the actual value.

#### Implementations

- <span id="elem-is-zero"></span>`fn is_zero(&self) -> bool`

#### Trait Implementations

##### `impl<M, E> AsRef for One<M, E>`

- <span id="one-asref-as-ref"></span>`fn as_ref(&self) -> &Elem<M, E>` — [`Elem`](#elem)

##### `impl<M, E> Clone for Elem<M, E>`

- <span id="elem-clone"></span>`fn clone(&self) -> Self`

### `One<M, E>`

```rust
struct One<M, E>(Elem<M, E>);
```

#### Implementations

- <span id="one-newrr"></span>`fn newRR(m: &Modulus<'_, M>) -> Self` — [`Modulus`](modulus/index.md#modulus)

#### Trait Implementations

##### `impl<M, E> AsRef for One<M, E>`

- <span id="one-asref-as-ref"></span>`fn as_ref(&self) -> &Elem<M, E>` — [`Elem`](#elem)

##### `impl<M: PublicModulus, E> Clone for One<M, E>`

- <span id="one-clone"></span>`fn clone(&self) -> Self`

## Traits

### `PublicModulus`

```rust
trait PublicModulus { ... }
```

#### Implementors

- [`N`](../../rsa/index.md#n)

## Functions

### `from_montgomery_amm`

```rust
fn from_montgomery_amm<M>(limbs: self::boxed_limbs::BoxedLimbs<M>, m: &self::modulus::Modulus<'_, M>) -> Elem<M, Unencoded>
```

Does a Montgomery reduction on `limbs` assuming they are Montgomery-encoded ('R') and assuming
they are the same size as `m`, but perhaps not reduced mod `m`. The result will be
fully reduced mod `m`.

### `elem_mul`

```rust
fn elem_mul<M, AF, BF>(a: &Elem<M, AF>, b: Elem<M, BF>, m: &self::modulus::Modulus<'_, M>) -> Elem<M, <(AF, BF) as ProductEncoding>::Output>
where
    (AF, BF): ProductEncoding
```

### `elem_double`

```rust
fn elem_double<M, AF>(r: &mut Elem<M, AF>, m: &self::modulus::Modulus<'_, M>)
```

### `elem_reduced_once`

```rust
fn elem_reduced_once<A, M>(a: &Elem<A, Unencoded>, m: &self::modulus::Modulus<'_, M>, other_modulus_len_bits: crate::bits::BitLength) -> Elem<M, Unencoded>
```

### `elem_reduced`

```rust
fn elem_reduced<Larger, Smaller>(a: &Elem<Larger, Unencoded>, m: &self::modulus::Modulus<'_, Smaller>, other_prime_len_bits: crate::bits::BitLength) -> Elem<Smaller, RInverse>
```

### `elem_squared`

```rust
fn elem_squared<M, E>(a: Elem<M, E>, m: &self::modulus::Modulus<'_, M>) -> Elem<M, <(E, E) as ProductEncoding>::Output>
where
    (E, E): ProductEncoding
```

### `elem_widen`

```rust
fn elem_widen<Larger, Smaller>(a: Elem<Smaller, Unencoded>, m: &self::modulus::Modulus<'_, Larger>, smaller_modulus_bits: crate::bits::BitLength) -> Result<Elem<Larger, Unencoded>, error::Unspecified>
```

### `elem_add`

```rust
fn elem_add<M, E>(a: Elem<M, E>, b: Elem<M, E>, m: &self::modulus::Modulus<'_, M>) -> Elem<M, E>
```

### `elem_sub`

```rust
fn elem_sub<M, E>(a: Elem<M, E>, b: &Elem<M, E>, m: &self::modulus::Modulus<'_, M>) -> Elem<M, E>
```

### `elem_exp_vartime`

```rust
fn elem_exp_vartime<M>(base: Elem<M, R>, exponent: core::num::NonZeroU64, m: &self::modulus::Modulus<'_, M>) -> Elem<M, R>
```

Calculates base**exponent (mod m).

The run time  is a function of the number of limbs in `m` and the bit
length and Hamming Weight of `exponent`. The bounds on `m` are pretty
obvious but the bounds on `exponent` are less obvious. Callers should
document the bounds they place on the maximum value and maximum Hamming
weight of `exponent`.

### `elem_exp_consttime`

```rust
fn elem_exp_consttime<M>(base: Elem<M, R>, exponent: &self::private_exponent::PrivateExponent, m: &self::modulus::Modulus<'_, M>) -> Result<Elem<M, Unencoded>, error::Unspecified>
```

### `verify_inverses_consttime`

```rust
fn verify_inverses_consttime<M>(a: &Elem<M, R>, b: Elem<M, Unencoded>, m: &self::modulus::Modulus<'_, M>) -> Result<(), error::Unspecified>
```

Verified a == b**-1 (mod m), i.e. a**-1 == b (mod m).

### `elem_verify_equal_consttime`

```rust
fn elem_verify_equal_consttime<M, E>(a: &Elem<M, E>, b: &Elem<M, E>) -> Result<(), error::Unspecified>
```

