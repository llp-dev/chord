# Crate `subtle`

# subtle [![](https://img.shields.io/crates/v/subtle.svg)](https://crates.io/crates/subtle) [![](https://img.shields.io/badge/dynamic/json.svg?label=docs&uri=https%3A%2F%2Fcrates.io%2Fapi%2Fv1%2Fcrates%2Fsubtle%2Fversions&query=%24.versions%5B0%5D.num&colorB=4F74A6)](https://doc.dalek.rs/subtle) [![](https://travis-ci.org/dalek-cryptography/subtle.svg?branch=master)](https://travis-ci.org/dalek-cryptography/subtle)

**Pure-Rust traits and utilities for constant-time cryptographic implementations.**

It consists of a `Choice` type, and a collection of traits using `Choice`
instead of `bool` which are intended to execute in constant-time.  The `Choice`
type is a wrapper around a `u8` that holds a `0` or `1`.

```toml
subtle = "2.6"
```

This crate represents a “best-effort” attempt, since side-channels
are ultimately a property of a deployed cryptographic system
including the hardware it runs on, not just of software.

The traits are implemented using bitwise operations, and should execute in
constant time provided that a) the bitwise operations are constant-time and
b) the bitwise operations are not recognized as a conditional assignment and
optimized back into a branch.

For a compiler to recognize that bitwise operations represent a conditional
assignment, it needs to know that the value used to generate the bitmasks is
really a boolean `i1` rather than an `i8` byte value. In an attempt to
prevent this refinement, the crate tries to hide the value of a `Choice`'s
inner `u8` by passing it through a volatile read. For more information, see
the _About_ section below.

Rust versions from 1.51 or higher have const generics support. You may enable
`const-generics` feautre to have `subtle` traits implemented for arrays `[T; N]`.

Versions prior to `2.2` recommended use of the `nightly` feature to enable an
optimization barrier; this is not required in versions `2.2` and above.

Note: the `subtle` crate contains `debug_assert`s to check invariants during
debug builds. These invariant checks involve secret-dependent branches, and
are not present when compiled in release mode. This crate is intended to be
used in release mode.

## Documentation

Documentation is available [here][docs].

## Minimum Supported Rust Version

Rust **1.41** or higher.

Minimum supported Rust version can be changed in the future, but it will be done with a minor version bump.

## About

This library aims to be the Rust equivalent of Go’s `crypto/subtle` module.

Old versions of the optimization barrier in `impl From<u8> for Choice` were
based on Tim Maclean's [work on `rust-timing-shield`][rust-timing-shield],
which attempts to provide a more comprehensive approach for preventing
software side-channels in Rust code.
From version `2.2`, it was based on Diane Hosfelt and Amber Sprenkels' work on
"Secret Types in Rust".

`subtle` is authored by isis agora lovecruft and Henry de Valence.

## Warning

This code is a low-level library, intended for specific use-cases implementing
cryptographic protocols.  It represents a best-effort attempt to protect
against some software side-channels.  Because side-channel resistance is not a
property of software alone, but of software together with hardware, any such
effort is fundamentally limited.

**USE AT YOUR OWN RISK**



## Contents

