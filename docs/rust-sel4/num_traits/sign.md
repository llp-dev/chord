**num_traits > sign**

# Module: sign

## Contents

**Functions**

- [`abs`](#abs) - Computes the absolute value.
- [`abs_sub`](#abs_sub) - The positive difference of two numbers.
- [`signum`](#signum) - Returns the sign of the number.

**Traits**

- [`Signed`](#signed) - Useful functions for signed numbers (i.e. numbers that can be negative).
- [`Unsigned`](#unsigned) - A trait for values which cannot be negative

---

## num_traits::sign::Signed

*Trait*

Useful functions for signed numbers (i.e. numbers that can be negative).

**Methods:**

- `abs`: Computes the absolute value.
- `abs_sub`: The positive difference of two numbers.
- `signum`: Returns the sign of the number.
- `is_positive`: Returns true if the number is positive and false if the number is zero or negative.
- `is_negative`: Returns true if the number is negative and false if the number is zero or positive.



## num_traits::sign::Unsigned

*Trait*

A trait for values which cannot be negative



## num_traits::sign::abs

*Function*

Computes the absolute value.

For `f32` and `f64`, `NaN` will be returned if the number is `NaN`

For signed integers, `::MIN` will be returned if the number is `::MIN`.

```rust
fn abs<T>(value: T) -> T
```



## num_traits::sign::abs_sub

*Function*

The positive difference of two numbers.

Returns zero if `x` is less than or equal to `y`, otherwise the difference
between `x` and `y` is returned.

```rust
fn abs_sub<T>(x: T, y: T) -> T
```



## num_traits::sign::signum

*Function*

Returns the sign of the number.

For `f32` and `f64`:

* `1.0` if the number is positive, `+0.0` or `INFINITY`
* `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
* `NaN` if the number is `NaN`

For signed integers:

* `0` if the number is zero
* `1` if the number is positive
* `-1` if the number is negative

```rust
fn signum<T>(value: T) -> T
```



