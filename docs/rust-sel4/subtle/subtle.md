**subtle**

# Module: subtle

## Contents

**Macros**

- [`generate_integer_conditional_select`](#generate_integer_conditional_select)
- [`generate_integer_equal`](#generate_integer_equal) - Given the bit-width `$bit_width` and the corresponding primitive
- [`generate_unsigned_integer_greater`](#generate_unsigned_integer_greater)
- [`to_signed_int`](#to_signed_int)

**Structs**

- [`BlackBox`](#blackbox) - Wrapper type which implements an optimization barrier for all accesses.
- [`Choice`](#choice) - The `Choice` struct represents a choice for use in conditional assignment.
- [`CtOption`](#ctoption) - The `CtOption<T>` type represents an optional value similar to the

**Functions**

- [`black_box`](#black_box) - This function is a best-effort attempt to prevent the compiler from knowing

**Traits**

- [`ConditionallyNegatable`](#conditionallynegatable) - A type which can be conditionally negated in constant time.
- [`ConditionallySelectable`](#conditionallyselectable) - A type which can be conditionally selected in constant time.
- [`ConstantTimeEq`](#constanttimeeq) - An `Eq`-like trait that produces a `Choice` instead of a `bool`.
- [`ConstantTimeGreater`](#constanttimegreater) - A type which can be compared in some manner and be determined to be greater
- [`ConstantTimeLess`](#constanttimeless) - A type which can be compared in some manner and be determined to be less

---

## subtle::BlackBox

*Struct*

Wrapper type which implements an optimization barrier for all accesses.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Methods:**

- `fn new(value: T) -> Self` - Constructs a new instance of `BlackBox` which will wrap the specified value.
- `fn get(self: Self) -> T` - Read the inner value, applying an optimization barrier on access.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BlackBox<T>`



## subtle::Choice

*Struct*

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

[rust-timing-shield]:
https://www.chosenplaintext.ca/open-source/rust-timing-shield/security

**Tuple Struct**: `(u8)`

**Methods:**

- `fn unwrap_u8(self: &Self) -> u8` - Unwrap the `Choice` wrapper to reveal the underlying `u8`.

**Traits:** Copy

**Trait Implementations:**

- **ConstantTimeEq**
  - `fn ct_eq(self: &Self, rhs: &Choice) -> Choice`
- **BitAnd**
  - `fn bitand(self: Self, rhs: Choice) -> Choice`
- **From**
  - `fn from(input: u8) -> Choice`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, rhs: Choice)`
- **Not**
  - `fn not(self: Self) -> Choice`
- **BitOr**
  - `fn bitor(self: Self, rhs: Choice) -> Choice`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, rhs: Choice)`
- **ConditionallySelectable**
  - `fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: Choice)`
- **Clone**
  - `fn clone(self: &Self) -> Choice`
- **BitXor**
  - `fn bitxor(self: Self, rhs: Choice) -> Choice`



## subtle::ConditionallyNegatable

*Trait*

A type which can be conditionally negated in constant time.

# Note

A generic implementation of `ConditionallyNegatable` is provided
for types `T` which are `ConditionallySelectable` and have `Neg`
implemented on `&T`.

**Methods:**

- `conditional_negate`: Negate `self` if `choice == Choice(1)`; otherwise, leave it



## subtle::ConditionallySelectable

*Trait*

A type which can be conditionally selected in constant time.

This trait also provides generic implementations of conditional
assignment and conditional swaps.

**Methods:**

- `conditional_select`: Select `a` or `b` according to `choice`.
- `conditional_assign`: Conditionally assign `other` to `self`, according to `choice`.
- `conditional_swap`: Conditionally swap `self` and `other` if `choice == 1`; otherwise,



## subtle::ConstantTimeEq

*Trait*

An `Eq`-like trait that produces a `Choice` instead of a `bool`.

# Example

```
use subtle::ConstantTimeEq;
let x: u8 = 5;
let y: u8 = 13;

assert_eq!(x.ct_eq(&y).unwrap_u8(), 0);
assert_eq!(x.ct_eq(&x).unwrap_u8(), 1);
```

**Methods:**

- `ct_eq`: Determine if two items are equal.
- `ct_ne`: Determine if two items are NOT equal.



## subtle::ConstantTimeGreater

*Trait*

A type which can be compared in some manner and be determined to be greater
than another of the same type.

**Methods:**

- `ct_gt`: Determine whether `self > other`.



## subtle::ConstantTimeLess

*Trait*

A type which can be compared in some manner and be determined to be less
than another of the same type.

**Methods:**

- `ct_lt`: Determine whether `self < other`.



## subtle::CtOption

*Struct*

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

**Generic Parameters:**
- T

**Fields:**
- `value: T`
- `is_some: Choice`

**Methods:**

- `fn new(value: T, is_some: Choice) -> CtOption<T>` - This method is used to construct a new `CtOption<T>` and takes
- `fn expect(self: Self, msg: &str) -> T` - Returns the contained value, consuming the `self` value.
- `fn unwrap(self: Self) -> T` - This returns the underlying value but panics if it
- `fn unwrap_or(self: Self, def: T) -> T` - This returns the underlying value if it is `Some`
- `fn unwrap_or_else<F>(self: Self, f: F) -> T` - This returns the underlying value if it is `Some`
- `fn is_some(self: &Self) -> Choice` - Returns a true `Choice` if this value is `Some`.
- `fn is_none(self: &Self) -> Choice` - Returns a true `Choice` if this value is `None`.
- `fn map<U, F>(self: Self, f: F) -> CtOption<U>` - Returns a `None` value if the option is `None`, otherwise
- `fn and_then<U, F>(self: Self, f: F) -> CtOption<U>` - Returns a `None` value if the option is `None`, otherwise
- `fn or_else<F>(self: Self, f: F) -> CtOption<T>` - Returns `self` if it contains a value, and otherwise returns the result of
- `fn into_option(self: Self) -> Option<T>` - Convert the `CtOption<T>` wrapper into an `Option<T>`, depending on whether

**Traits:** Copy

**Trait Implementations:**

- **ConditionallySelectable**
  - `fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self`
- **ConstantTimeEq**
  - `fn ct_eq(self: &Self, rhs: &CtOption<T>) -> Choice` - Two `CtOption<T>`s are equal if they are both `Some` and
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CtOption<T>`



## subtle::black_box

*Function*

This function is a best-effort attempt to prevent the compiler from knowing
anything about the value of the returned `u8`, other than its type.

Because we want to support stable Rust, we don't have access to inline
assembly or test::black_box, so we use the fact that volatile values will
never be elided to register values.

Note: Rust's notion of "volatile" is subject to change over time. While this
code may break in a non-destructive way in the future, “constant-time” code
is a continually moving target, and this is better than doing nothing.

```rust
fn black_box<T>(input: T) -> T
```



## subtle::generate_integer_conditional_select

*Declarative Macro*

```rust
macro_rules! generate_integer_conditional_select {
    ($($t:tt)*) => { ... };
}
```



## subtle::generate_integer_equal

*Declarative Macro*

Given the bit-width `$bit_width` and the corresponding primitive
unsigned and signed types `$t_u` and `$t_i` respectively, generate
an `ConstantTimeEq` implementation.

```rust
macro_rules! generate_integer_equal {
    ($t_u:ty, $t_i:ty, $bit_width:expr) => { ... };
}
```



## subtle::generate_unsigned_integer_greater

*Declarative Macro*

```rust
macro_rules! generate_unsigned_integer_greater {
    ($t_u: ty, $bit_width: expr) => { ... };
}
```



## subtle::to_signed_int

*Declarative Macro*

```rust
macro_rules! to_signed_int {
    (u8) => { ... };
    (u16) => { ... };
    (u32) => { ... };
    (u64) => { ... };
    (u128) => { ... };
    (i8) => { ... };
    (i16) => { ... };
    (i32) => { ... };
    (i64) => { ... };
    (i128) => { ... };
}
```



