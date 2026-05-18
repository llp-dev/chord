*[num_traits](../../index.md) / [ops](../index.md) / [wrapping](index.md)*

---

# Module `wrapping`

## Contents

- [Traits](#traits)
  - [`WrappingAdd`](#wrappingadd)
  - [`WrappingSub`](#wrappingsub)
  - [`WrappingMul`](#wrappingmul)
  - [`WrappingNeg`](#wrappingneg)
  - [`WrappingShl`](#wrappingshl)
  - [`WrappingShr`](#wrappingshr)
- [Macros](#macros)
  - [`wrapping_impl!`](#wrapping-impl)
  - [`wrapping_unary_impl!`](#wrapping-unary-impl)
  - [`wrapping_shift_impl!`](#wrapping-shift-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WrappingAdd`](#wrappingadd) | trait | Performs addition that wraps around on overflow. |
| [`WrappingSub`](#wrappingsub) | trait | Performs subtraction that wraps around on overflow. |
| [`WrappingMul`](#wrappingmul) | trait | Performs multiplication that wraps around on overflow. |
| [`WrappingNeg`](#wrappingneg) | trait | Performs a negation that does not panic. |
| [`WrappingShl`](#wrappingshl) | trait | Performs a left shift that does not panic. |
| [`WrappingShr`](#wrappingshr) | trait | Performs a right shift that does not panic. |
| [`wrapping_impl!`](#wrapping-impl) | macro |  |
| [`wrapping_unary_impl!`](#wrapping-unary-impl) | macro |  |
| [`wrapping_shift_impl!`](#wrapping-shift-impl) | macro |  |

## Traits

### `WrappingAdd`

```rust
trait WrappingAdd: Sized + Add<Self, Output = Self> { ... }
```

Performs addition that wraps around on overflow.

#### Required Methods

- `fn wrapping_add(&self, v: &Self) -> Self`

  Wrapping (modular) addition. Computes `self + other`, wrapping around at the boundary of

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingSub`

```rust
trait WrappingSub: Sized + Sub<Self, Output = Self> { ... }
```

Performs subtraction that wraps around on overflow.

#### Required Methods

- `fn wrapping_sub(&self, v: &Self) -> Self`

  Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the boundary

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingMul`

```rust
trait WrappingMul: Sized + Mul<Self, Output = Self> { ... }
```

Performs multiplication that wraps around on overflow.

#### Required Methods

- `fn wrapping_mul(&self, v: &Self) -> Self`

  Wrapping (modular) multiplication. Computes `self * other`, wrapping around at the boundary

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingNeg`

```rust
trait WrappingNeg: Sized { ... }
```

Performs a negation that does not panic.

#### Required Methods

- `fn wrapping_neg(&self) -> Self`

  Wrapping (modular) negation. Computes `-self`,

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingShl`

```rust
trait WrappingShl: Sized + Shl<usize, Output = Self> { ... }
```

Performs a left shift that does not panic.

#### Required Methods

- `fn wrapping_shl(&self, rhs: u32) -> Self`

  Panic-free bitwise shift-left; yields `self << mask(rhs)`,

#### Implementors

- `core::num::Wrapping<T>`
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

### `WrappingShr`

```rust
trait WrappingShr: Sized + Shr<usize, Output = Self> { ... }
```

Performs a right shift that does not panic.

#### Required Methods

- `fn wrapping_shr(&self, rhs: u32) -> Self`

  Panic-free bitwise shift-right; yields `self >> mask(rhs)`,

#### Implementors

- `core::num::Wrapping<T>`
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

## Macros

### `wrapping_impl!`

### `wrapping_unary_impl!`

### `wrapping_shift_impl!`