- [Structs](#structs)
  - [`Choice`](#choice)
  - [`CtOption`](#ctoption)
  - [`BlackBox`](#blackbox)
- [Traits](#traits)
  - [`ConstantTimeEq`](#constanttimeeq)
  - [`ConditionallySelectable`](#conditionallyselectable)
  - [`ConditionallyNegatable`](#conditionallynegatable)
  - [`ConstantTimeGreater`](#constanttimegreater)
  - [`ConstantTimeLess`](#constanttimeless)
- [Functions](#functions)
  - [`black_box`](#black-box)
- [Macros](#macros)
  - [`generate_integer_equal!`](#generate-integer-equal)
  - [`to_signed_int!`](#to-signed-int)
  - [`generate_integer_conditional_select!`](#generate-integer-conditional-select)
  - [`generate_unsigned_integer_greater!`](#generate-unsigned-integer-greater)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Choice`](#choice) | struct | The `Choice` struct represents a choice for use in conditional assignment. |
| [`CtOption`](#ctoption) | struct | The `CtOption<T>` type represents an optional value similar to the [`Option<T>`](core::option::Option) type but is intended for use in constant time APIs. |
| [`BlackBox`](#blackbox) | struct | Wrapper type which implements an optimization barrier for all accesses. |
| [`ConstantTimeEq`](#constanttimeeq) | trait | An `Eq`-like trait that produces a `Choice` instead of a `bool`. |
| [`ConditionallySelectable`](#conditionallyselectable) | trait | A type which can be conditionally selected in constant time. |
| [`ConditionallyNegatable`](#conditionallynegatable) | trait | A type which can be conditionally negated in constant time. |
| [`ConstantTimeGreater`](#constanttimegreater) | trait | A type which can be compared in some manner and be determined to be greater than another of the same type. |
| [`ConstantTimeLess`](#constanttimeless) | trait | A type which can be compared in some manner and be determined to be less than another of the same type. |
| [`black_box`](#black-box) | fn | This function is a best-effort attempt to prevent the compiler from knowing anything about the value of the returned `u8`, other than its type. |
| [`generate_integer_equal!`](#generate-integer-equal) | macro | Given the bit-width `$bit_width` and the corresponding primitive unsigned and signed types `$t_u` and `$t_i` respectively, generate an `ConstantTimeEq` implementation. |
| [`to_signed_int!`](#to-signed-int) | macro |  |
| [`generate_integer_conditional_select!`](#generate-integer-conditional-select) | macro |  |
| [`generate_unsigned_integer_greater!`](#generate-unsigned-integer-greater) | macro |  |

## Structs

### `Choice`

```rust
struct Choice(u8);
```

The `Choice` struct represents a choice for use in conditional assignment.

It is a wrapper around a `u8`, which should have the value either `1` (true)
or `0` (false).

The conversion from `u8` to `Choice` passes the value through an optimization
barrier, as a best-effort attempt to prevent the compiler from inferring that
the `Choice` value is a boolean. This strategy is based on Tim Maclean's
[work on `rust-timing-shield`][rust-timing-shield], which attempts to provide
a more comprehensive approach for preventing software side-channels in Rust
code.

The `Choice` struct implements operators for AND, OR, XOR, and NOT, to allow
combining `Choice` values. These operations do not short-circuit.


#### Implementations

- <span id="choice-unwrap-u8"></span>`fn unwrap_u8(&self) -> u8`

  Unwrap the `Choice` wrapper to reveal the underlying `u8`.

  

  # Note

  

  This function only exists as an **escape hatch** for the rare case

  where it's not possible to use one of the `subtle`-provided

  trait impls.

  

  **To convert a `Choice` to a `bool`, use the `From` implementation instead.**

#### Trait Implementations

##### `impl BitAnd for Choice`

- <span id="choice-bitand-type-output"></span>`type Output = Choice`

- <span id="choice-bitand"></span>`fn bitand(self, rhs: Choice) -> Choice` — [`Choice`](#choice)

##### `impl BitAndAssign for Choice`

- <span id="choice-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: Choice)` — [`Choice`](#choice)

##### `impl BitOr for Choice`

- <span id="choice-bitor-type-output"></span>`type Output = Choice`

- <span id="choice-bitor"></span>`fn bitor(self, rhs: Choice) -> Choice` — [`Choice`](#choice)

##### `impl BitOrAssign for Choice`

- <span id="choice-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: Choice)` — [`Choice`](#choice)

##### `impl BitXor for Choice`

- <span id="choice-bitxor-type-output"></span>`type Output = Choice`

- <span id="choice-bitxor"></span>`fn bitxor(self, rhs: Choice) -> Choice` — [`Choice`](#choice)

##### `impl BitXorAssign for Choice`

- <span id="choice-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: Choice)` — [`Choice`](#choice)

##### `impl Clone for Choice`

- <span id="choice-clone"></span>`fn clone(&self) -> Choice` — [`Choice`](#choice)

##### `impl ConditionallySelectable for Choice`

- <span id="choice-conditionallyselectable-conditional-select"></span>`fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self` — [`Choice`](#choice)

##### `impl ConstantTimeEq for Choice`

- <span id="choice-constanttimeeq-ct-eq"></span>`fn ct_eq(&self, rhs: &Choice) -> Choice` — [`Choice`](#choice)

##### `impl Copy for Choice`

##### `impl Debug for Choice`

- <span id="choice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Not for Choice`

- <span id="choice-not-type-output"></span>`type Output = Choice`

- <span id="choice-not"></span>`fn not(self) -> Choice` — [`Choice`](#choice)

### `CtOption<T>`

```rust
struct CtOption<T> {
    value: T,
    is_some: Choice,
}
```

The `CtOption<T>` type represents an optional value similar to the
[`Option<T>`](core::option::Option) type but is intended for
use in constant time APIs.

Any given `CtOption<T>` is either `Some` or `None`, but unlike
`Option<T>` these variants are not exposed. The
[`is_some()`](CtOption::is_some) method is used to determine if
the value is `Some`, and [`unwrap_or()`](CtOption::unwrap_or) and
[`unwrap_or_else()`](CtOption::unwrap_or_else) methods are
provided to access the underlying value. The value can also be
obtained with [`unwrap()`](CtOption::unwrap) but this will panic
if it is `None`.

Functions that are intended to be constant time may not produce
valid results for all inputs, such as square root and inversion
operations in finite field arithmetic. Returning an `Option<T>`
from these functions makes it difficult for the caller to reason
about the result in constant time, and returning an incorrect
value burdens the caller and increases the chance of bugs.

#### Implementations

- <span id="ctoption-new"></span>`fn new(value: T, is_some: Choice) -> CtOption<T>` — [`Choice`](#choice), [`CtOption`](#ctoption)

  This method is used to construct a new `CtOption<T>` and takes

  a value of type `T`, and a `Choice` that determines whether

  the optional value should be `Some` or not. If `is_some` is

  false, the value will still be stored but its value is never

  exposed.

- <span id="ctoption-expect"></span>`fn expect(self, msg: &str) -> T`

  Returns the contained value, consuming the `self` value.

  

  # Panics

  

  Panics if the value is none with a custom panic message provided by

  `msg`.

- <span id="ctoption-unwrap"></span>`fn unwrap(self) -> T`

  This returns the underlying value but panics if it

  is not `Some`.

- <span id="ctoption-unwrap-or"></span>`fn unwrap_or(self, def: T) -> T`

  This returns the underlying value if it is `Some`

  or the provided value otherwise.

- <span id="ctoption-unwrap-or-else"></span>`fn unwrap_or_else<F>(self, f: F) -> T`

  This returns the underlying value if it is `Some`

  or the value produced by the provided closure otherwise.

  

  This operates in constant time, because the provided closure

  is always called.

- <span id="ctoption-is-some"></span>`fn is_some(&self) -> Choice` — [`Choice`](#choice)

  Returns a true `Choice` if this value is `Some`.

- <span id="ctoption-is-none"></span>`fn is_none(&self) -> Choice` — [`Choice`](#choice)

  Returns a true `Choice` if this value is `None`.

- <span id="ctoption-map"></span>`fn map<U, F>(self, f: F) -> CtOption<U>` — [`CtOption`](#ctoption)

  Returns a `None` value if the option is `None`, otherwise

  returns a `CtOption` enclosing the value of the provided closure.

  The closure is given the enclosed value or, if the option is

  `None`, it is provided a dummy value computed using

  `Default::default()`.

  

  This operates in constant time, because the provided closure

  is always called.

- <span id="ctoption-and-then"></span>`fn and_then<U, F>(self, f: F) -> CtOption<U>` — [`CtOption`](#ctoption)

  Returns a `None` value if the option is `None`, otherwise

  returns the result of the provided closure. The closure is

  given the enclosed value or, if the option is `None`, it

  is provided a dummy value computed using `Default::default()`.

  

  This operates in constant time, because the provided closure

  is always called.

- <span id="ctoption-or-else"></span>`fn or_else<F>(self, f: F) -> CtOption<T>` — [`CtOption`](#ctoption)

  Returns `self` if it contains a value, and otherwise returns the result of

  calling `f`. The provided function `f` is always called.

- <span id="ctoption-into-option"></span>`fn into_option(self) -> Option<T>`

  Convert the `CtOption<T>` wrapper into an `Option<T>`, depending on whether

  the underlying `is_some` `Choice` was a `0` or a `1` once unwrapped.

  

  # Note

  

  This function exists to avoid ending up with ugly, verbose and/or bad handled

  conversions from the `CtOption<T>` wraps to an `Option<T>` or `Result<T, E>`.

  This implementation doesn't intend to be constant-time nor try to protect the

  leakage of the `T` since the `Option<T>` will do it anyways.

  

  It's equivalent to the corresponding `From` impl, however this version is

  friendlier for type inference.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for CtOption<T>`

- <span id="ctoption-clone"></span>`fn clone(&self) -> CtOption<T>` — [`CtOption`](#ctoption)

##### `impl<T: ConditionallySelectable> ConditionallySelectable for CtOption<T>`

- <span id="ctoption-conditionallyselectable-conditional-select"></span>`fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self` — [`Choice`](#choice)

##### `impl<T: ConstantTimeEq> ConstantTimeEq for CtOption<T>`

- <span id="ctoption-constanttimeeq-ct-eq"></span>`fn ct_eq(&self, rhs: &CtOption<T>) -> Choice` — [`CtOption`](#ctoption), [`Choice`](#choice)

  Two `CtOption<T>`s are equal if they are both `Some` and

  their values are equal, or both `None`.

##### `impl<T: marker::Copy> Copy for CtOption<T>`

##### `impl<T: fmt::Debug> Debug for CtOption<T>`

- <span id="ctoption-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlackBox<T: Copy>`

```rust
struct BlackBox<T: Copy>(T);
```

Wrapper type which implements an optimization barrier for all accesses.

#### Implementations

- <span id="blackbox-new"></span>`fn new(value: T) -> Self`

  Constructs a new instance of `BlackBox` which will wrap the specified value.

  

  All access to the inner value will be mediated by a `black_box` optimization barrier.

- <span id="blackbox-get"></span>`fn get(self) -> T`

  Read the inner value, applying an optimization barrier on access.

#### Trait Implementations

##### `impl<T: clone::Clone + Copy> Clone for BlackBox<T>`

- <span id="blackbox-clone"></span>`fn clone(&self) -> BlackBox<T>` — [`BlackBox`](#blackbox)

##### `impl<T: marker::Copy + Copy> Copy for BlackBox<T>`

##### `impl<T: fmt::Debug + Copy> Debug for BlackBox<T>`

- <span id="blackbox-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `ConstantTimeEq`

```rust
trait ConstantTimeEq { ... }
```

An `Eq`-like trait that produces a `Choice` instead of a `bool`.

# Example

```rust
use subtle::ConstantTimeEq;
let x: u8 = 5;
let y: u8 = 13;

assert_eq!(x.ct_eq(&y).unwrap_u8(), 0);
assert_eq!(x.ct_eq(&x).unwrap_u8(), 1);
```

#### Required Methods

- `fn ct_eq(&self, other: &Self) -> Choice`

  Determine if two items are equal.

#### Provided Methods

- `fn ct_ne(&self, other: &Self) -> Choice`

  Determine if two items are NOT equal.

#### Implementors

- [`Choice`](#choice)
- [`CtOption`](#ctoption)
- `[T]`
- `cmp::Ordering`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `ConditionallySelectable`

```rust
trait ConditionallySelectable: Copy { ... }
```

A type which can be conditionally selected in constant time.

This trait also provides generic implementations of conditional
assignment and conditional swaps.

#### Required Methods

- `fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self`

  Select `a` or `b` according to `choice`.

#### Provided Methods

- `fn conditional_assign(&mut self, other: &Self, choice: Choice)`

  Conditionally assign `other` to `self`, according to `choice`.

- `fn conditional_swap(a: &mut Self, b: &mut Self, choice: Choice)`

  Conditionally swap `self` and `other` if `choice == 1`; otherwise,

#### Implementors

- [`Choice`](#choice)
- [`CtOption`](#ctoption)
- `cmp::Ordering`
- `i16`
- `i32`
- `i64`
- `i8`
- `u16`
- `u32`
- `u64`
- `u8`

### `ConditionallyNegatable`

```rust
trait ConditionallyNegatable { ... }
```

A type which can be conditionally negated in constant time.

# Note

A generic implementation of `ConditionallyNegatable` is provided
for types `T` which are `ConditionallySelectable` and have `Neg`
implemented on `&T`.

#### Required Methods

- `fn conditional_negate(&mut self, choice: Choice)`

  Negate `self` if `choice == Choice(1)`; otherwise, leave it

#### Implementors

- `T`

### `ConstantTimeGreater`

```rust
trait ConstantTimeGreater { ... }
```

A type which can be compared in some manner and be determined to be greater
than another of the same type.

#### Required Methods

- `fn ct_gt(&self, other: &Self) -> Choice`

  Determine whether `self > other`.

#### Implementors

- `cmp::Ordering`
- `u16`
- `u32`
- `u64`
- `u8`

### `ConstantTimeLess`

```rust
trait ConstantTimeLess: ConstantTimeEq + ConstantTimeGreater { ... }
```

A type which can be compared in some manner and be determined to be less
than another of the same type.

#### Provided Methods

- `fn ct_lt(&self, other: &Self) -> Choice`

  Determine whether `self < other`.

#### Implementors

- `cmp::Ordering`
- `u16`
- `u32`
- `u64`
- `u8`

## Functions

### `black_box`

```rust
fn black_box<T: Copy>(input: T) -> T
```

This function is a best-effort attempt to prevent the compiler from knowing
anything about the value of the returned `u8`, other than its type.

Because we want to support stable Rust, we don't have access to inline
assembly or test::black_box, so we use the fact that volatile values will
never be elided to register values.

Note: Rust's notion of "volatile" is subject to change over time. While this
code may break in a non-destructive way in the future, “constant-time” code
is a continually moving target, and this is better than doing nothing.

## Macros

### `generate_integer_equal!`

Given the bit-width `$bit_width` and the corresponding primitive
unsigned and signed types `$t_u` and `$t_i` respectively, generate
an `ConstantTimeEq` implementation.

### `to_signed_int!`

### `generate_integer_conditional_select!`

### `generate_unsigned_integer_greater!`

