*[libm](../../../index.md) / [math](../../index.md) / [support](../index.md) / [int_traits](index.md)*

---

# Module `int_traits`

## Contents

- [Modules](#modules)
  - [`narrowing_div`](#narrowing-div)
- [Traits](#traits)
  - [`NarrowingDiv`](#narrowingdiv)
  - [`MinInt`](#minint)
  - [`Int`](#int)
  - [`DInt`](#dint)
  - [`HInt`](#hint)
  - [`CastInto`](#castinto)
  - [`CastFrom`](#castfrom)
- [Type Aliases](#type-aliases)
  - [`OtherSign`](#othersign)
- [Macros](#macros)
  - [`int_impl_common!`](#int-impl-common)
  - [`int_impl!`](#int-impl)
  - [`impl_d_int!`](#impl-d-int)
  - [`impl_h_int!`](#impl-h-int)
  - [`cast_into!`](#cast-into)
  - [`cast_into_float!`](#cast-into-float)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`narrowing_div`](#narrowing-div) | mod |  |
| [`NarrowingDiv`](#narrowingdiv) | trait |  |
| [`MinInt`](#minint) | trait | Minimal integer implementations needed on all integer types, including wide integers. |
| [`Int`](#int) | trait | Trait for some basic operations on integers |
| [`DInt`](#dint) | trait | Trait for integers twice the bit width of another integer. |
| [`HInt`](#hint) | trait | Trait for integers half the bit width of another integer. |
| [`CastInto`](#castinto) | trait | Trait to express (possibly lossy) casting of integers |
| [`CastFrom`](#castfrom) | trait |  |
| [`OtherSign`](#othersign) | type | Access the associated `OtherSign` type from an int (helper to avoid ambiguous associated types). |
| [`int_impl_common!`](#int-impl-common) | macro |  |
| [`int_impl!`](#int-impl) | macro |  |
| [`impl_d_int!`](#impl-d-int) | macro |  |
| [`impl_h_int!`](#impl-h-int) | macro |  |
| [`cast_into!`](#cast-into) | macro |  |
| [`cast_into_float!`](#cast-into-float) | macro |  |

## Modules

- [`narrowing_div`](narrowing_div/index.md)

## Traits

### `NarrowingDiv`

```rust
trait NarrowingDiv: DInt + MinInt<Unsigned = Self> { ... }
```

Trait for unsigned division of a double-wide integer
when the quotient doesn't overflow.

This is the inverse of widening multiplication:
 - for any `x` and nonzero `y`: `x.widen_mul(y).checked_narrowing_div_rem(y) == Some((x, 0))`,
 - and for any `r in 0..y`: `x.carrying_mul(y, r).checked_narrowing_div_rem(y) == Some((x, r))`,

#### Required Methods

- `fn unchecked_narrowing_div_rem(self, n: <Self as >::H) -> (<Self as >::H, <Self as >::H)`

  Computes `(self / n, self % n))`

#### Provided Methods

- `fn checked_narrowing_div_rem(self, n: <Self as >::H) -> Option<(<Self as >::H, <Self as >::H)>`

  Returns `Some((self / n, self % n))` when `self.hi() < n`.

#### Implementors

- [`u256`](../big/index.md#u256)
- `u128`
- `u16`
- `u32`
- `u64`

### `MinInt`

```rust
trait MinInt: Copy + fmt::Debug + ops::BitOr<Output = Self> + ops::Not<Output = Self> + ops::Shl<u32, Output = Self> { ... }
```

Minimal integer implementations needed on all integer types, including wide integers.

#### Associated Types

- `type OtherSign: 1`

- `type Unsigned: 1`

#### Associated Constants

- `const SIGNED: bool`

- `const BITS: u32`

- `const ZERO: Self`

- `const ONE: Self`

- `const MIN: Self`

- `const MAX: Self`

#### Implementors

- [`i256`](../big/index.md#i256)
- [`u256`](../big/index.md#u256)
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

### `Int`

```rust
trait Int: MinInt + fmt::Display + fmt::Binary + fmt::LowerHex + ops::AddAssign + ops::SubAssign + ops::MulAssign + ops::DivAssign + ops::RemAssign + ops::BitAndAssign + ops::BitOrAssign + ops::BitXorAssign + ops::ShlAssign<i32> + ops::ShlAssign<u32> + ops::ShrAssign<u32> + ops::ShrAssign<i32> + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> + ops::Rem<Output = Self> + ops::Shl<i32, Output = Self> + ops::Shl<u32, Output = Self> + ops::Shr<i32, Output = Self> + ops::Shr<u32, Output = Self> + ops::BitXor<Output = Self> + ops::BitAnd<Output = Self> + cmp::Ord + From<bool> + CastFrom<i32> + CastFrom<u16> + CastFrom<u32> + CastFrom<u8> + CastFrom<usize> + CastInto<i32> + CastInto<u16> + CastInto<u32> + CastInto<u8> + CastInto<usize> { ... }
```

Trait for some basic operations on integers

#### Required Methods

- `fn signed(self) -> <<Self as >::Unsigned as MinInt>::OtherSign`

- `fn unsigned(self) -> <Self as >::Unsigned`

- `fn from_unsigned(unsigned: <Self as >::Unsigned) -> Self`

- `fn abs(self) -> Self`

- `fn unsigned_abs(self) -> <Self as >::Unsigned`

- `fn from_bool(b: bool) -> Self`

- `fn logical_shr(self, other: u32) -> Self`

  Prevents the need for excessive conversions between signed and unsigned

- `fn abs_diff(self, other: Self) -> <Self as >::Unsigned`

  Absolute difference between two integers.

- `fn is_zero(self) -> bool`

- `fn checked_add(self, other: Self) -> Option<Self>`

- `fn checked_sub(self, other: Self) -> Option<Self>`

- `fn wrapping_neg(self) -> Self`

- `fn wrapping_add(self, other: Self) -> Self`

- `fn wrapping_mul(self, other: Self) -> Self`

- `fn wrapping_sub(self, other: Self) -> Self`

- `fn wrapping_shl(self, other: u32) -> Self`

- `fn wrapping_shr(self, other: u32) -> Self`

- `fn rotate_left(self, other: u32) -> Self`

- `fn overflowing_add(self, other: Self) -> (Self, bool)`

- `fn overflowing_sub(self, other: Self) -> (Self, bool)`

- `fn carrying_add(self, other: Self, carry: bool) -> (Self, bool)`

- `fn borrowing_sub(self, other: Self, borrow: bool) -> (Self, bool)`

- `fn leading_zeros(self) -> u32`

- `fn trailing_zeros(self) -> u32`

- `fn ilog2(self) -> u32`

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

### `DInt`

```rust
trait DInt: MinInt + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Shl<u32, Output = Self> + ops::Shr<u32, Output = Self> + Ord { ... }
```

Trait for integers twice the bit width of another integer. This is implemented for all
primitives except for `u8`, because there is not a smaller primitive.

#### Associated Types

- `type H: 1`

#### Required Methods

- `fn lo(self) -> <Self as >::H`

  Returns the low half of `self`

- `fn hi(self) -> <Self as >::H`

  Returns the high half of `self`

#### Provided Methods

- `fn lo_hi(self) -> (<Self as >::H, <Self as >::H)`

  Returns the low and high halves of `self` as a tuple

- `fn from_lo_hi(lo: <Self as >::H, hi: <Self as >::H) -> Self`

  Constructs an integer using lower and higher half parts

#### Implementors

- [`i256`](../big/index.md#i256)
- [`u256`](../big/index.md#u256)
- `i128`
- `i16`
- `i32`
- `i64`
- `u128`
- `u16`
- `u32`
- `u64`

### `HInt`

```rust
trait HInt: Int { ... }
```

Trait for integers half the bit width of another integer. This is implemented for all
primitives except for `u128`, because it there is not a larger primitive.

#### Associated Types

- `type D: 2`

#### Required Methods

- `fn widen(self) -> <Self as >::D`

  Widens (using default extension) the integer to have double bit width

- `fn zero_widen(self) -> <Self as >::D`

  Widens (zero extension only) the integer to have double bit width. This is needed to get

- `fn widen_hi(self) -> <Self as >::D`

  Widens the integer to have double bit width and shifts the integer into the higher bits

- `fn zero_widen_mul(self, rhs: Self) -> <Self as >::D`

  Widening multiplication with zero widening. This cannot overflow.

- `fn widen_mul(self, rhs: Self) -> <Self as >::D`

  Widening multiplication. This cannot overflow.

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`

### `CastInto<T: Copy>`

```rust
trait CastInto<T: Copy>: Copy { ... }
```

Trait to express (possibly lossy) casting of integers

#### Required Methods

- `fn cast(self) -> T`

  By default, casts should be exact.

- `fn cast_lossy(self) -> T`

  Call for casts that are expected to truncate.

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

### `CastFrom<T: Copy>`

```rust
trait CastFrom<T: Copy>: Copy { ... }
```

#### Required Methods

- `fn cast_from(value: T) -> Self`

  By default, casts should be exact.

- `fn cast_from_lossy(value: T) -> Self`

  Call for casts that are expected to truncate.

#### Implementors

- `T`

## Type Aliases

### `OtherSign<I>`

```rust
type OtherSign<I> = <I as MinInt>::OtherSign;
```

Access the associated `OtherSign` type from an int (helper to avoid ambiguous associated
types).

## Macros

### `int_impl_common!`

### `int_impl!`

### `impl_d_int!`

### `impl_h_int!`

### `cast_into!`

### `cast_into_float!`

