*[num_traits](../../index.md) / [ops](../index.md) / [euclid](index.md)*

---

# Module `euclid`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Euclid`](#euclid) | trait |  |
| [`CheckedEuclid`](#checkedeuclid) | trait |  |
| [`euclid_forward_impl!`](#euclid-forward-impl) | macro |  |
| [`checked_euclid_forward_impl!`](#checked-euclid-forward-impl) | macro |  |

## Traits

### `Euclid`

```rust
trait Euclid: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> { ... }
```

#### Required Methods

- `fn div_euclid(&self, v: &Self) -> Self`

  Calculates Euclidean division, the matching method for `rem_euclid`.

- `fn rem_euclid(&self, v: &Self) -> Self`

  Calculates the least nonnegative remainder of `self (mod v)`.

#### Provided Methods

- `fn div_rem_euclid(&self, v: &Self) -> (Self, Self)`

  Returns both the quotient and remainder from Euclidean division.

#### Implementors

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

### `CheckedEuclid`

```rust
trait CheckedEuclid: Euclid { ... }
```

#### Required Methods

- `fn checked_div_euclid(&self, v: &Self) -> Option<Self>`

  Performs euclid division that returns `None` instead of panicking on division by zero

- `fn checked_rem_euclid(&self, v: &Self) -> Option<Self>`

  Finds the euclid remainder of dividing two numbers, checking for underflow, overflow and

#### Provided Methods

- `fn checked_div_rem_euclid(&self, v: &Self) -> Option<(Self, Self)>`

  Returns both the quotient and remainder from checked Euclidean division.

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

### `euclid_forward_impl!`

### `checked_euclid_forward_impl!`

