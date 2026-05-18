*[num_traits](../../index.md) / [ops](../index.md) / [checked](index.md)*

---

# Module `checked`

## Contents

- [Traits](#traits)
  - [`CheckedAdd`](#checkedadd)
  - [`CheckedSub`](#checkedsub)
  - [`CheckedMul`](#checkedmul)
  - [`CheckedDiv`](#checkeddiv)
  - [`CheckedRem`](#checkedrem)
  - [`CheckedNeg`](#checkedneg)
  - [`CheckedShl`](#checkedshl)
  - [`CheckedShr`](#checkedshr)
- [Macros](#macros)
  - [`checked_impl!`](#checked-impl)
  - [`checked_impl_unary!`](#checked-impl-unary)
  - [`checked_shift_impl!`](#checked-shift-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CheckedAdd`](#checkedadd) | trait | Performs addition that returns `None` instead of wrapping around on overflow. |
| [`CheckedSub`](#checkedsub) | trait | Performs subtraction that returns `None` instead of wrapping around on underflow. |
| [`CheckedMul`](#checkedmul) | trait | Performs multiplication that returns `None` instead of wrapping around on underflow or overflow. |
| [`CheckedDiv`](#checkeddiv) | trait | Performs division that returns `None` instead of panicking on division by zero and instead of wrapping around on underflow and overflow. |
| [`CheckedRem`](#checkedrem) | trait | Performs an integral remainder that returns `None` instead of panicking on division by zero and instead of wrapping around on underflow and overflow. |
| [`CheckedNeg`](#checkedneg) | trait | Performs negation that returns `None` if the result can't be represented. |
| [`CheckedShl`](#checkedshl) | trait | Performs a left shift that returns `None` on shifts larger than or equal to the type width. |
| [`CheckedShr`](#checkedshr) | trait | Performs a right shift that returns `None` on shifts larger than or equal to the type width. |
| [`checked_impl!`](#checked-impl) | macro |  |
| [`checked_impl_unary!`](#checked-impl-unary) | macro |  |
| [`checked_shift_impl!`](#checked-shift-impl) | macro |  |

## Traits

### `CheckedAdd`

```rust
trait CheckedAdd: Sized + Add<Self, Output = Self> { ... }
```

Performs addition that returns `None` instead of wrapping around on
overflow.

#### Required Methods

- `fn checked_add(&self, v: &Self) -> Option<Self>`

  Adds two numbers, checking for overflow. If overflow happens, `None` is

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

### `CheckedSub`

```rust
trait CheckedSub: Sized + Sub<Self, Output = Self> { ... }
```

Performs subtraction that returns `None` instead of wrapping around on underflow.

#### Required Methods

- `fn checked_sub(&self, v: &Self) -> Option<Self>`

  Subtracts two numbers, checking for underflow. If underflow happens,

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

### `CheckedMul`

```rust
trait CheckedMul: Sized + Mul<Self, Output = Self> { ... }
```

Performs multiplication that returns `None` instead of wrapping around on underflow or
overflow.

#### Required Methods

- `fn checked_mul(&self, v: &Self) -> Option<Self>`

  Multiplies two numbers, checking for underflow or overflow. If underflow

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

### `CheckedDiv`

```rust
trait CheckedDiv: Sized + Div<Self, Output = Self> { ... }
```

Performs division that returns `None` instead of panicking on division by zero and instead of
wrapping around on underflow and overflow.

#### Required Methods

- `fn checked_div(&self, v: &Self) -> Option<Self>`

  Divides two numbers, checking for underflow, overflow and division by

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

### `CheckedRem`

```rust
trait CheckedRem: Sized + Rem<Self, Output = Self> { ... }
```

Performs an integral remainder that returns `None` instead of panicking on division by zero and
instead of wrapping around on underflow and overflow.

#### Required Methods

- `fn checked_rem(&self, v: &Self) -> Option<Self>`

  Finds the remainder of dividing two numbers, checking for underflow, overflow and division

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

### `CheckedNeg`

```rust
trait CheckedNeg: Sized { ... }
```

Performs negation that returns `None` if the result can't be represented.

#### Required Methods

- `fn checked_neg(&self) -> Option<Self>`

  Negates a number, returning `None` for results that can't be represented, like signed `MIN`

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

### `CheckedShl`

```rust
trait CheckedShl: Sized + Shl<u32, Output = Self> { ... }
```

Performs a left shift that returns `None` on shifts larger than
or equal to the type width.

#### Required Methods

- `fn checked_shl(&self, rhs: u32) -> Option<Self>`

  Checked shift left. Computes `self << rhs`, returning `None`

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

### `CheckedShr`

```rust
trait CheckedShr: Sized + Shr<u32, Output = Self> { ... }
```

Performs a right shift that returns `None` on shifts larger than
or equal to the type width.

#### Required Methods

- `fn checked_shr(&self, rhs: u32) -> Option<Self>`

  Checked shift right. Computes `self >> rhs`, returning `None`

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

### `checked_impl!`

### `checked_impl_unary!`

### `checked_shift_impl!`

