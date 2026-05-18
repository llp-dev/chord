**num_traits > int**

# Module: int

## Contents

**Traits**

- [`PrimInt`](#primint) - Generic trait for primitive integers.

---

## num_traits::int::PrimInt

*Trait*

Generic trait for primitive integers.

The `PrimInt` trait is an abstraction over the builtin primitive integer types (e.g., `u8`,
`u32`, `isize`, `i128`, ...). It inherits the basic numeric traits and extends them with
bitwise operators and non-wrapping arithmetic.

The trait explicitly inherits `Copy`, `Eq`, `Ord`, and `Sized`. The intention is that all
types implementing this trait behave like primitive types that are passed by value by default
and behave like builtin integers. Furthermore, the types are expected to expose the integer
value in binary representation and support bitwise operators. The standard bitwise operations
(e.g., bitwise-and, bitwise-or, right-shift, left-shift) are inherited and the trait extends
these with introspective queries (e.g., `PrimInt::count_ones()`, `PrimInt::leading_zeros()`),
bitwise combinators (e.g., `PrimInt::rotate_left()`), and endianness converters (e.g.,
`PrimInt::to_be()`).

All `PrimInt` types are expected to be fixed-width binary integers. The width can be queried
via `T::zero().count_zeros()`. The trait currently lacks a way to query the width at
compile-time.

While a default implementation for all builtin primitive integers is provided, the trait is in
no way restricted to these. Other integer types that fulfil the requirements are free to
implement the trait was well.

This trait and many of the method names originate in the unstable `core::num::Int` trait from
the rust standard library. The original trait was never stabilized and thus removed from the
standard library.

**Methods:**

- `count_ones`: Returns the number of ones in the binary representation of `self`.
- `count_zeros`: Returns the number of zeros in the binary representation of `self`.
- `leading_ones`: Returns the number of leading ones in the binary representation
- `leading_zeros`: Returns the number of leading zeros in the binary representation
- `trailing_ones`: Returns the number of trailing ones in the binary representation
- `trailing_zeros`: Returns the number of trailing zeros in the binary representation
- `rotate_left`: Shifts the bits to the left by a specified amount, `n`, wrapping
- `rotate_right`: Shifts the bits to the right by a specified amount, `n`, wrapping
- `signed_shl`: Shifts the bits to the left by a specified amount, `n`, filling
- `signed_shr`: Shifts the bits to the right by a specified amount, `n`, copying
- `unsigned_shl`: Shifts the bits to the left by a specified amount, `n`, filling
- `unsigned_shr`: Shifts the bits to the right by a specified amount, `n`, filling
- `swap_bytes`: Reverses the byte order of the integer.
- `reverse_bits`: Reverses the order of bits in the integer.
- `from_be`: Convert an integer from big endian to the target's endianness.
- `from_le`: Convert an integer from little endian to the target's endianness.
- `to_be`: Convert `self` to big endian from the target's endianness.
- `to_le`: Convert `self` to little endian from the target's endianness.
- `pow`: Raises self to the power of `exp`, using exponentiation by squaring.



