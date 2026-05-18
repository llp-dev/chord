**object > read > util**

# Module: read::util

## Contents

**Structs**

- [`Bytes`](#bytes) - A newtype for byte slices.
- [`StringTable`](#stringtable) - A table of zero-terminated strings.

---

## object::read::util::Bytes

*Struct*

A newtype for byte slices.

It has these important features:
- no methods that can panic, such as `Index`
- convenience methods for `Pod` types
- a useful `Debug` implementation

**Generic Parameters:**
- 'data

**Tuple Struct**: `(&'data [u8])`

**Methods:**

- `fn len(self: &Self) -> usize` - Return the length of the byte slice.
- `fn is_empty(self: &Self) -> bool` - Return true if the byte slice is empty.
- `fn skip(self: & mut Self, offset: usize) -> Result<(), ()>` - Skip over the given number of bytes at the start of the byte slice.
- `fn read_bytes(self: & mut Self, count: usize) -> Result<Bytes<'data>, ()>` - Return a reference to the given number of bytes at the start of the byte slice.
- `fn read_bytes_at(self: Self, offset: usize, count: usize) -> Result<Bytes<'data>, ()>` - Return a reference to the given number of bytes at the given offset of the byte slice.
- `fn read<T>(self: & mut Self) -> Result<&'data T, ()>` - Return a reference to a `Pod` struct at the start of the byte slice.
- `fn read_at<T>(self: Self, offset: usize) -> Result<&'data T, ()>` - Return a reference to a `Pod` struct at the given offset of the byte slice.
- `fn read_slice<T>(self: & mut Self, count: usize) -> Result<&'data [T], ()>` - Return a reference to a slice of `Pod` structs at the start of the byte slice.
- `fn read_slice_at<T>(self: Self, offset: usize, count: usize) -> Result<&'data [T], ()>` - Return a reference to a slice of `Pod` structs at the given offset of the byte slice.
- `fn read_string(self: & mut Self) -> Result<&'data [u8], ()>` - Read a null terminated string.
- `fn read_string_at(self: Self, offset: usize) -> Result<&'data [u8], ()>` - Read a null terminated string at an offset.
- `fn read_uleb128(self: & mut Self) -> Result<u64, ()>` - Read an unsigned LEB128 number.
- `fn read_sleb128(self: & mut Self) -> Result<i64, ()>` - Read a signed LEB128 number.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Bytes<'data>`
- **Default**
  - `fn default() -> Bytes<'data>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Bytes<'data>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## object::read::util::StringTable

*Struct*

A table of zero-terminated strings.

This is used by most file formats for strings such as section names and symbol names.

**Generic Parameters:**
- 'data
- R

**Methods:**

- `fn new(data: R, start: u64, end: u64) -> Self` - Interpret the given data as a string table.
- `fn get(self: &Self, offset: u32) -> Result<&'data [u8], ()>` - Return the string at the given offset.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> StringTable<'data, R>`
- **Default**
  - `fn default() -> Self`



