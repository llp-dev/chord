*[num_traits](../../index.md) / [ops](../index.md) / [saturating](index.md)*

---

# Module `saturating`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Saturating`](#saturating) | trait | Saturating math operations. |
| [`SaturatingAdd`](#saturatingadd) | trait | Performs addition that saturates at the numeric bounds instead of overflowing. |
| [`SaturatingSub`](#saturatingsub) | trait | Performs subtraction that saturates at the numeric bounds instead of overflowing. |
| [`SaturatingMul`](#saturatingmul) | trait | Performs multiplication that saturates at the numeric bounds instead of overflowing. |
| [`deprecated_saturating_impl!`](#deprecated-saturating-impl) | macro |  |
| [`saturating_impl!`](#saturating-impl) | macro |  |

## Traits

### `Saturating`

```rust
trait Saturating { ... }
```

Saturating math operations. Deprecated, use `SaturatingAdd`, `SaturatingSub` and
`SaturatingMul` instead.

#### Required Methods

- `fn saturating_add(self, v: Self) -> Self`

  Saturating addition operator.

- `fn saturating_sub(self, v: Self) -> Self`

  Saturating subtraction operator.

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

### `SaturatingAdd`

```rust
trait SaturatingAdd: Sized + Add<Self, Output = Self> { ... }
```

Performs addition that saturates at the numeric bounds instead of overflowing.

#### Required Methods

- `fn saturating_add(&self, v: &Self) -> Self`

  Saturating addition. Computes `self + other`, saturating at the relevant high or low boundary of

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

### `SaturatingSub`

```rust
trait SaturatingSub: Sized + Sub<Self, Output = Self> { ... }
```

Performs subtraction that saturates at the numeric bounds instead of overflowing.

#### Required Methods

- `fn saturating_sub(&self, v: &Self) -> Self`

  Saturating subtraction. Computes `self - other`, saturating at the relevant high or low boundary of

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

### `SaturatingMul`

```rust
trait SaturatingMul: Sized + Mul<Self, Output = Self> { ... }
```

Performs multiplication that saturates at the numeric bounds instead of overflowing.

#### Required Methods

- `fn saturating_mul(&self, v: &Self) -> Self`

  Saturating multiplication. Computes `self * other`, saturating at the relevant high or low boundary of

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

### `deprecated_saturating_impl!`

### `saturating_impl!`

