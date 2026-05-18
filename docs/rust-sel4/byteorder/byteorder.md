**byteorder**

# Module: byteorder

## Contents

**Enums**

- [`BigEndian`](#bigendian) - Defines big-endian serialization.
- [`LittleEndian`](#littleendian) - Defines little-endian serialization.

**Traits**

- [`ByteOrder`](#byteorder) - `ByteOrder` describes types that can serialize integers as bytes.

**Type Aliases**

- [`BE`](#be) - A type alias for [`BigEndian`].
- [`LE`](#le) - A type alias for [`LittleEndian`].
- [`NativeEndian`](#nativeendian) - Defines system native-endian serialization.
- [`NetworkEndian`](#networkendian) - Defines network byte order serialization.

---

## byteorder::BE

*Type Alias*: `BigEndian`

A type alias for [`BigEndian`].

[`BigEndian`]: enum.BigEndian.html



## byteorder::BigEndian

*Enum*

Defines big-endian serialization.

Note that this type has no value constructor. It is used purely at the
type level.

# Examples

Write and read `u32` numbers in big endian order:

```rust
use byteorder::{ByteOrder, BigEndian};

let mut buf = [0; 4];
BigEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, BigEndian::read_u32(&buf));
```

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BigEndian) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BigEndian) -> bool`
- **ByteOrder**
  - `fn read_u16(buf: &[u8]) -> u16`
  - `fn read_u32(buf: &[u8]) -> u32`
  - `fn read_u64(buf: &[u8]) -> u64`
  - `fn read_u128(buf: &[u8]) -> u128`
  - `fn read_uint(buf: &[u8], nbytes: usize) -> u64`
  - `fn read_uint128(buf: &[u8], nbytes: usize) -> u128`
  - `fn write_u16(buf: & mut [u8], n: u16)`
  - `fn write_u32(buf: & mut [u8], n: u32)`
  - `fn write_u64(buf: & mut [u8], n: u64)`
  - `fn write_u128(buf: & mut [u8], n: u128)`
  - `fn write_uint(buf: & mut [u8], n: u64, nbytes: usize)`
  - `fn write_uint128(buf: & mut [u8], n: u128, nbytes: usize)`
  - `fn read_u16_into(src: &[u8], dst: & mut [u16])`
  - `fn read_u32_into(src: &[u8], dst: & mut [u32])`
  - `fn read_u64_into(src: &[u8], dst: & mut [u64])`
  - `fn read_u128_into(src: &[u8], dst: & mut [u128])`
  - `fn write_u16_into(src: &[u16], dst: & mut [u8])`
  - `fn write_u32_into(src: &[u32], dst: & mut [u8])`
  - `fn write_u64_into(src: &[u64], dst: & mut [u8])`
  - `fn write_u128_into(src: &[u128], dst: & mut [u8])`
  - `fn from_slice_u16(numbers: & mut [u16])`
  - `fn from_slice_u32(numbers: & mut [u32])`
  - `fn from_slice_u64(numbers: & mut [u64])`
  - `fn from_slice_u128(numbers: & mut [u128])`
  - `fn from_slice_f32(numbers: & mut [f32])`
  - `fn from_slice_f64(numbers: & mut [f64])`
- **Clone**
  - `fn clone(self: &Self) -> BigEndian`
- **Ord**
  - `fn cmp(self: &Self, other: &BigEndian) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> BigEndian`



## byteorder::ByteOrder

*Trait*

`ByteOrder` describes types that can serialize integers as bytes.

Note that `Self` does not appear anywhere in this trait's definition!
Therefore, in order to use it, you'll need to use syntax like
`T::read_u16(&[0, 1])` where `T` implements `ByteOrder`.

This crate provides two types that implement `ByteOrder`: [`BigEndian`]
and [`LittleEndian`].
This trait is sealed and cannot be implemented for callers to avoid
breaking backwards compatibility when adding new derived traits.

# Examples

Write and read `u32` numbers in little endian order:

```rust
use byteorder::{ByteOrder, LittleEndian};

let mut buf = [0; 4];
LittleEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
```

Write and read `i16` numbers in big endian order:

```rust
use byteorder::{ByteOrder, BigEndian};

let mut buf = [0; 2];
BigEndian::write_i16(&mut buf, -5_000);
assert_eq!(-5_000, BigEndian::read_i16(&buf));
```

[`BigEndian`]: enum.BigEndian.html
[`LittleEndian`]: enum.LittleEndian.html

**Methods:**

- `read_u16`: Reads an unsigned 16 bit integer from `buf`.
- `read_u24`: Reads an unsigned 24 bit integer from `buf`, stored in u32.
- `read_u32`: Reads an unsigned 32 bit integer from `buf`.
- `read_u48`: Reads an unsigned 48 bit integer from `buf`, stored in u64.
- `read_u64`: Reads an unsigned 64 bit integer from `buf`.
- `read_u128`: Reads an unsigned 128 bit integer from `buf`.
- `read_uint`: Reads an unsigned n-bytes integer from `buf`.
- `read_uint128`: Reads an unsigned n-bytes integer from `buf`.
- `write_u16`: Writes an unsigned 16 bit integer `n` to `buf`.
- `write_u24`: Writes an unsigned 24 bit integer `n` to `buf`, stored in u32.
- `write_u32`: Writes an unsigned 32 bit integer `n` to `buf`.
- `write_u48`: Writes an unsigned 48 bit integer `n` to `buf`, stored in u64.
- `write_u64`: Writes an unsigned 64 bit integer `n` to `buf`.
- `write_u128`: Writes an unsigned 128 bit integer `n` to `buf`.
- `write_uint`: Writes an unsigned integer `n` to `buf` using only `nbytes`.
- `write_uint128`: Writes an unsigned integer `n` to `buf` using only `nbytes`.
- `read_i16`: Reads a signed 16 bit integer from `buf`.
- `read_i24`: Reads a signed 24 bit integer from `buf`, stored in i32.
- `read_i32`: Reads a signed 32 bit integer from `buf`.
- `read_i48`: Reads a signed 48 bit integer from `buf`, stored in i64.
- `read_i64`: Reads a signed 64 bit integer from `buf`.
- `read_i128`: Reads a signed 128 bit integer from `buf`.
- `read_int`: Reads a signed n-bytes integer from `buf`.
- `read_int128`: Reads a signed n-bytes integer from `buf`.
- `read_f32`: Reads a IEEE754 single-precision (4 bytes) floating point number.
- `read_f64`: Reads a IEEE754 double-precision (8 bytes) floating point number.
- `write_i16`: Writes a signed 16 bit integer `n` to `buf`.
- `write_i24`: Writes a signed 24 bit integer `n` to `buf`, stored in i32.
- `write_i32`: Writes a signed 32 bit integer `n` to `buf`.
- `write_i48`: Writes a signed 48 bit integer `n` to `buf`, stored in i64.
- `write_i64`: Writes a signed 64 bit integer `n` to `buf`.
- `write_i128`: Writes a signed 128 bit integer `n` to `buf`.
- `write_int`: Writes a signed integer `n` to `buf` using only `nbytes`.
- `write_int128`: Writes a signed integer `n` to `buf` using only `nbytes`.
- `write_f32`: Writes a IEEE754 single-precision (4 bytes) floating point number.
- `write_f64`: Writes a IEEE754 double-precision (8 bytes) floating point number.
- `read_u16_into`: Reads unsigned 16 bit integers from `src` into `dst`.
- `read_u32_into`: Reads unsigned 32 bit integers from `src` into `dst`.
- `read_u64_into`: Reads unsigned 64 bit integers from `src` into `dst`.
- `read_u128_into`: Reads unsigned 128 bit integers from `src` into `dst`.
- `read_i16_into`: Reads signed 16 bit integers from `src` to `dst`.
- `read_i32_into`: Reads signed 32 bit integers from `src` into `dst`.
- `read_i64_into`: Reads signed 64 bit integers from `src` into `dst`.
- `read_i128_into`: Reads signed 128 bit integers from `src` into `dst`.
- `read_f32_into`: Reads IEEE754 single-precision (4 bytes) floating point numbers from
- `read_f32_into_unchecked`: **DEPRECATED**.
- `read_f64_into`: Reads IEEE754 single-precision (4 bytes) floating point numbers from
- `read_f64_into_unchecked`: **DEPRECATED**.
- `write_u16_into`: Writes unsigned 16 bit integers from `src` into `dst`.
- `write_u32_into`: Writes unsigned 32 bit integers from `src` into `dst`.
- `write_u64_into`: Writes unsigned 64 bit integers from `src` into `dst`.
- `write_u128_into`: Writes unsigned 128 bit integers from `src` into `dst`.
- `write_i8_into`: Writes signed 8 bit integers from `src` into `dst`.
- `write_i16_into`: Writes signed 16 bit integers from `src` into `dst`.
- `write_i32_into`: Writes signed 32 bit integers from `src` into `dst`.
- `write_i64_into`: Writes signed 64 bit integers from `src` into `dst`.
- `write_i128_into`: Writes signed 128 bit integers from `src` into `dst`.
- `write_f32_into`: Writes IEEE754 single-precision (4 bytes) floating point numbers from
- `write_f64_into`: Writes IEEE754 double-precision (8 bytes) floating point numbers from
- `from_slice_u16`: Converts the given slice of unsigned 16 bit integers to a particular
- `from_slice_u32`: Converts the given slice of unsigned 32 bit integers to a particular
- `from_slice_u64`: Converts the given slice of unsigned 64 bit integers to a particular
- `from_slice_u128`: Converts the given slice of unsigned 128 bit integers to a particular
- `from_slice_i16`: Converts the given slice of signed 16 bit integers to a particular
- `from_slice_i32`: Converts the given slice of signed 32 bit integers to a particular
- `from_slice_i64`: Converts the given slice of signed 64 bit integers to a particular
- `from_slice_i128`: Converts the given slice of signed 128 bit integers to a particular
- `from_slice_f32`: Converts the given slice of IEEE754 single-precision (4 bytes) floating
- `from_slice_f64`: Converts the given slice of IEEE754 double-precision (8 bytes) floating



## byteorder::LE

*Type Alias*: `LittleEndian`

A type alias for [`LittleEndian`].

[`LittleEndian`]: enum.LittleEndian.html



## byteorder::LittleEndian

*Enum*

Defines little-endian serialization.

Note that this type has no value constructor. It is used purely at the
type level.

# Examples

Write and read `u32` numbers in little endian order:

```rust
use byteorder::{ByteOrder, LittleEndian};

let mut buf = [0; 4];
LittleEndian::write_u32(&mut buf, 1_000_000);
assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
```

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ByteOrder**
  - `fn read_u16(buf: &[u8]) -> u16`
  - `fn read_u32(buf: &[u8]) -> u32`
  - `fn read_u64(buf: &[u8]) -> u64`
  - `fn read_u128(buf: &[u8]) -> u128`
  - `fn read_uint(buf: &[u8], nbytes: usize) -> u64`
  - `fn read_uint128(buf: &[u8], nbytes: usize) -> u128`
  - `fn write_u16(buf: & mut [u8], n: u16)`
  - `fn write_u32(buf: & mut [u8], n: u32)`
  - `fn write_u64(buf: & mut [u8], n: u64)`
  - `fn write_u128(buf: & mut [u8], n: u128)`
  - `fn write_uint(buf: & mut [u8], n: u64, nbytes: usize)`
  - `fn write_uint128(buf: & mut [u8], n: u128, nbytes: usize)`
  - `fn read_u16_into(src: &[u8], dst: & mut [u16])`
  - `fn read_u32_into(src: &[u8], dst: & mut [u32])`
  - `fn read_u64_into(src: &[u8], dst: & mut [u64])`
  - `fn read_u128_into(src: &[u8], dst: & mut [u128])`
  - `fn write_u16_into(src: &[u16], dst: & mut [u8])`
  - `fn write_u32_into(src: &[u32], dst: & mut [u8])`
  - `fn write_u64_into(src: &[u64], dst: & mut [u8])`
  - `fn write_u128_into(src: &[u128], dst: & mut [u8])`
  - `fn from_slice_u16(numbers: & mut [u16])`
  - `fn from_slice_u32(numbers: & mut [u32])`
  - `fn from_slice_u64(numbers: & mut [u64])`
  - `fn from_slice_u128(numbers: & mut [u128])`
  - `fn from_slice_f32(numbers: & mut [f32])`
  - `fn from_slice_f64(numbers: & mut [f64])`
- **Default**
  - `fn default() -> LittleEndian`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &LittleEndian) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> LittleEndian`
- **PartialEq**
  - `fn eq(self: &Self, other: &LittleEndian) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &LittleEndian) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## byteorder::NativeEndian

*Type Alias*: `LittleEndian`

Defines system native-endian serialization.

Note that this type has no value constructor. It is used purely at the
type level.

On this platform, this is an alias for [`LittleEndian`].

[`LittleEndian`]: enum.LittleEndian.html



## byteorder::NetworkEndian

*Type Alias*: `BigEndian`

Defines network byte order serialization.

Network byte order is defined by [RFC 1700][1] to be big-endian, and is
referred to in several protocol specifications.  This type is an alias of
[`BigEndian`].

[1]: https://tools.ietf.org/html/rfc1700

Note that this type has no value constructor. It is used purely at the
type level.

# Examples

Write and read `i16` numbers in big endian order:

```rust
use byteorder::{ByteOrder, NetworkEndian, BigEndian};

let mut buf = [0; 2];
BigEndian::write_i16(&mut buf, -5_000);
assert_eq!(-5_000, NetworkEndian::read_i16(&buf));
```

[`BigEndian`]: enum.BigEndian.html



