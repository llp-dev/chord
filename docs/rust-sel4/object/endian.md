**object > endian**

# Module: endian

## Contents

**Structs**

- [`BigEndian`](#bigendian) - Compile-time big endian byte order.
- [`I16Bytes`](#i16bytes) - An unaligned `i16` value with an externally specified endianness of type `E`.
- [`I32Bytes`](#i32bytes) - An unaligned `i32` value with an externally specified endianness of type `E`.
- [`I64Bytes`](#i64bytes) - An unaligned `i64` value with an externally specified endianness of type `E`.
- [`LittleEndian`](#littleendian) - Compile-time little endian byte order.
- [`U16Bytes`](#u16bytes) - An unaligned `u16` value with an externally specified endianness of type `E`.
- [`U32Bytes`](#u32bytes) - An unaligned `u32` value with an externally specified endianness of type `E`.
- [`U64Bytes`](#u64bytes) - An unaligned `u64` value with an externally specified endianness of type `E`.

**Enums**

- [`Endianness`](#endianness) - An endianness that is selectable at run-time.

**Traits**

- [`Endian`](#endian) - A trait for using an endianness specification.

**Type Aliases**

- [`I16`](#i16) - An `i16` value with an externally specified endianness of type `E`.
- [`I32`](#i32) - An `i32` value with an externally specified endianness of type `E`.
- [`I64`](#i64) - An `i64` value with an externally specified endianness of type `E`.
- [`NativeEndian`](#nativeendian) - The native endianness for the target platform.
- [`U16`](#u16) - A `u16` value with an externally specified endianness of type `E`.
- [`U32`](#u32) - A `u32` value with an externally specified endianness of type `E`.
- [`U64`](#u64) - A `u64` value with an externally specified endianness of type `E`.

---

## object::endian::BigEndian

*Struct*

Compile-time big endian byte order.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Endian**
  - `fn from_big_endian(big_endian: bool) -> Option<Self>`
  - `fn is_big_endian(self: Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BigEndian) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> BigEndian`
- **Default**
  - `fn default() -> BigEndian`



## object::endian::Endian

*Trait*

A trait for using an endianness specification.

Provides methods for converting between the specified endianness and
the native endianness of the target machine.

This trait does not require that the endianness is known at compile time.

**Methods:**

- `from_big_endian`: Construct a specification for the endianness of some values.
- `from_little_endian`: Construct a specification for the endianness of some values.
- `is_big_endian`: Return true for big endian byte order.
- `is_little_endian`: Return true for little endian byte order.
- `read_u16`: Converts an unsigned 16 bit integer to native endian.
- `read_u32`: Converts an unsigned 32 bit integer to native endian.
- `read_u64`: Converts an unsigned 64 bit integer to native endian.
- `read_i16`: Converts a signed 16 bit integer to native endian.
- `read_i32`: Converts a signed 32 bit integer to native endian.
- `read_i64`: Converts a signed 64 bit integer to native endian.
- `read_u16_bytes`: Converts an unaligned unsigned 16 bit integer to native endian.
- `read_u32_bytes`: Converts an unaligned unsigned 32 bit integer to native endian.
- `read_u64_bytes`: Converts an unaligned unsigned 64 bit integer to native endian.
- `read_i16_bytes`: Converts an unaligned signed 16 bit integer to native endian.
- `read_i32_bytes`: Converts an unaligned signed 32 bit integer to native endian.
- `read_i64_bytes`: Converts an unaligned signed 64 bit integer to native endian.
- `write_u16`: Converts an unsigned 16 bit integer from native endian.
- `write_u32`: Converts an unsigned 32 bit integer from native endian.
- `write_u64`: Converts an unsigned 64 bit integer from native endian.
- `write_i16`: Converts a signed 16 bit integer from native endian.
- `write_i32`: Converts a signed 32 bit integer from native endian.
- `write_i64`: Converts a signed 64 bit integer from native endian.
- `write_u16_bytes`: Converts an unaligned unsigned 16 bit integer from native endian.
- `write_u32_bytes`: Converts an unaligned unsigned 32 bit integer from native endian.
- `write_u64_bytes`: Converts an unaligned unsigned 64 bit integer from native endian.
- `write_i16_bytes`: Converts an unaligned signed 16 bit integer from native endian.
- `write_i32_bytes`: Converts an unaligned signed 32 bit integer from native endian.
- `write_i64_bytes`: Converts an unaligned signed 64 bit integer from native endian.



## object::endian::Endianness

*Enum*

An endianness that is selectable at run-time.

**Variants:**
- `Little` - Little endian byte order.
- `Big` - Big endian byte order.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Endianness) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Endianness`
- **Default**
  - `fn default() -> Endianness`
- **Endian**
  - `fn from_big_endian(big_endian: bool) -> Option<Self>`
  - `fn is_big_endian(self: Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::endian::I16

*Type Alias*: `I16Bytes<E>`

An `i16` value with an externally specified endianness of type `E`.



## object::endian::I16Bytes

*Struct*

An unaligned `i16` value with an externally specified endianness of type `E`.

**Generic Parameters:**
- E

**Tuple Struct**: `()`

**Methods:**

- `fn from_bytes(n: [u8; 2]) -> Self` - Construct a new value given bytes that already have the required endianness.
- `fn new(e: E, n: i16) -> Self` - Construct a new value given a native endian value.
- `fn get(self: Self, e: E) -> i16` - Return the value as a native endian value.
- `fn set(self: & mut Self, e: E, n: i16)` - Set the value given a native endian value.

**Traits:** Copy, Eq, Pod

**Trait Implementations:**

- **Default**
  - `fn default() -> I16Bytes<E>`
- **Clone**
  - `fn clone(self: &Self) -> I16Bytes<E>`
- **PartialEq**
  - `fn eq(self: &Self, other: &I16Bytes<E>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &I16Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &I16Bytes<E>) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::endian::I32

*Type Alias*: `I32Bytes<E>`

An `i32` value with an externally specified endianness of type `E`.



## object::endian::I32Bytes

*Struct*

An unaligned `i32` value with an externally specified endianness of type `E`.

**Generic Parameters:**
- E

**Tuple Struct**: `()`

**Methods:**

- `fn from_bytes(n: [u8; 4]) -> Self` - Construct a new value given bytes that already have the required endianness.
- `fn new(e: E, n: i32) -> Self` - Construct a new value given a native endian value.
- `fn get(self: Self, e: E) -> i32` - Return the value as a native endian value.
- `fn set(self: & mut Self, e: E, n: i32)` - Set the value given a native endian value.

**Traits:** Copy, Eq, Pod

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &I32Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &I32Bytes<E>) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> I32Bytes<E>`
- **Clone**
  - `fn clone(self: &Self) -> I32Bytes<E>`
- **PartialEq**
  - `fn eq(self: &Self, other: &I32Bytes<E>) -> bool`



## object::endian::I64

*Type Alias*: `I64Bytes<E>`

An `i64` value with an externally specified endianness of type `E`.



## object::endian::I64Bytes

*Struct*

An unaligned `i64` value with an externally specified endianness of type `E`.

**Generic Parameters:**
- E

**Tuple Struct**: `()`

**Methods:**

- `fn from_bytes(n: [u8; 8]) -> Self` - Construct a new value given bytes that already have the required endianness.
- `fn new(e: E, n: i64) -> Self` - Construct a new value given a native endian value.
- `fn get(self: Self, e: E) -> i64` - Return the value as a native endian value.
- `fn set(self: & mut Self, e: E, n: i64)` - Set the value given a native endian value.

**Traits:** Copy, Eq, Pod

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &I64Bytes<E>) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> I64Bytes<E>`
- **Clone**
  - `fn clone(self: &Self) -> I64Bytes<E>`
- **PartialEq**
  - `fn eq(self: &Self, other: &I64Bytes<E>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &I64Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::endian::LittleEndian

*Struct*

Compile-time little endian byte order.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &LittleEndian) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> LittleEndian`
- **Default**
  - `fn default() -> LittleEndian`
- **Endian**
  - `fn from_big_endian(big_endian: bool) -> Option<Self>`
  - `fn is_big_endian(self: Self) -> bool`



## object::endian::NativeEndian

*Type Alias*: `LittleEndian`

The native endianness for the target platform.



## object::endian::U16

*Type Alias*: `U16Bytes<E>`

A `u16` value with an externally specified endianness of type `E`.



## object::endian::U16Bytes

*Struct*

An unaligned `u16` value with an externally specified endianness of type `E`.

**Generic Parameters:**
- E

**Tuple Struct**: `()`

**Methods:**

- `fn from_bytes(n: [u8; 2]) -> Self` - Construct a new value given bytes that already have the required endianness.
- `fn new(e: E, n: u16) -> Self` - Construct a new value given a native endian value.
- `fn get(self: Self, e: E) -> u16` - Return the value as a native endian value.
- `fn set(self: & mut Self, e: E, n: u16)` - Set the value given a native endian value.

**Traits:** Pod, Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> U16Bytes<E>`
- **Clone**
  - `fn clone(self: &Self) -> U16Bytes<E>`
- **PartialEq**
  - `fn eq(self: &Self, other: &U16Bytes<E>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &U16Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &U16Bytes<E>) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::endian::U32

*Type Alias*: `U32Bytes<E>`

A `u32` value with an externally specified endianness of type `E`.



## object::endian::U32Bytes

*Struct*

An unaligned `u32` value with an externally specified endianness of type `E`.

**Generic Parameters:**
- E

**Tuple Struct**: `()`

**Methods:**

- `fn from_bytes(n: [u8; 4]) -> Self` - Construct a new value given bytes that already have the required endianness.
- `fn new(e: E, n: u32) -> Self` - Construct a new value given a native endian value.
- `fn get(self: Self, e: E) -> u32` - Return the value as a native endian value.
- `fn set(self: & mut Self, e: E, n: u32)` - Set the value given a native endian value.

**Traits:** Copy, Eq, Pod

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> U32Bytes<E>`
- **Clone**
  - `fn clone(self: &Self) -> U32Bytes<E>`
- **PartialEq**
  - `fn eq(self: &Self, other: &U32Bytes<E>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &U32Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &U32Bytes<E>) -> $crate::cmp::Ordering`



## object::endian::U64

*Type Alias*: `U64Bytes<E>`

A `u64` value with an externally specified endianness of type `E`.



## object::endian::U64Bytes

*Struct*

An unaligned `u64` value with an externally specified endianness of type `E`.

**Generic Parameters:**
- E

**Tuple Struct**: `()`

**Methods:**

- `fn from_bytes(n: [u8; 8]) -> Self` - Construct a new value given bytes that already have the required endianness.
- `fn new(e: E, n: u64) -> Self` - Construct a new value given a native endian value.
- `fn get(self: Self, e: E) -> u64` - Return the value as a native endian value.
- `fn set(self: & mut Self, e: E, n: u64)` - Set the value given a native endian value.

**Traits:** Copy, Eq, Pod

**Trait Implementations:**

- **Default**
  - `fn default() -> U64Bytes<E>`
- **Clone**
  - `fn clone(self: &Self) -> U64Bytes<E>`
- **PartialEq**
  - `fn eq(self: &Self, other: &U64Bytes<E>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &U64Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &U64Bytes<E>) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



