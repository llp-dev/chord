**gimli > read > reader**

# Module: read::reader

## Contents

**Structs**

- [`ReaderOffsetId`](#readeroffsetid) - An identifier for an offset within a section reader.

**Traits**

- [`Reader`](#reader) - A trait for reading the data from a DWARF section.
- [`ReaderOffset`](#readeroffset) - A trait for offsets with a DWARF section.

---

## gimli::read::reader::Reader

*Trait*

A trait for reading the data from a DWARF section.

All read operations advance the section offset of the reader
unless specified otherwise.

## Choosing a `Reader` Implementation

`gimli` comes with a few different `Reader` implementations and lets you
choose the one that is right for your use case. A `Reader` is essentially a
view into the raw bytes that make up some DWARF, but this view might borrow
the underlying data or use reference counting ownership, and it might be
thread safe or not.

| Implementation    | Ownership         | Thread Safe | Notes |
|:------------------|:------------------|:------------|:------|
| [`EndianSlice`](./struct.EndianSlice.html)        | Borrowed          | Yes         | Fastest, but requires that all of your code work with borrows. |
| [`EndianRcSlice`](./struct.EndianRcSlice.html)    | Reference counted | No          | Shared ownership via reference counting, which alleviates the borrow restrictions of `EndianSlice` but imposes reference counting increments and decrements. Cannot be sent across threads, because the reference count is not atomic. |
| [`EndianArcSlice`](./struct.EndianArcSlice.html)  | Reference counted | Yes         | The same as `EndianRcSlice`, but uses atomic reference counting, and therefore reference counting operations are slower but `EndianArcSlice`s may be sent across threads. |
| [`EndianReader<T>`](./struct.EndianReader.html)   | Same as `T`       | Same as `T` | Escape hatch for easily defining your own type of `Reader`. |

**Methods:**

- `Endian`: The endianity of bytes that are read.
- `Offset`: The type used for offsets and lengths.
- `endian`: Return the endianity of bytes that are read.
- `len`: Return the number of bytes remaining.
- `empty`: Set the number of bytes remaining to zero.
- `truncate`: Set the number of bytes remaining to the specified length.
- `offset_from`: Return the offset of this reader's data relative to the start of
- `offset_id`: Return an identifier for the current reader offset.
- `lookup_offset_id`: Return the offset corresponding to the given `id` if
- `find`: Find the index of the first occurrence of the given byte.
- `skip`: Discard the specified number of bytes.
- `split`: Split a reader in two.
- `cannot_implement`: This trait cannot be implemented if "read" feature is not enabled.
- `read_slice`: Read exactly `buf.len()` bytes into `buf`.
- `read_u8_array`: Read a u8 array.
- `is_empty`: Return true if the number of bytes remaining is zero.
- `read_u8`: Read a u8.
- `read_i8`: Read an i8.
- `read_u16`: Read a u16.
- `read_i16`: Read an i16.
- `read_u32`: Read a u32.
- `read_i32`: Read an i32.
- `read_u64`: Read a u64.
- `read_i64`: Read an i64.
- `read_f32`: Read a f32.
- `read_f64`: Read a f64.
- `read_uint`: Read an unsigned n-bytes integer u64.
- `read_null_terminated_slice`: Read a null-terminated slice, and return it (excluding the null).
- `skip_leb128`: Skip a LEB128 encoded integer.
- `read_uleb128`: Read an unsigned LEB128 encoded integer.
- `read_uleb128_u32`: Read an unsigned LEB128 encoded u32.
- `read_uleb128_u16`: Read an unsigned LEB128 encoded u16.
- `read_sleb128`: Read a signed LEB128 encoded integer.
- `read_initial_length`: Read an initial length field.
- `read_address_size`: Read a byte and validate it as an address size.
- `read_address`: Read an address-sized integer, and return it as a `u64`.
- `read_word`: Parse a word-sized integer according to the DWARF format.
- `read_length`: Parse a word-sized section length according to the DWARF format.
- `read_offset`: Parse a word-sized section offset according to the DWARF format.
- `read_sized_offset`: Parse a section offset of the given size.



## gimli::read::reader::ReaderOffset

*Trait*

A trait for offsets with a DWARF section.

This allows consumers to choose a size that is appropriate for their address space.

**Methods:**

- `from_u8`: Convert a u8 to an offset.
- `from_u16`: Convert a u16 to an offset.
- `from_i16`: Convert an i16 to an offset.
- `from_u32`: Convert a u32 to an offset.
- `from_u64`: Convert a u64 to an offset.
- `into_u64`: Convert an offset to a u64.
- `wrapping_add`: Wrapping (modular) addition. Computes `self + other`.
- `checked_sub`: Checked subtraction. Computes `self - other`.



## gimli::read::reader::ReaderOffsetId

*Struct*

An identifier for an offset within a section reader.

This is used for error reporting. The meaning of this value is specific to
each reader implementation. The values should be chosen to be unique amongst
all readers. If values are not unique then errors may point to the wrong reader.

**Tuple Struct**: `(u64)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ReaderOffsetId`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ReaderOffsetId) -> bool`



