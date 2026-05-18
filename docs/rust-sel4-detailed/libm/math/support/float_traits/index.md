*[libm](../../../index.md) / [math](../../index.md) / [support](../index.md) / [float_traits](index.md)*

---

# Module `float_traits`

## Contents

- [Traits](#traits)
  - [`Float`](#float)
  - [`DFloat`](#dfloat)
  - [`HFloat`](#hfloat)
- [Functions](#functions)
  - [`f32_from_bits`](#f32-from-bits)
  - [`f32_to_bits`](#f32-to-bits)
  - [`f64_from_bits`](#f64-from-bits)
  - [`f64_to_bits`](#f64-to-bits)
- [Type Aliases](#type-aliases)
  - [`IntTy`](#intty)
- [Macros](#macros)
  - [`float_impl!`](#float-impl)
  - [`impl_d_float!`](#impl-d-float)
  - [`impl_h_float!`](#impl-h-float)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Float`](#float) | trait | Trait for some basic operations on floats |
| [`DFloat`](#dfloat) | trait | Trait for floats twice the bit width of another integer. |
| [`HFloat`](#hfloat) | trait | Trait for floats half the bit width of another float. |
| [`f32_from_bits`](#f32-from-bits) | fn | `f32::from_bits` |
| [`f32_to_bits`](#f32-to-bits) | fn | `f32::to_bits` |
| [`f64_from_bits`](#f64-from-bits) | fn | `f64::from_bits` |
| [`f64_to_bits`](#f64-to-bits) | fn | `f64::to_bits` |
| [`IntTy`](#intty) | type | Access the associated `Int` type from a float (helper to avoid ambiguous associated types). |
| [`float_impl!`](#float-impl) | macro |  |
| [`impl_d_float!`](#impl-d-float) | macro |  |
| [`impl_h_float!`](#impl-h-float) | macro |  |

## Traits

### `Float`

```rust
trait Float: Copy + fmt::Debug + PartialEq + PartialOrd + ops::AddAssign + ops::MulAssign + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> + ops::Rem<Output = Self> + ops::Neg<Output = Self> + 'static { ... }
```

Trait for some basic operations on floats

#### Associated Types

- `type Int: 1`

- `type SignedInt: 3`

#### Associated Constants

- `const ZERO: Self`

- `const NEG_ZERO: Self`

- `const ONE: Self`

- `const NEG_ONE: Self`

- `const INFINITY: Self`

- `const NEG_INFINITY: Self`

- `const NAN: Self`

- `const NEG_NAN: Self`

- `const MAX: Self`

- `const MIN: Self`

- `const EPSILON: Self`

- `const PI: Self`

- `const NEG_PI: Self`

- `const FRAC_PI_2: Self`

- `const MIN_POSITIVE_NORMAL: Self`

- `const BITS: u32`

- `const SIG_BITS: u32`

- `const EXP_BITS: u32`

- `const EXP_SAT: u32`

- `const EXP_BIAS: u32`

- `const EXP_MAX: i32`

- `const EXP_MIN: i32`

- `const EXP_MIN_SUBNORM: i32`

- `const SIGN_MASK: <Self as >::Int`

- `const SIG_MASK: <Self as >::Int`

- `const EXP_MASK: <Self as >::Int`

- `const IMPLICIT_BIT: <Self as >::Int`

#### Required Methods

- `fn to_bits(self) -> <Self as >::Int`

  Returns `self` transmuted to `Self::Int`

- `fn is_nan(self) -> bool`

  Returns true if the value is NaN.

- `fn is_infinite(self) -> bool`

  Returns true if the value is +inf or -inf.

- `fn is_sign_negative(self) -> bool`

  Returns true if the sign is negative. Extracts the sign bit regardless of zero or NaN.

- `fn from_bits(a: <Self as >::Int) -> Self`

  Returns a `Self::Int` transmuted back to `Self`

- `fn abs(self) -> Self`

- `fn copysign(self, other: Self) -> Self`

  Returns a number composed of the magnitude of self and the sign of sign.

- `fn fma(self, y: Self, z: Self) -> Self`

  Fused multiply add, rounding once.

- `fn normalize(significand: <Self as >::Int) -> (i32, <Self as >::Int)`

  Returns (normalized exponent, normalized significand)

#### Provided Methods

- `fn to_bits_signed(self) -> <Self as >::SignedInt`

  Returns `self` transmuted to `Self::SignedInt`

- `fn biteq(self, rhs: Self) -> bool`

  Check bitwise equality.

- `fn eq_repr(self, rhs: Self) -> bool`

  Checks if two floats have the same bit representation. *Except* for NaNs! NaN can be

- `fn is_sign_positive(self) -> bool`

  Returns true if the sign is positive. Extracts the sign bit regardless of zero or NaN.

- `fn is_subnormal(self) -> bool`

  Returns if `self` is subnormal.

- `fn ex(self) -> u32`

  Returns the exponent, not adjusting for bias, not accounting for subnormals or zero.

- `fn exp_unbiased(self) -> i32`

  Extract the exponent and adjust it for bias, not accounting for subnormals or zero.

- `fn frac(self) -> <Self as >::Int`

  Returns the significand with no implicit bit (or the "fractional" part)

- `fn from_parts(negative: bool, exponent: u32, significand: <Self as >::Int) -> Self`

  Constructs a `Self` from its parts. Inputs are treated as bits and shifted into position.

- `fn signum(self) -> Self`

  Returns a number that represents the sign of self.

- `fn canonicalize(self) -> Self`

  Make a best-effort attempt to canonicalize the number. Note that this is allowed

#### Implementors

- `f32`
- `f64`

### `DFloat`

```rust
trait DFloat: Float { ... }
```

Trait for floats twice the bit width of another integer.

#### Associated Types

- `type H: 1`

#### Required Methods

- `fn narrow(self) -> <Self as >::H`

  Narrow the float type.

#### Implementors

- `f64`

### `HFloat`

```rust
trait HFloat: Float { ... }
```

Trait for floats half the bit width of another float.

#### Associated Types

- `type D: 1`

#### Required Methods

- `fn widen(self) -> <Self as >::D`

  Widen the float type.

#### Implementors

- `f32`

## Functions

### `f32_from_bits`

```rust
const fn f32_from_bits(bits: u32) -> f32
```

`f32::from_bits`

### `f32_to_bits`

```rust
const fn f32_to_bits(x: f32) -> u32
```

`f32::to_bits`

### `f64_from_bits`

```rust
const fn f64_from_bits(bits: u64) -> f64
```

`f64::from_bits`

### `f64_to_bits`

```rust
const fn f64_to_bits(x: f64) -> u64
```

`f64::to_bits`

## Type Aliases

### `IntTy<F>`

```rust
type IntTy<F> = <F as Float>::Int;
```

Access the associated `Int` type from a float (helper to avoid ambiguous associated types).

## Macros

### `float_impl!`

### `impl_d_float!`

### `impl_h_float!`

