*[num_traits](../index.md) / [cast](index.md)*

---

# Module `cast`

## Contents

- [Traits](#traits)
  - [`ToPrimitive`](#toprimitive)
  - [`FromPrimitive`](#fromprimitive)
  - [`NumCast`](#numcast)
  - [`AsPrimitive`](#asprimitive)
- [Functions](#functions)
  - [`cast`](#cast)
- [Macros](#macros)
  - [`impl_to_primitive_int_to_int!`](#impl-to-primitive-int-to-int)
  - [`impl_to_primitive_int_to_uint!`](#impl-to-primitive-int-to-uint)
  - [`impl_to_primitive_int!`](#impl-to-primitive-int)
  - [`impl_to_primitive_uint_to_int!`](#impl-to-primitive-uint-to-int)
  - [`impl_to_primitive_uint_to_uint!`](#impl-to-primitive-uint-to-uint)
  - [`impl_to_primitive_uint!`](#impl-to-primitive-uint)
  - [`impl_to_primitive_float_to_float!`](#impl-to-primitive-float-to-float)
  - [`float_to_int_unchecked!`](#float-to-int-unchecked)
  - [`impl_to_primitive_float_to_signed_int!`](#impl-to-primitive-float-to-signed-int)
  - [`impl_to_primitive_float_to_unsigned_int!`](#impl-to-primitive-float-to-unsigned-int)
  - [`impl_to_primitive_float!`](#impl-to-primitive-float)
  - [`impl_from_primitive!`](#impl-from-primitive)
  - [`impl_to_primitive_wrapping!`](#impl-to-primitive-wrapping)
  - [`impl_from_primitive_wrapping!`](#impl-from-primitive-wrapping)
  - [`impl_num_cast!`](#impl-num-cast)
  - [`impl_as_primitive!`](#impl-as-primitive)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ToPrimitive`](#toprimitive) | trait | A generic trait for converting a value to a number. |
| [`FromPrimitive`](#fromprimitive) | trait | A generic trait for converting a number to a value. |
| [`NumCast`](#numcast) | trait | An interface for casting between machine scalars. |
| [`AsPrimitive`](#asprimitive) | trait | A generic interface for casting between machine scalars with the `as` operator, which admits narrowing and precision loss. |
| [`cast`](#cast) | fn | Cast from one machine scalar to another. |
| [`impl_to_primitive_int_to_int!`](#impl-to-primitive-int-to-int) | macro |  |
| [`impl_to_primitive_int_to_uint!`](#impl-to-primitive-int-to-uint) | macro |  |
| [`impl_to_primitive_int!`](#impl-to-primitive-int) | macro |  |
| [`impl_to_primitive_uint_to_int!`](#impl-to-primitive-uint-to-int) | macro |  |
| [`impl_to_primitive_uint_to_uint!`](#impl-to-primitive-uint-to-uint) | macro |  |
| [`impl_to_primitive_uint!`](#impl-to-primitive-uint) | macro |  |
| [`impl_to_primitive_float_to_float!`](#impl-to-primitive-float-to-float) | macro |  |
| [`float_to_int_unchecked!`](#float-to-int-unchecked) | macro |  |
| [`impl_to_primitive_float_to_signed_int!`](#impl-to-primitive-float-to-signed-int) | macro |  |
| [`impl_to_primitive_float_to_unsigned_int!`](#impl-to-primitive-float-to-unsigned-int) | macro |  |
| [`impl_to_primitive_float!`](#impl-to-primitive-float) | macro |  |
| [`impl_from_primitive!`](#impl-from-primitive) | macro |  |
| [`impl_to_primitive_wrapping!`](#impl-to-primitive-wrapping) | macro |  |
| [`impl_from_primitive_wrapping!`](#impl-from-primitive-wrapping) | macro |  |
| [`impl_num_cast!`](#impl-num-cast) | macro |  |
| [`impl_as_primitive!`](#impl-as-primitive) | macro |  |

## Traits

### `ToPrimitive`

```rust
trait ToPrimitive { ... }
```

A generic trait for converting a value to a number.

A value can be represented by the target type when it lies within
the range of scalars supported by the target type.
For example, a negative integer cannot be represented by an unsigned
integer type, and an `i64` with a very high magnitude might not be
convertible to an `i32`.
On the other hand, conversions with possible precision loss or truncation
are admitted, like an `f32` with a decimal part to an integer type, or
even a large `f64` saturating to `f32` infinity.

#### Required Methods

- `fn to_i64(&self) -> Option<i64>`

  Converts the value of `self` to an `i64`. If the value cannot be

- `fn to_u64(&self) -> Option<u64>`

  Converts the value of `self` to a `u64`. If the value cannot be

#### Provided Methods

- `fn to_isize(&self) -> Option<isize>`

  Converts the value of `self` to an `isize`. If the value cannot be

- `fn to_i8(&self) -> Option<i8>`

  Converts the value of `self` to an `i8`. If the value cannot be

- `fn to_i16(&self) -> Option<i16>`

  Converts the value of `self` to an `i16`. If the value cannot be

- `fn to_i32(&self) -> Option<i32>`

  Converts the value of `self` to an `i32`. If the value cannot be

- `fn to_i128(&self) -> Option<i128>`

  Converts the value of `self` to an `i128`. If the value cannot be

- `fn to_usize(&self) -> Option<usize>`

  Converts the value of `self` to a `usize`. If the value cannot be

- `fn to_u8(&self) -> Option<u8>`

  Converts the value of `self` to a `u8`. If the value cannot be

- `fn to_u16(&self) -> Option<u16>`

  Converts the value of `self` to a `u16`. If the value cannot be

- `fn to_u32(&self) -> Option<u32>`

  Converts the value of `self` to a `u32`. If the value cannot be

- `fn to_u128(&self) -> Option<u128>`

  Converts the value of `self` to a `u128`. If the value cannot be

- `fn to_f32(&self) -> Option<f32>`

  Converts the value of `self` to an `f32`. Overflows may map to positive

- `fn to_f64(&self) -> Option<f64>`

  Converts the value of `self` to an `f64`. Overflows may map to positive

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

### `FromPrimitive`

```rust
trait FromPrimitive: Sized { ... }
```

A generic trait for converting a number to a value.

A value can be represented by the target type when it lies within
the range of scalars supported by the target type.
For example, a negative integer cannot be represented by an unsigned
integer type, and an `i64` with a very high magnitude might not be
convertible to an `i32`.
On the other hand, conversions with possible precision loss or truncation
are admitted, like an `f32` with a decimal part to an integer type, or
even a large `f64` saturating to `f32` infinity.

#### Required Methods

- `fn from_i64(n: i64) -> Option<Self>`

  Converts an `i64` to return an optional value of this type. If the

- `fn from_u64(n: u64) -> Option<Self>`

  Converts an `u64` to return an optional value of this type. If the

#### Provided Methods

- `fn from_isize(n: isize) -> Option<Self>`

  Converts an `isize` to return an optional value of this type. If the

- `fn from_i8(n: i8) -> Option<Self>`

  Converts an `i8` to return an optional value of this type. If the

- `fn from_i16(n: i16) -> Option<Self>`

  Converts an `i16` to return an optional value of this type. If the

- `fn from_i32(n: i32) -> Option<Self>`

  Converts an `i32` to return an optional value of this type. If the

- `fn from_i128(n: i128) -> Option<Self>`

  Converts an `i128` to return an optional value of this type. If the

- `fn from_usize(n: usize) -> Option<Self>`

  Converts a `usize` to return an optional value of this type. If the

- `fn from_u8(n: u8) -> Option<Self>`

  Converts an `u8` to return an optional value of this type. If the

- `fn from_u16(n: u16) -> Option<Self>`

  Converts an `u16` to return an optional value of this type. If the

- `fn from_u32(n: u32) -> Option<Self>`

  Converts an `u32` to return an optional value of this type. If the

- `fn from_u128(n: u128) -> Option<Self>`

  Converts an `u128` to return an optional value of this type. If the

- `fn from_f32(n: f32) -> Option<Self>`

  Converts a `f32` to return an optional value of this type. If the

- `fn from_f64(n: f64) -> Option<Self>`

  Converts a `f64` to return an optional value of this type. If the

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

### `NumCast`

```rust
trait NumCast: Sized + ToPrimitive { ... }
```

An interface for casting between machine scalars.

#### Required Methods

- `fn from<T: ToPrimitive>(n: T) -> Option<Self>`

  Creates a number from another value that can be converted into

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

### `AsPrimitive<T>`

```rust
trait AsPrimitive<T>: 'static + Copy
where
    T: 'static + Copy { ... }
```

A generic interface for casting between machine scalars with the
`as` operator, which admits narrowing and precision loss.
Implementers of this trait `AsPrimitive` should behave like a primitive
numeric type (e.g. a newtype around another primitive), and the
intended conversion must never fail.

# Examples

```rust
use num_traits::AsPrimitive;
let three: i32 = (3.14159265f32).as_();
assert_eq!(three, 3);
```

# Safety

**In Rust versions before 1.45.0**, some uses of the `as` operator were not entirely safe.
In particular, it was undefined behavior if
a truncated floating point value could not fit in the target integer
type ([#10184](https://github.com/rust-lang/rust/issues/10184)).

```ignore
use num_traits::AsPrimitive;
let x: u8 = (1.04E+17).as_(); // UB
```


#### Required Methods

- `fn as_(self) -> T`

  Convert a value to another, using the `as` operator.

#### Implementors

- `bool`
- `char`
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

### `cast`

```rust
fn cast<T: NumCast, U: NumCast>(n: T) -> Option<U>
```

Cast from one machine scalar to another.

# Examples

```rust
use num_traits as num;
let twenty: f32 = num::cast(0x14).unwrap();
assert_eq!(twenty, 20f32);
```


## Macros

### `impl_to_primitive_int_to_int!`

### `impl_to_primitive_int_to_uint!`

### `impl_to_primitive_int!`

### `impl_to_primitive_uint_to_int!`

### `impl_to_primitive_uint_to_uint!`

### `impl_to_primitive_uint!`

### `impl_to_primitive_float_to_float!`

### `float_to_int_unchecked!`

### `impl_to_primitive_float_to_signed_int!`

### `impl_to_primitive_float_to_unsigned_int!`

### `impl_to_primitive_float!`

### `impl_from_primitive!`

### `impl_to_primitive_wrapping!`

### `impl_from_primitive_wrapping!`

### `impl_num_cast!`

### `impl_as_primitive!`

