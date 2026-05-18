*[num_traits](../index.md) / [sign](index.md)*

---

# Module `sign`

## Contents

- [Traits](#traits)
  - [`Signed`](#signed)
  - [`Unsigned`](#unsigned)
- [Functions](#functions)
  - [`abs`](#abs)
  - [`abs_sub`](#abs-sub)
  - [`signum`](#signum)
- [Macros](#macros)
  - [`signed_impl!`](#signed-impl)
  - [`signed_float_impl!`](#signed-float-impl)
  - [`empty_trait_impl!`](#empty-trait-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Signed`](#signed) | trait | Useful functions for signed numbers (i.e. numbers that can be negative). |
| [`Unsigned`](#unsigned) | trait | A trait for values which cannot be negative |
| [`abs`](#abs) | fn | Computes the absolute value. |
| [`abs_sub`](#abs-sub) | fn | The positive difference of two numbers. |
| [`signum`](#signum) | fn | Returns the sign of the number. |
| [`signed_impl!`](#signed-impl) | macro |  |
| [`signed_float_impl!`](#signed-float-impl) | macro |  |
| [`empty_trait_impl!`](#empty-trait-impl) | macro |  |

## Traits

### `Signed`

```rust
trait Signed: Sized + Num + Neg<Output = Self> { ... }
```

Useful functions for signed numbers (i.e. numbers that can be negative).

#### Required Methods

- `fn abs(&self) -> Self`

  Computes the absolute value.

- `fn abs_sub(&self, other: &Self) -> Self`

  The positive difference of two numbers.

- `fn signum(&self) -> Self`

  Returns the sign of the number.

- `fn is_positive(&self) -> bool`

  Returns true if the number is positive and false if the number is zero or negative.

- `fn is_negative(&self) -> bool`

  Returns true if the number is negative and false if the number is zero or positive.

#### Implementors

- `core::num::Wrapping<T>`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`

### `Unsigned`

```rust
trait Unsigned: Num { ... }
```

A trait for values which cannot be negative

#### Implementors

- `core::num::Wrapping<T>`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Functions

### `abs`

```rust
fn abs<T: Signed>(value: T) -> T
```

Computes the absolute value.

For `f32` and `f64`, `NaN` will be returned if the number is `NaN`

For signed integers, `::MIN` will be returned if the number is `::MIN`.

### `abs_sub`

```rust
fn abs_sub<T: Signed>(x: T, y: T) -> T
```

The positive difference of two numbers.

Returns zero if `x` is less than or equal to `y`, otherwise the difference
between `x` and `y` is returned.

### `signum`

```rust
fn signum<T: Signed>(value: T) -> T
```

Returns the sign of the number.

For `f32` and `f64`:

* `1.0` if the number is positive, `+0.0` or `INFINITY`
* `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
* `NaN` if the number is `NaN`

For signed integers:

* `0` if the number is zero
* `1` if the number is positive
* `-1` if the number is negative

## Macros

### `signed_impl!`

### `signed_float_impl!`

### `empty_trait_impl!`

