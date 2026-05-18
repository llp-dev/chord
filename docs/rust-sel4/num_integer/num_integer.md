**num_integer**

# Module: num_integer

## Contents

**Structs**

- [`ExtendedGcd`](#extendedgcd) - Greatest common divisor and Bézout coefficients
- [`IterBinomial`](#iterbinomial) - An iterator over binomial coefficients.

**Functions**

- [`binomial`](#binomial) - Calculate the binomial coefficient.
- [`div_ceil`](#div_ceil) - Ceiled integer division
- [`div_floor`](#div_floor) - Floored integer division
- [`div_mod_floor`](#div_mod_floor) - Simultaneous floored integer division and modulus
- [`div_rem`](#div_rem) - Simultaneous integer division and modulus
- [`gcd`](#gcd) - Calculates the Greatest Common Divisor (GCD) of the number and `other`. The
- [`gcd_lcm`](#gcd_lcm) - Calculates the Greatest Common Divisor (GCD) and
- [`lcm`](#lcm) - Calculates the Lowest Common Multiple (LCM) of the number and `other`.
- [`mod_floor`](#mod_floor) - Floored integer modulus
- [`multinomial`](#multinomial) - Calculate the multinomial coefficient.

**Traits**

- [`Integer`](#integer)

---

## num_integer::ExtendedGcd

*Struct*

Greatest common divisor and Bézout coefficients

```no_build
let e = isize::extended_gcd(a, b);
assert_eq!(e.gcd, e.x*a + e.y*b);
```

**Generic Parameters:**
- A

**Fields:**
- `gcd: A`
- `x: A`
- `y: A`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ExtendedGcd<A>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ExtendedGcd<A>) -> bool`



## num_integer::Integer

*Trait*

**Methods:**

- `div_floor`: Floored integer division.
- `mod_floor`: Floored integer modulo, satisfying:
- `div_ceil`: Ceiled integer division.
- `gcd`: Greatest Common Divisor (GCD).
- `lcm`: Lowest Common Multiple (LCM).
- `gcd_lcm`: Greatest Common Divisor (GCD) and
- `extended_gcd`: Greatest common divisor and Bézout coefficients.
- `extended_gcd_lcm`: Greatest common divisor, least common multiple, and Bézout coefficients.
- `divides`: Deprecated, use `is_multiple_of` instead.
- `is_multiple_of`: Returns `true` if `self` is a multiple of `other`.
- `is_even`: Returns `true` if the number is even.
- `is_odd`: Returns `true` if the number is odd.
- `div_rem`: Simultaneous truncated integer division and modulus.
- `div_mod_floor`: Simultaneous floored integer division and modulus.
- `next_multiple_of`: Rounds up to nearest multiple of argument.
- `prev_multiple_of`: Rounds down to nearest multiple of argument.
- `dec`: Decrements self by one.
- `inc`: Increments self by one.



## num_integer::IterBinomial

*Struct*

An iterator over binomial coefficients.

**Generic Parameters:**
- T

**Methods:**

- `fn new(n: T) -> IterBinomial<T>` - For a given n, iterate over all binomial coefficients binomial(n, k), for k=0...n.

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<T>`



## num_integer::binomial

*Function*

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

```rust
fn binomial<T>(n: T, k: T) -> T
```



## num_integer::div_ceil

*Function*

Ceiled integer division

```rust
fn div_ceil<T>(x: T, y: T) -> T
```



## num_integer::div_floor

*Function*

Floored integer division

```rust
fn div_floor<T>(x: T, y: T) -> T
```



## num_integer::div_mod_floor

*Function*

Simultaneous floored integer division and modulus

```rust
fn div_mod_floor<T>(x: T, y: T) -> (T, T)
```



## num_integer::div_rem

*Function*

Simultaneous integer division and modulus

```rust
fn div_rem<T>(x: T, y: T) -> (T, T)
```



## num_integer::gcd

*Function*

Calculates the Greatest Common Divisor (GCD) of the number and `other`. The
result is always non-negative.

```rust
fn gcd<T>(x: T, y: T) -> T
```



## num_integer::gcd_lcm

*Function*

Calculates the Greatest Common Divisor (GCD) and
Lowest Common Multiple (LCM) of the number and `other`.

```rust
fn gcd_lcm<T>(x: T, y: T) -> (T, T)
```



## num_integer::lcm

*Function*

Calculates the Lowest Common Multiple (LCM) of the number and `other`.

```rust
fn lcm<T>(x: T, y: T) -> T
```



## num_integer::mod_floor

*Function*

Floored integer modulus

```rust
fn mod_floor<T>(x: T, y: T) -> T
```



## num_integer::multinomial

*Function*

Calculate the multinomial coefficient.

```rust
fn multinomial<T>(k: &[T]) -> T
```



