*[num_traits](../index.md) / [pow](index.md)*

---

# Module `pow`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`float_impls`](#float-impls) | mod |  |
| [`Pow`](#pow) | trait | Binary operator for raising a value to a power. |
| [`pow`](#pow) | fn | Raises a value to the power of exp, using exponentiation by squaring. |
| [`checked_pow`](#checked-pow) | fn | Raises a value to the power of exp, returning `None` if an overflow occurred. |
| [`pow_impl!`](#pow-impl) | macro |  |

## Modules

- [`float_impls`](float_impls/index.md)

## Traits

### `Pow<RHS>`

```rust
trait Pow<RHS> { ... }
```

Binary operator for raising a value to a power.

#### Associated Types

- `type Output`

#### Required Methods

- `fn pow(self, rhs: RHS) -> <Self as >::Output`

  Returns `self` to the power `rhs`.

#### Implementors

- `&'a core::num::Wrapping<i128>`
- `&'a core::num::Wrapping<i16>`
- `&'a core::num::Wrapping<i32>`
- `&'a core::num::Wrapping<i64>`
- `&'a core::num::Wrapping<i8>`
- `&'a core::num::Wrapping<isize>`
- `&'a core::num::Wrapping<u128>`
- `&'a core::num::Wrapping<u16>`
- `&'a core::num::Wrapping<u32>`
- `&'a core::num::Wrapping<u64>`
- `&'a core::num::Wrapping<u8>`
- `&'a core::num::Wrapping<usize>`
- `&'a f32`
- `&'a f64`
- `&'a i128`
- `&'a i16`
- `&'a i32`
- `&'a i64`
- `&'a i8`
- `&'a isize`
- `&'a u128`
- `&'a u16`
- `&'a u32`
- `&'a u64`
- `&'a u8`
- `&'a usize`
- `&'b core::num::Wrapping<i128>`
- `&'b core::num::Wrapping<i16>`
- `&'b core::num::Wrapping<i32>`
- `&'b core::num::Wrapping<i64>`
- `&'b core::num::Wrapping<i8>`
- `&'b core::num::Wrapping<isize>`
- `&'b core::num::Wrapping<u128>`
- `&'b core::num::Wrapping<u16>`
- `&'b core::num::Wrapping<u32>`
- `&'b core::num::Wrapping<u64>`
- `&'b core::num::Wrapping<u8>`
- `&'b core::num::Wrapping<usize>`
- `&'b f32`
- `&'b f64`
- `&'b i128`
- `&'b i16`
- `&'b i32`
- `&'b i64`
- `&'b i8`
- `&'b isize`
- `&'b u128`
- `&'b u16`
- `&'b u32`
- `&'b u64`
- `&'b u8`
- `&'b usize`
- `core::num::Wrapping<i128>`
- `core::num::Wrapping<i16>`
- `core::num::Wrapping<i32>`
- `core::num::Wrapping<i64>`
- `core::num::Wrapping<i8>`
- `core::num::Wrapping<isize>`
- `core::num::Wrapping<u128>`
- `core::num::Wrapping<u16>`
- `core::num::Wrapping<u32>`
- `core::num::Wrapping<u64>`
- `core::num::Wrapping<u8>`
- `core::num::Wrapping<usize>`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Functions

### `pow`

```rust
fn pow<T: Clone + One + Mul<T, Output = T>>(base: T, exp: usize) -> T
```

Raises a value to the power of exp, using exponentiation by squaring.

Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.

# Example

```rust
use num_traits::pow;

assert_eq!(pow(2i8, 4), 16);
assert_eq!(pow(6u8, 3), 216);
assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you
```

### `checked_pow`

```rust
fn checked_pow<T: Clone + One + CheckedMul>(base: T, exp: usize) -> Option<T>
```

Raises a value to the power of exp, returning `None` if an overflow occurred.

Note that `0⁰` (`checked_pow(0, 0)`) returns `Some(1)`. Mathematically this is undefined.

Otherwise same as the `pow` function.

# Example

```rust
use num_traits::checked_pow;

assert_eq!(checked_pow(2i8, 4), Some(16));
assert_eq!(checked_pow(7i8, 8), None);
assert_eq!(checked_pow(7u32, 8), Some(5_764_801));
assert_eq!(checked_pow(0u32, 0), Some(1)); // Be aware if this case affect you
```

## Macros

### `pow_impl!`

