**num_traits > cast**

# Module: cast

## Contents

**Functions**

- [`cast`](#cast) - Cast from one machine scalar to another.

**Traits**

- [`AsPrimitive`](#asprimitive) - A generic interface for casting between machine scalars with the
- [`FromPrimitive`](#fromprimitive) - A generic trait for converting a number to a value.
- [`NumCast`](#numcast) - An interface for casting between machine scalars.
- [`ToPrimitive`](#toprimitive) - A generic trait for converting a value to a number.

---

## num_traits::cast::AsPrimitive

*Trait*

A generic interface for casting between machine scalars with the
`as` operator, which admits narrowing and precision loss.
Implementers of this trait `AsPrimitive` should behave like a primitive
numeric type (e.g. a newtype around another primitive), and the
intended conversion must never fail.

# Examples

```
# use num_traits::AsPrimitive;
let three: i32 = (3.14159265f32).as_();
assert_eq!(three, 3);
```

# Safety

**In Rust versions before 1.45.0**, some uses of the `as` operator were not entirely safe.
In particular, it was undefined behavior if
a truncated floating point value could not fit in the target integer
type ([#10184](https://github.com/rust-lang/rust/issues/10184)).

```ignore
# use num_traits::AsPrimitive;
let x: u8 = (1.04E+17).as_(); // UB
```


**Methods:**

- `as_`: Convert a value to another, using the `as` operator.



## num_traits::cast::FromPrimitive

*Trait*

A generic trait for converting a number to a value.

A value can be represented by the target type when it lies within
the range of scalars supported by the target type.
For example, a negative integer cannot be represented by an unsigned
integer type, and an `i64` with a very high magnitude might not be
convertible to an `i32`.
On the other hand, conversions with possible precision loss or truncation
are admitted, like an `f32` with a decimal part to an integer type, or
even a large `f64` saturating to `f32` infinity.

**Methods:**

- `from_isize`: Converts an `isize` to return an optional value of this type. If the
- `from_i8`: Converts an `i8` to return an optional value of this type. If the
- `from_i16`: Converts an `i16` to return an optional value of this type. If the
- `from_i32`: Converts an `i32` to return an optional value of this type. If the
- `from_i64`: Converts an `i64` to return an optional value of this type. If the
- `from_i128`: Converts an `i128` to return an optional value of this type. If the
- `from_usize`: Converts a `usize` to return an optional value of this type. If the
- `from_u8`: Converts an `u8` to return an optional value of this type. If the
- `from_u16`: Converts an `u16` to return an optional value of this type. If the
- `from_u32`: Converts an `u32` to return an optional value of this type. If the
- `from_u64`: Converts an `u64` to return an optional value of this type. If the
- `from_u128`: Converts an `u128` to return an optional value of this type. If the
- `from_f32`: Converts a `f32` to return an optional value of this type. If the
- `from_f64`: Converts a `f64` to return an optional value of this type. If the



## num_traits::cast::NumCast

*Trait*

An interface for casting between machine scalars.

**Methods:**

- `from`: Creates a number from another value that can be converted into



## num_traits::cast::ToPrimitive

*Trait*

A generic trait for converting a value to a number.

A value can be represented by the target type when it lies within
the range of scalars supported by the target type.
For example, a negative integer cannot be represented by an unsigned
integer type, and an `i64` with a very high magnitude might not be
convertible to an `i32`.
On the other hand, conversions with possible precision loss or truncation
are admitted, like an `f32` with a decimal part to an integer type, or
even a large `f64` saturating to `f32` infinity.

**Methods:**

- `to_isize`: Converts the value of `self` to an `isize`. If the value cannot be
- `to_i8`: Converts the value of `self` to an `i8`. If the value cannot be
- `to_i16`: Converts the value of `self` to an `i16`. If the value cannot be
- `to_i32`: Converts the value of `self` to an `i32`. If the value cannot be
- `to_i64`: Converts the value of `self` to an `i64`. If the value cannot be
- `to_i128`: Converts the value of `self` to an `i128`. If the value cannot be
- `to_usize`: Converts the value of `self` to a `usize`. If the value cannot be
- `to_u8`: Converts the value of `self` to a `u8`. If the value cannot be
- `to_u16`: Converts the value of `self` to a `u16`. If the value cannot be
- `to_u32`: Converts the value of `self` to a `u32`. If the value cannot be
- `to_u64`: Converts the value of `self` to a `u64`. If the value cannot be
- `to_u128`: Converts the value of `self` to a `u128`. If the value cannot be
- `to_f32`: Converts the value of `self` to an `f32`. Overflows may map to positive
- `to_f64`: Converts the value of `self` to an `f64`. Overflows may map to positive



## num_traits::cast::cast

*Function*

Cast from one machine scalar to another.

# Examples

```
# use num_traits as num;
let twenty: f32 = num::cast(0x14).unwrap();
assert_eq!(twenty, 20f32);
```


```rust
fn cast<T, U>(n: T) -> Option<U>
```



