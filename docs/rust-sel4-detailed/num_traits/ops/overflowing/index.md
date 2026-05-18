*[num_traits](../../index.md) / [ops](../index.md) / [overflowing](index.md)*

---

# Module `overflowing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OverflowingAdd`](#overflowingadd) | trait | Performs addition with a flag for overflow. |
| [`OverflowingSub`](#overflowingsub) | trait | Performs substraction with a flag for overflow. |
| [`OverflowingMul`](#overflowingmul) | trait | Performs multiplication with a flag for overflow. |
| [`overflowing_impl!`](#overflowing-impl) | macro |  |

## Traits

### `OverflowingAdd`

```rust
trait OverflowingAdd: Sized + Add<Self, Output = Self> { ... }
```

Performs addition with a flag for overflow.

#### Required Methods

- `fn overflowing_add(&self, v: &Self) -> (Self, bool)`

  Returns a tuple of the sum along with a boolean indicating whether an arithmetic overflow would occur.

#### Implementors

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

### `OverflowingSub`

```rust
trait OverflowingSub: Sized + Sub<Self, Output = Self> { ... }
```

Performs substraction with a flag for overflow.

#### Required Methods

- `fn overflowing_sub(&self, v: &Self) -> (Self, bool)`

  Returns a tuple of the difference along with a boolean indicating whether an arithmetic overflow would occur.

#### Implementors

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

### `OverflowingMul`

```rust
trait OverflowingMul: Sized + Mul<Self, Output = Self> { ... }
```

Performs multiplication with a flag for overflow.

#### Required Methods

- `fn overflowing_mul(&self, v: &Self) -> (Self, bool)`

  Returns a tuple of the product along with a boolean indicating whether an arithmetic overflow would occur.

#### Implementors

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

### `overflowing_impl!`

