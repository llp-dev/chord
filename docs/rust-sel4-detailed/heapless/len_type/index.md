*[heapless](../index.md) / [len_type](index.md)*

---

# Module `len_type`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sealed`](#sealed) | trait |  |
| [`LenType`](#lentype) | trait | A sealed trait representing a valid type to use as a length for a container. |
| [`check_capacity_fits`](#check-capacity-fits) | fn |  |
| [`impl_lentype!`](#impl-lentype) | macro |  |

## Traits

### `Sealed`

```rust
trait Sealed: Send + Sync + Copy + Display + Debug + PartialEq + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + PartialOrd + TryFrom<usize, Error: Debug> + TryInto<usize, Error: Debug> { ... }
```

#### Associated Constants

- `const ZERO: Self`

- `const MAX: Self`

- `const MAX_USIZE: usize`

#### Required Methods

- `fn one() -> Self`

  The one value of the integer type.

#### Provided Methods

- `fn from_usize(val: usize) -> Self`

  An infallible conversion from `usize` to `LenT`.

- `fn into_usize(self) -> usize`

  An infallible conversion from `LenT` to `usize`.

- `fn to_non_max(self) -> Option<usize>`

  Converts `LenT` into `Some(usize)`, unless it's `Self::MAX`, where it returns `None`.

#### Implementors

- `u16`
- `u32`
- `u8`
- `usize`

### `LenType`

```rust
trait LenType: Sealed { ... }
```

A sealed trait representing a valid type to use as a length for a container.

This cannot be implemented in user code, and is restricted to `u8`, `u16`, `u32`, and `usize`.

#### Implementors

- `u16`
- `u32`
- `u8`
- `usize`

## Functions

### `check_capacity_fits`

```rust
const fn check_capacity_fits<LenT: LenType, const N: usize>()
```

## Macros

### `impl_lentype!`

