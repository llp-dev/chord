# Crate `byteorder`

This crate provides convenience methods for encoding and decoding numbers in
either [big-endian or little-endian order].

The organization of the crate is pretty simple. A trait, [`ByteOrder`](#byteorder), specifies
byte conversion methods for each type of number in Rust (sans numbers that have
a platform dependent size like `usize` and `isize`). Two types, [`BigEndian`](#bigendian)
and [`LittleEndian`](#littleendian) implement these methods. Finally, `ReadBytesExt` and
`WriteBytesExt` provide convenience methods available to all types that
implement [`Read`](#read) and [`Write`](#write).

An alias, [`NetworkEndian`](#networkendian), for [`BigEndian`](#bigendian) is provided to help improve
code clarity.

An additional alias, [`NativeEndian`](#nativeendian), is provided for the endianness of the
local platform. This is convenient when serializing data for use and
conversions are not desired.

# Examples

Read unsigned 16 bit big-endian integers from a [`Read`](#read) type:

```rust
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
// Note that we use type parameters to indicate which kind of byte order
// we want!
assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
```

Write unsigned 16 bit little-endian integers to a [`Write`](#write) type:

```rust
use byteorder::{LittleEndian, WriteBytesExt};

let mut wtr = vec![];
wtr.write_u16::<LittleEndian>(517).unwrap();
wtr.write_u16::<LittleEndian>(768).unwrap();
assert_eq!(wtr, vec![5, 2, 0, 3]);
```

# Optional Features

This crate optionally provides support for 128 bit values (`i128` and `u128`)
when built with the `i128` feature enabled.

This crate can also be used without the standard library.

# Alternatives

Note that as of Rust 1.32, the standard numeric types provide built-in methods
like `to_le_bytes` and `from_le_bytes`, which support some of the same use
cases.











## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Enums](#enums)
  - [`BigEndian`](#bigendian)
  - [`LittleEndian`](#littleendian)
- [Traits](#traits)
  - [`ByteOrder`](#byteorder)
- [Functions](#functions)
  - [`extend_sign`](#extend-sign)
  - [`extend_sign128`](#extend-sign128)
  - [`unextend_sign`](#unextend-sign)
  - [`unextend_sign128`](#unextend-sign128)
  - [`pack_size`](#pack-size)
  - [`pack_size128`](#pack-size128)
- [Type Aliases](#type-aliases)
  - [`BE`](#be)
  - [`LE`](#le)
  - [`NetworkEndian`](#networkendian)
  - [`NativeEndian`](#nativeendian)
- [Macros](#macros)
  - [`read_slice!`](#read-slice)
  - [`write_slice!`](#write-slice)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`BigEndian`](#bigendian) | enum | Defines big-endian serialization. |
| [`LittleEndian`](#littleendian) | enum | Defines little-endian serialization. |
| [`ByteOrder`](#byteorder) | trait | `ByteOrder` describes types that can serialize integers as bytes. |
| [`extend_sign`](#extend-sign) | fn |  |
| [`extend_sign128`](#extend-sign128) | fn |  |
| [`unextend_sign`](#unextend-sign) | fn |  |
| [`unextend_sign128`](#unextend-sign128) | fn |  |
| [`pack_size`](#pack-size) | fn |  |
| [`pack_size128`](#pack-size128) | fn |  |
| [`BE`](#be) | type | A type alias for [`BigEndian`]. |
| [`LE`](#le) | type | A type alias for [`LittleEndian`]. |
| [`NetworkEndian`](#networkendian) | type | Defines network byte order serialization. |
| [`NativeEndian`](#nativeendian) | type | Defines system native-endian serialization. |
| [`read_slice!`](#read-slice) | macro | Copies a &[u8] $src into a &mut [$ty] $dst for the endianness given by $from_bytes (must be either from_be_bytes or from_le_bytes). |
| [`write_slice!`](#write-slice) | macro | Copies a &[$ty] $src into a &mut [u8] $dst for the endianness given by $from_bytes (must be either from_be_bytes or from_le_bytes). |

## Modules

- [`private`](private/index.md)

## Enums

### `BigEndian`

```rust
enum BigEndian {
}
```

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

#### Trait Implementations

##### `impl ByteOrder for BigEndian`

- <span id="bigendian-byteorder-read-u16"></span>`fn read_u16(buf: &[u8]) -> u16`

- <span id="bigendian-byteorder-read-u32"></span>`fn read_u32(buf: &[u8]) -> u32`

- <span id="bigendian-byteorder-read-u64"></span>`fn read_u64(buf: &[u8]) -> u64`

- <span id="bigendian-byteorder-read-u128"></span>`fn read_u128(buf: &[u8]) -> u128`

- <span id="bigendian-byteorder-read-uint"></span>`fn read_uint(buf: &[u8], nbytes: usize) -> u64`

- <span id="bigendian-byteorder-read-uint128"></span>`fn read_uint128(buf: &[u8], nbytes: usize) -> u128`

- <span id="bigendian-byteorder-write-u16"></span>`fn write_u16(buf: &mut [u8], n: u16)`

- <span id="bigendian-byteorder-write-u32"></span>`fn write_u32(buf: &mut [u8], n: u32)`

- <span id="bigendian-byteorder-write-u64"></span>`fn write_u64(buf: &mut [u8], n: u64)`

- <span id="bigendian-byteorder-write-u128"></span>`fn write_u128(buf: &mut [u8], n: u128)`

- <span id="bigendian-byteorder-write-uint"></span>`fn write_uint(buf: &mut [u8], n: u64, nbytes: usize)`

- <span id="bigendian-byteorder-write-uint128"></span>`fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize)`

- <span id="bigendian-byteorder-read-u16-into"></span>`fn read_u16_into(src: &[u8], dst: &mut [u16])`

- <span id="bigendian-byteorder-read-u32-into"></span>`fn read_u32_into(src: &[u8], dst: &mut [u32])`

- <span id="bigendian-byteorder-read-u64-into"></span>`fn read_u64_into(src: &[u8], dst: &mut [u64])`

- <span id="bigendian-byteorder-read-u128-into"></span>`fn read_u128_into(src: &[u8], dst: &mut [u128])`

- <span id="bigendian-byteorder-write-u16-into"></span>`fn write_u16_into(src: &[u16], dst: &mut [u8])`

- <span id="bigendian-byteorder-write-u32-into"></span>`fn write_u32_into(src: &[u32], dst: &mut [u8])`

- <span id="bigendian-byteorder-write-u64-into"></span>`fn write_u64_into(src: &[u64], dst: &mut [u8])`

- <span id="bigendian-byteorder-write-u128-into"></span>`fn write_u128_into(src: &[u128], dst: &mut [u8])`

- <span id="bigendian-byteorder-from-slice-u16"></span>`fn from_slice_u16(numbers: &mut [u16])`

- <span id="bigendian-byteorder-from-slice-u32"></span>`fn from_slice_u32(numbers: &mut [u32])`

- <span id="bigendian-byteorder-from-slice-u64"></span>`fn from_slice_u64(numbers: &mut [u64])`

- <span id="bigendian-byteorder-from-slice-u128"></span>`fn from_slice_u128(numbers: &mut [u128])`

- <span id="bigendian-byteorder-from-slice-f32"></span>`fn from_slice_f32(numbers: &mut [f32])`

- <span id="bigendian-byteorder-from-slice-f64"></span>`fn from_slice_f64(numbers: &mut [f64])`

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigEndian`

- <span id="bigendian-default"></span>`fn default() -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for BigEndian`

- <span id="bigendian-ord-cmp"></span>`fn cmp(&self, other: &BigEndian) -> cmp::Ordering` — [`BigEndian`](#bigendian)

##### `impl PartialEq for BigEndian`

- <span id="bigendian-partialeq-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](#bigendian)

##### `impl PartialOrd for BigEndian`

- <span id="bigendian-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BigEndian) -> option::Option<cmp::Ordering>` — [`BigEndian`](#bigendian)

##### `impl Sealed for super::BigEndian`

##### `impl StructuralPartialEq for BigEndian`

### `LittleEndian`

```rust
enum LittleEndian {
}
```

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

#### Trait Implementations

##### `impl ByteOrder for LittleEndian`

- <span id="littleendian-byteorder-read-u16"></span>`fn read_u16(buf: &[u8]) -> u16`

- <span id="littleendian-byteorder-read-u32"></span>`fn read_u32(buf: &[u8]) -> u32`

- <span id="littleendian-byteorder-read-u64"></span>`fn read_u64(buf: &[u8]) -> u64`

- <span id="littleendian-byteorder-read-u128"></span>`fn read_u128(buf: &[u8]) -> u128`

- <span id="littleendian-byteorder-read-uint"></span>`fn read_uint(buf: &[u8], nbytes: usize) -> u64`

- <span id="littleendian-byteorder-read-uint128"></span>`fn read_uint128(buf: &[u8], nbytes: usize) -> u128`

- <span id="littleendian-byteorder-write-u16"></span>`fn write_u16(buf: &mut [u8], n: u16)`

- <span id="littleendian-byteorder-write-u32"></span>`fn write_u32(buf: &mut [u8], n: u32)`

- <span id="littleendian-byteorder-write-u64"></span>`fn write_u64(buf: &mut [u8], n: u64)`

- <span id="littleendian-byteorder-write-u128"></span>`fn write_u128(buf: &mut [u8], n: u128)`

- <span id="littleendian-byteorder-write-uint"></span>`fn write_uint(buf: &mut [u8], n: u64, nbytes: usize)`

- <span id="littleendian-byteorder-write-uint128"></span>`fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize)`

- <span id="littleendian-byteorder-read-u16-into"></span>`fn read_u16_into(src: &[u8], dst: &mut [u16])`

- <span id="littleendian-byteorder-read-u32-into"></span>`fn read_u32_into(src: &[u8], dst: &mut [u32])`

- <span id="littleendian-byteorder-read-u64-into"></span>`fn read_u64_into(src: &[u8], dst: &mut [u64])`

- <span id="littleendian-byteorder-read-u128-into"></span>`fn read_u128_into(src: &[u8], dst: &mut [u128])`

- <span id="littleendian-byteorder-write-u16-into"></span>`fn write_u16_into(src: &[u16], dst: &mut [u8])`

- <span id="littleendian-byteorder-write-u32-into"></span>`fn write_u32_into(src: &[u32], dst: &mut [u8])`

- <span id="littleendian-byteorder-write-u64-into"></span>`fn write_u64_into(src: &[u64], dst: &mut [u8])`

- <span id="littleendian-byteorder-write-u128-into"></span>`fn write_u128_into(src: &[u128], dst: &mut [u8])`

- <span id="littleendian-byteorder-from-slice-u16"></span>`fn from_slice_u16(numbers: &mut [u16])`

- <span id="littleendian-byteorder-from-slice-u32"></span>`fn from_slice_u32(numbers: &mut [u32])`

- <span id="littleendian-byteorder-from-slice-u64"></span>`fn from_slice_u64(numbers: &mut [u64])`

- <span id="littleendian-byteorder-from-slice-u128"></span>`fn from_slice_u128(numbers: &mut [u128])`

- <span id="littleendian-byteorder-from-slice-f32"></span>`fn from_slice_f32(numbers: &mut [f32])`

- <span id="littleendian-byteorder-from-slice-f64"></span>`fn from_slice_f64(numbers: &mut [f64])`

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LittleEndian`

- <span id="littleendian-default"></span>`fn default() -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LittleEndian`

- <span id="littleendian-ord-cmp"></span>`fn cmp(&self, other: &LittleEndian) -> cmp::Ordering` — [`LittleEndian`](#littleendian)

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-partialeq-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](#littleendian)

##### `impl PartialOrd for LittleEndian`

- <span id="littleendian-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LittleEndian) -> option::Option<cmp::Ordering>` — [`LittleEndian`](#littleendian)

##### `impl Sealed for super::LittleEndian`

##### `impl StructuralPartialEq for LittleEndian`

## Traits

### `ByteOrder`

```rust
trait ByteOrder: Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + private::Sealed { ... }
```

`ByteOrder` describes types that can serialize integers as bytes.

Note that `Self` does not appear anywhere in this trait's definition!
Therefore, in order to use it, you'll need to use syntax like
`T::read_u16(&[0, 1])` where `T` implements `ByteOrder`.

This crate provides two types that implement `ByteOrder`: [`BigEndian`](#bigendian)
and [`LittleEndian`](#littleendian).
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



#### Required Methods

- `fn read_u16(buf: &[u8]) -> u16`

  Reads an unsigned 16 bit integer from `buf`.

- `fn read_u32(buf: &[u8]) -> u32`

  Reads an unsigned 32 bit integer from `buf`.

- `fn read_u64(buf: &[u8]) -> u64`

  Reads an unsigned 64 bit integer from `buf`.

- `fn read_u128(buf: &[u8]) -> u128`

  Reads an unsigned 128 bit integer from `buf`.

- `fn read_uint(buf: &[u8], nbytes: usize) -> u64`

  Reads an unsigned n-bytes integer from `buf`.

- `fn read_uint128(buf: &[u8], nbytes: usize) -> u128`

  Reads an unsigned n-bytes integer from `buf`.

- `fn write_u16(buf: &mut [u8], n: u16)`

  Writes an unsigned 16 bit integer `n` to `buf`.

- `fn write_u32(buf: &mut [u8], n: u32)`

  Writes an unsigned 32 bit integer `n` to `buf`.

- `fn write_u64(buf: &mut [u8], n: u64)`

  Writes an unsigned 64 bit integer `n` to `buf`.

- `fn write_u128(buf: &mut [u8], n: u128)`

  Writes an unsigned 128 bit integer `n` to `buf`.

- `fn write_uint(buf: &mut [u8], n: u64, nbytes: usize)`

  Writes an unsigned integer `n` to `buf` using only `nbytes`.

- `fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize)`

  Writes an unsigned integer `n` to `buf` using only `nbytes`.

- `fn read_u16_into(src: &[u8], dst: &mut [u16])`

  Reads unsigned 16 bit integers from `src` into `dst`.

- `fn read_u32_into(src: &[u8], dst: &mut [u32])`

  Reads unsigned 32 bit integers from `src` into `dst`.

- `fn read_u64_into(src: &[u8], dst: &mut [u64])`

  Reads unsigned 64 bit integers from `src` into `dst`.

- `fn read_u128_into(src: &[u8], dst: &mut [u128])`

  Reads unsigned 128 bit integers from `src` into `dst`.

- `fn write_u16_into(src: &[u16], dst: &mut [u8])`

  Writes unsigned 16 bit integers from `src` into `dst`.

- `fn write_u32_into(src: &[u32], dst: &mut [u8])`

  Writes unsigned 32 bit integers from `src` into `dst`.

- `fn write_u64_into(src: &[u64], dst: &mut [u8])`

  Writes unsigned 64 bit integers from `src` into `dst`.

- `fn write_u128_into(src: &[u128], dst: &mut [u8])`

  Writes unsigned 128 bit integers from `src` into `dst`.

- `fn from_slice_u16(numbers: &mut [u16])`

  Converts the given slice of unsigned 16 bit integers to a particular

- `fn from_slice_u32(numbers: &mut [u32])`

  Converts the given slice of unsigned 32 bit integers to a particular

- `fn from_slice_u64(numbers: &mut [u64])`

  Converts the given slice of unsigned 64 bit integers to a particular

- `fn from_slice_u128(numbers: &mut [u128])`

  Converts the given slice of unsigned 128 bit integers to a particular

- `fn from_slice_f32(numbers: &mut [f32])`

  Converts the given slice of IEEE754 single-precision (4 bytes) floating

- `fn from_slice_f64(numbers: &mut [f64])`

  Converts the given slice of IEEE754 double-precision (8 bytes) floating

#### Provided Methods

- `fn read_u24(buf: &[u8]) -> u32`

  Reads an unsigned 24 bit integer from `buf`, stored in u32.

- `fn read_u48(buf: &[u8]) -> u64`

  Reads an unsigned 48 bit integer from `buf`, stored in u64.

- `fn write_u24(buf: &mut [u8], n: u32)`

  Writes an unsigned 24 bit integer `n` to `buf`, stored in u32.

- `fn write_u48(buf: &mut [u8], n: u64)`

  Writes an unsigned 48 bit integer `n` to `buf`, stored in u64.

- `fn read_i16(buf: &[u8]) -> i16`

  Reads a signed 16 bit integer from `buf`.

- `fn read_i24(buf: &[u8]) -> i32`

  Reads a signed 24 bit integer from `buf`, stored in i32.

- `fn read_i32(buf: &[u8]) -> i32`

  Reads a signed 32 bit integer from `buf`.

- `fn read_i48(buf: &[u8]) -> i64`

  Reads a signed 48 bit integer from `buf`, stored in i64.

- `fn read_i64(buf: &[u8]) -> i64`

  Reads a signed 64 bit integer from `buf`.

- `fn read_i128(buf: &[u8]) -> i128`

  Reads a signed 128 bit integer from `buf`.

- `fn read_int(buf: &[u8], nbytes: usize) -> i64`

  Reads a signed n-bytes integer from `buf`.

- `fn read_int128(buf: &[u8], nbytes: usize) -> i128`

  Reads a signed n-bytes integer from `buf`.

- `fn read_f32(buf: &[u8]) -> f32`

  Reads a IEEE754 single-precision (4 bytes) floating point number.

- `fn read_f64(buf: &[u8]) -> f64`

  Reads a IEEE754 double-precision (8 bytes) floating point number.

- `fn write_i16(buf: &mut [u8], n: i16)`

  Writes a signed 16 bit integer `n` to `buf`.

- `fn write_i24(buf: &mut [u8], n: i32)`

  Writes a signed 24 bit integer `n` to `buf`, stored in i32.

- `fn write_i32(buf: &mut [u8], n: i32)`

  Writes a signed 32 bit integer `n` to `buf`.

- `fn write_i48(buf: &mut [u8], n: i64)`

  Writes a signed 48 bit integer `n` to `buf`, stored in i64.

- `fn write_i64(buf: &mut [u8], n: i64)`

  Writes a signed 64 bit integer `n` to `buf`.

- `fn write_i128(buf: &mut [u8], n: i128)`

  Writes a signed 128 bit integer `n` to `buf`.

- `fn write_int(buf: &mut [u8], n: i64, nbytes: usize)`

  Writes a signed integer `n` to `buf` using only `nbytes`.

- `fn write_int128(buf: &mut [u8], n: i128, nbytes: usize)`

  Writes a signed integer `n` to `buf` using only `nbytes`.

- `fn write_f32(buf: &mut [u8], n: f32)`

  Writes a IEEE754 single-precision (4 bytes) floating point number.

- `fn write_f64(buf: &mut [u8], n: f64)`

  Writes a IEEE754 double-precision (8 bytes) floating point number.

- `fn read_i16_into(src: &[u8], dst: &mut [i16])`

  Reads signed 16 bit integers from `src` to `dst`.

- `fn read_i32_into(src: &[u8], dst: &mut [i32])`

  Reads signed 32 bit integers from `src` into `dst`.

- `fn read_i64_into(src: &[u8], dst: &mut [i64])`

  Reads signed 64 bit integers from `src` into `dst`.

- `fn read_i128_into(src: &[u8], dst: &mut [i128])`

  Reads signed 128 bit integers from `src` into `dst`.

- `fn read_f32_into(src: &[u8], dst: &mut [f32])`

  Reads IEEE754 single-precision (4 bytes) floating point numbers from

- `fn read_f32_into_unchecked(src: &[u8], dst: &mut [f32])`

  **DEPRECATED**.

- `fn read_f64_into(src: &[u8], dst: &mut [f64])`

  Reads IEEE754 single-precision (4 bytes) floating point numbers from

- `fn read_f64_into_unchecked(src: &[u8], dst: &mut [f64])`

  **DEPRECATED**.

- `fn write_i8_into(src: &[i8], dst: &mut [u8])`

  Writes signed 8 bit integers from `src` into `dst`.

- `fn write_i16_into(src: &[i16], dst: &mut [u8])`

  Writes signed 16 bit integers from `src` into `dst`.

- `fn write_i32_into(src: &[i32], dst: &mut [u8])`

  Writes signed 32 bit integers from `src` into `dst`.

- `fn write_i64_into(src: &[i64], dst: &mut [u8])`

  Writes signed 64 bit integers from `src` into `dst`.

- `fn write_i128_into(src: &[i128], dst: &mut [u8])`

  Writes signed 128 bit integers from `src` into `dst`.

- `fn write_f32_into(src: &[f32], dst: &mut [u8])`

  Writes IEEE754 single-precision (4 bytes) floating point numbers from

- `fn write_f64_into(src: &[f64], dst: &mut [u8])`

  Writes IEEE754 double-precision (8 bytes) floating point numbers from

- `fn from_slice_i16(src: &mut [i16])`

  Converts the given slice of signed 16 bit integers to a particular

- `fn from_slice_i32(src: &mut [i32])`

  Converts the given slice of signed 32 bit integers to a particular

- `fn from_slice_i64(src: &mut [i64])`

  Converts the given slice of signed 64 bit integers to a particular

- `fn from_slice_i128(src: &mut [i128])`

  Converts the given slice of signed 128 bit integers to a particular

#### Implementors

- [`BigEndian`](#bigendian)
- [`LittleEndian`](#littleendian)

## Functions

### `extend_sign`

```rust
fn extend_sign(val: u64, nbytes: usize) -> i64
```

### `extend_sign128`

```rust
fn extend_sign128(val: u128, nbytes: usize) -> i128
```

### `unextend_sign`

```rust
fn unextend_sign(val: i64, nbytes: usize) -> u64
```

### `unextend_sign128`

```rust
fn unextend_sign128(val: i128, nbytes: usize) -> u128
```

### `pack_size`

```rust
fn pack_size(n: u64) -> usize
```

### `pack_size128`

```rust
fn pack_size128(n: u128) -> usize
```

## Type Aliases

### `BE`

```rust
type BE = BigEndian;
```

A type alias for [`BigEndian`](#bigendian).


### `LE`

```rust
type LE = LittleEndian;
```

A type alias for [`LittleEndian`](#littleendian).


### `NetworkEndian`

```rust
type NetworkEndian = BigEndian;
```

Defines network byte order serialization.

Network byte order is defined by [RFC 1700][1] to be big-endian, and is
referred to in several protocol specifications.  This type is an alias of
[`BigEndian`](#bigendian).

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


### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

Defines system native-endian serialization.

Note that this type has no value constructor. It is used purely at the
type level.

On this platform, this is an alias for [`LittleEndian`](#littleendian).


## Macros

### `read_slice!`

Copies a &[u8] $src into a &mut [$ty] $dst for the endianness given by
$from_bytes (must be either from_be_bytes or from_le_bytes).

Panics if $src.len() != $dst.len() * size_of::<$ty>().

### `write_slice!`

Copies a &[$ty] $src into a &mut [u8] $dst for the endianness given by
$from_bytes (must be either from_be_bytes or from_le_bytes).

Panics if $src.len() * size_of::<$ty>() != $dst.len().

