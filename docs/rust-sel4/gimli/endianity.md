**gimli > endianity**

# Module: endianity

## Contents

**Structs**

- [`BigEndian`](#bigendian) - Big endian byte order.
- [`LittleEndian`](#littleendian) - Little endian byte order.

**Enums**

- [`RunTimeEndian`](#runtimeendian) - Byte order that is selectable at runtime.

**Traits**

- [`Endianity`](#endianity) - A trait describing the endianity of some buffer.

**Type Aliases**

- [`NativeEndian`](#nativeendian) - The native endianity for the target platform.

---

## gimli::endianity::BigEndian

*Struct*

Big endian byte order.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Endianity**
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



## gimli::endianity::Endianity

*Trait*

A trait describing the endianity of some buffer.

**Methods:**

- `is_big_endian`: Return true for big endian byte order.
- `is_little_endian`: Return true for little endian byte order.
- `read_u16`: Reads an unsigned 16 bit integer from `buf`.
- `read_u32`: Reads an unsigned 32 bit integer from `buf`.
- `read_u64`: Reads an unsigned 64 bit integer from `buf`.
- `read_uint`: Read an unsigned n-bytes integer u64.
- `read_i16`: Reads a signed 16 bit integer from `buf`.
- `read_i32`: Reads a signed 32 bit integer from `buf`.
- `read_i64`: Reads a signed 64 bit integer from `buf`.
- `read_f32`: Reads a 32 bit floating point number from `buf`.
- `read_f64`: Reads a 32 bit floating point number from `buf`.
- `write_u16`: Writes an unsigned 16 bit integer `n` to `buf`.
- `write_u32`: Writes an unsigned 32 bit integer `n` to `buf`.
- `write_u64`: Writes an unsigned 64 bit integer `n` to `buf`.



## gimli::endianity::LittleEndian

*Struct*

Little endian byte order.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Endianity**
  - `fn is_big_endian(self: Self) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &LittleEndian) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> LittleEndian`
- **Default**
  - `fn default() -> LittleEndian`



## gimli::endianity::NativeEndian

*Type Alias*: `LittleEndian`

The native endianity for the target platform.



## gimli::endianity::RunTimeEndian

*Enum*

Byte order that is selectable at runtime.

**Variants:**
- `Little` - Little endian byte order.
- `Big` - Big endian byte order.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RunTimeEndian) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Endianity**
  - `fn is_big_endian(self: Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RunTimeEndian`
- **Default**
  - `fn default() -> RunTimeEndian`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



