# Crate `num_integer`

Integer trait and functions.

## Compatibility

The `num-integer` crate is tested for rustc 1.31 and greater.

## Contents

- [Modules](#modules)
  - [`roots`](#roots)
  - [`average`](#average)
- [Structs](#structs)
  - [`ExtendedGcd`](#extendedgcd)
  - [`IterBinomial`](#iterbinomial)
- [Traits](#traits)
  - [`Roots`](#roots)
  - [`Average`](#average)
  - [`Integer`](#integer)
- [Functions](#functions)
  - [`cbrt`](#cbrt)
  - [`nth_root`](#nth-root)
  - [`sqrt`](#sqrt)
  - [`average_ceil`](#average-ceil)
  - [`average_floor`](#average-floor)
  - [`div_rem`](#div-rem)
  - [`div_floor`](#div-floor)
  - [`mod_floor`](#mod-floor)
  - [`div_mod_floor`](#div-mod-floor)
  - [`div_ceil`](#div-ceil)
  - [`gcd`](#gcd)
  - [`lcm`](#lcm)
  - [`gcd_lcm`](#gcd-lcm)
  - [`multiply_and_divide`](#multiply-and-divide)
  - [`binomial`](#binomial)
  - [`multinomial`](#multinomial)
- [Macros](#macros)
  - [`impl_integer_for_isize!`](#impl-integer-for-isize)
  - [`impl_integer_for_usize!`](#impl-integer-for-usize)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`roots`](#roots) | mod |  |
| [`average`](#average) | mod |  |
| [`ExtendedGcd`](#extendedgcd) | struct | Greatest common divisor and Bézout coefficients |
| [`IterBinomial`](#iterbinomial) | struct | An iterator over binomial coefficients. |
| [`Roots`](#roots) | trait |  |
| [`Average`](#average) | trait |  |
| [`Integer`](#integer) | trait |  |
| [`cbrt`](#cbrt) | fn |  |
| [`nth_root`](#nth-root) | fn |  |
| [`sqrt`](#sqrt) | fn |  |
| [`average_ceil`](#average-ceil) | fn |  |
| [`average_floor`](#average-floor) | fn |  |
| [`div_rem`](#div-rem) | fn | Simultaneous integer division and modulus |
| [`div_floor`](#div-floor) | fn | Floored integer division |
| [`mod_floor`](#mod-floor) | fn | Floored integer modulus |
| [`div_mod_floor`](#div-mod-floor) | fn | Simultaneous floored integer division and modulus |
| [`div_ceil`](#div-ceil) | fn | Ceiled integer division |
| [`gcd`](#gcd) | fn | Calculates the Greatest Common Divisor (GCD) of the number and `other`. |
| [`lcm`](#lcm) | fn | Calculates the Lowest Common Multiple (LCM) of the number and `other`. |
| [`gcd_lcm`](#gcd-lcm) | fn | Calculates the Greatest Common Divisor (GCD) and Lowest Common Multiple (LCM) of the number and `other`. |
| [`multiply_and_divide`](#multiply-and-divide) | fn | Calculate r * a / b, avoiding overflows and fractions. |
| [`binomial`](#binomial) | fn | Calculate the binomial coefficient. |
| [`multinomial`](#multinomial) | fn | Calculate the multinomial coefficient. |
| [`impl_integer_for_isize!`](#impl-integer-for-isize) | macro |  |
| [`impl_integer_for_usize!`](#impl-integer-for-usize) | macro |  |

## Modules

- [`roots`](roots/index.md)
- [`average`](average/index.md)

## Structs

### `ExtendedGcd<A>`

```rust
struct ExtendedGcd<A> {
    pub gcd: A,
    pub x: A,
    pub y: A,
}
```

Greatest common divisor and Bézout coefficients

```no_build
let e = isize::extended_gcd(a, b);
assert_eq!(e.gcd, e.x*a + e.y*b);
```

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for ExtendedGcd<A>`

- <span id="extendedgcd-clone"></span>`fn clone(&self) -> ExtendedGcd<A>` — [`ExtendedGcd`](#extendedgcd)

##### `impl<A: marker::Copy> Copy for ExtendedGcd<A>`

##### `impl<A: fmt::Debug> Debug for ExtendedGcd<A>`

- <span id="extendedgcd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: cmp::Eq> Eq for ExtendedGcd<A>`

##### `impl<A: cmp::PartialEq> PartialEq for ExtendedGcd<A>`

- <span id="extendedgcd-partialeq-eq"></span>`fn eq(&self, other: &ExtendedGcd<A>) -> bool` — [`ExtendedGcd`](#extendedgcd)

##### `impl<A> StructuralPartialEq for ExtendedGcd<A>`

### `IterBinomial<T>`

```rust
struct IterBinomial<T> {
    a: T,
    n: T,
    k: T,
}
```

An iterator over binomial coefficients.

#### Implementations

- <span id="iterbinomial-new"></span>`fn new(n: T) -> IterBinomial<T>` — [`IterBinomial`](#iterbinomial)

  For a given n, iterate over all binomial coefficients binomial(n, k), for k=0...n.

  

  Note that this might overflow, depending on `T`. For the primitive

  integer types, the following n are the largest ones for which there will

  be no overflow:

  

  type | n

  -----|---

  u8   | 10

  i8   |  9

  u16  | 18

  i16  | 17

  u32  | 34

  i32  | 33

  u64  | 67

  i64  | 66

  

  For larger n, `T` should be a bigint type.

#### Trait Implementations

##### `impl IntoIterator for IterBinomial<T>`

- <span id="iterbinomial-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterbinomial-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterbinomial-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterBinomial<T>`

- <span id="iterbinomial-iterator-type-item"></span>`type Item = T`

- <span id="iterbinomial-iterator-next"></span>`fn next(&mut self) -> Option<T>`

## Traits

### `Roots`

```rust
trait Roots: Integer { ... }
```

Provides methods to compute an integer's square root, cube root,
and arbitrary `n`th root.

#### Required Methods

- `fn nth_root(&self, n: u32) -> Self`

  Returns the truncated principal `n`th root of an integer

#### Provided Methods

- `fn sqrt(&self) -> Self`

  Returns the truncated principal square root of an integer -- `⌊√x⌋`

- `fn cbrt(&self) -> Self`

  Returns the truncated principal cube root of an integer --

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

### `Average`

```rust
trait Average: Integer { ... }
```

Provides methods to compute the average of two integers, without overflows.

#### Required Methods

- `fn average_ceil(&self, other: &Self) -> Self`

  Returns the ceiling value of the average of `self` and `other`.

- `fn average_floor(&self, other: &Self) -> Self`

  Returns the floor value of the average of `self` and `other`.

#### Implementors

- `I`

### `Integer`

```rust
trait Integer: Sized + Num + PartialOrd + Ord + Eq { ... }
```

#### Required Methods

- `fn div_floor(&self, other: &Self) -> Self`

  Floored integer division.

- `fn mod_floor(&self, other: &Self) -> Self`

  Floored integer modulo, satisfying:

- `fn gcd(&self, other: &Self) -> Self`

  Greatest Common Divisor (GCD).

- `fn lcm(&self, other: &Self) -> Self`

  Lowest Common Multiple (LCM).

- `fn is_multiple_of(&self, other: &Self) -> bool`

  Returns `true` if `self` is a multiple of `other`.

- `fn is_even(&self) -> bool`

  Returns `true` if the number is even.

- `fn is_odd(&self) -> bool`

  Returns `true` if the number is odd.

- `fn div_rem(&self, other: &Self) -> (Self, Self)`

  Simultaneous truncated integer division and modulus.

#### Provided Methods

- `fn div_ceil(&self, other: &Self) -> Self`

  Ceiled integer division.

- `fn gcd_lcm(&self, other: &Self) -> (Self, Self)`

  Greatest Common Divisor (GCD) and

- `fn extended_gcd(&self, other: &Self) -> ExtendedGcd<Self>`

  Greatest common divisor and Bézout coefficients.

- `fn extended_gcd_lcm(&self, other: &Self) -> (ExtendedGcd<Self>, Self)`

  Greatest common divisor, least common multiple, and Bézout coefficients.

- `fn divides(&self, other: &Self) -> bool`

  Deprecated, use `is_multiple_of` instead.

- `fn div_mod_floor(&self, other: &Self) -> (Self, Self)`

  Simultaneous floored integer division and modulus.

- `fn next_multiple_of(&self, other: &Self) -> Self`

  Rounds up to nearest multiple of argument.

- `fn prev_multiple_of(&self, other: &Self) -> Self`

  Rounds down to nearest multiple of argument.

- `fn dec(&mut self)`

  Decrements self by one.

- `fn inc(&mut self)`

  Increments self by one.

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

## Functions

### `cbrt`

```rust
fn cbrt<T: Roots>(x: T) -> T
```

Returns the truncated principal cube root of an integer --
see [Roots::cbrt](#roots-cbrt).

### `nth_root`

```rust
fn nth_root<T: Roots>(x: T, n: u32) -> T
```

Returns the truncated principal `n`th root of an integer --
see [Roots::nth_root](#roots-nth-root).

### `sqrt`

```rust
fn sqrt<T: Roots>(x: T) -> T
```

Returns the truncated principal square root of an integer --
see [Roots::sqrt](#roots-sqrt).

### `average_ceil`

```rust
fn average_ceil<T: Average>(x: T, y: T) -> T
```

Returns the ceiling value of the average of `x` and `y` --
see [Average::average_ceil](#average-average-ceil).

### `average_floor`

```rust
fn average_floor<T: Average>(x: T, y: T) -> T
```

Returns the floor value of the average of `x` and `y` --
see [Average::average_floor](#average-average-floor).

### `div_rem`

```rust
fn div_rem<T: Integer>(x: T, y: T) -> (T, T)
```

Simultaneous integer division and modulus

### `div_floor`

```rust
fn div_floor<T: Integer>(x: T, y: T) -> T
```

Floored integer division

### `mod_floor`

```rust
fn mod_floor<T: Integer>(x: T, y: T) -> T
```

Floored integer modulus

### `div_mod_floor`

```rust
fn div_mod_floor<T: Integer>(x: T, y: T) -> (T, T)
```

Simultaneous floored integer division and modulus

### `div_ceil`

```rust
fn div_ceil<T: Integer>(x: T, y: T) -> T
```

Ceiled integer division

### `gcd`

```rust
fn gcd<T: Integer>(x: T, y: T) -> T
```

Calculates the Greatest Common Divisor (GCD) of the number and `other`. The
result is always non-negative.

### `lcm`

```rust
fn lcm<T: Integer>(x: T, y: T) -> T
```

Calculates the Lowest Common Multiple (LCM) of the number and `other`.

### `gcd_lcm`

```rust
fn gcd_lcm<T: Integer>(x: T, y: T) -> (T, T)
```

Calculates the Greatest Common Divisor (GCD) and
Lowest Common Multiple (LCM) of the number and `other`.

### `multiply_and_divide`

```rust
fn multiply_and_divide<T: Integer + Clone>(r: T, a: T, b: T) -> T
```

Calculate r * a / b, avoiding overflows and fractions.

Assumes that b divides r * a evenly.

### `binomial`

```rust
fn binomial<T: Integer + Clone>(n: T, k: T) -> T
```

Calculate the binomial coefficient.

Note that this might overflow, depending on `T`. For the primitive integer
types, the following n are the largest ones possible such that there will
be no overflow for any k:

type | n
-----|---
u8   | 10
i8   |  9
u16  | 18
i16  | 17
u32  | 34
i32  | 33
u64  | 67
i64  | 66

For larger n, consider using a bigint type for `T`.

### `multinomial`

```rust
fn multinomial<T>(k: &[T]) -> T
where
    T: Add<&'a T, Output = T> + Integer + Clone
```

Calculate the multinomial coefficient.

## Macros

### `impl_integer_for_isize!`

### `impl_integer_for_usize!`

