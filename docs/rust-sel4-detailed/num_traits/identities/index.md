*[num_traits](../index.md) / [identities](index.md)*

---

# Module `identities`

## Contents

- [Traits](#traits)
  - [`Zero`](#zero)
  - [`ConstZero`](#constzero)
  - [`One`](#one)
  - [`ConstOne`](#constone)
- [Functions](#functions)
  - [`zero`](#zero)
  - [`one`](#one)
- [Macros](#macros)
  - [`zero_impl!`](#zero-impl)
  - [`one_impl!`](#one-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Zero`](#zero) | trait | Defines an additive identity element for `Self`. |
| [`ConstZero`](#constzero) | trait | Defines an associated constant representing the additive identity element for `Self`. |
| [`One`](#one) | trait | Defines a multiplicative identity element for `Self`. |
| [`ConstOne`](#constone) | trait | Defines an associated constant representing the multiplicative identity element for `Self`. |
| [`zero`](#zero) | fn | Returns the additive identity, `0`. |
| [`one`](#one) | fn | Returns the multiplicative identity, `1`. |
| [`zero_impl!`](#zero-impl) | macro |  |
| [`one_impl!`](#one-impl) | macro |  |

## Traits

### `Zero`

```rust
trait Zero: Sized + Add<Self, Output = Self> { ... }
```

Defines an additive identity element for `Self`.

# Laws

```text
a + 0 = a       ∀ a ∈ Self
0 + a = a       ∀ a ∈ Self
```

#### Required Methods

- `fn zero() -> Self`

  Returns the additive identity element of `Self`, `0`.

- `fn is_zero(&self) -> bool`

  Returns `true` if `self` is equal to the additive identity.

#### Provided Methods

- `fn set_zero(&mut self)`

  Sets `self` to the additive identity element of `Self`, `0`.

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
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `ConstZero`

```rust
trait ConstZero: Zero { ... }
```

Defines an associated constant representing the additive identity element
for `Self`.

#### Associated Constants

- `const ZERO: Self`

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
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `One`

```rust
trait One: Sized + Mul<Self, Output = Self> { ... }
```

Defines a multiplicative identity element for `Self`.

# Laws

```text
a * 1 = a       ∀ a ∈ Self
1 * a = a       ∀ a ∈ Self
```

#### Required Methods

- `fn one() -> Self`

  Returns the multiplicative identity element of `Self`, `1`.

#### Provided Methods

- `fn set_one(&mut self)`

  Sets `self` to the multiplicative identity element of `Self`, `1`.

- `fn is_one(&self) -> bool`

  Returns `true` if `self` is equal to the multiplicative identity.

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
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `ConstOne`

```rust
trait ConstOne: One { ... }
```

Defines an associated constant representing the multiplicative identity
element for `Self`.

#### Associated Constants

- `const ONE: Self`

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
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Functions

### `zero`

```rust
fn zero<T: Zero>() -> T
```

Returns the additive identity, `0`.

### `one`

```rust
fn one<T: One>() -> T
```

Returns the multiplicative identity, `1`.

## Macros

### `zero_impl!`

### `one_impl!`

