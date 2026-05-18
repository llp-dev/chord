**gpt_disk_types > block**

# Module: block

## Contents

**Structs**

- [`BlockSize`](#blocksize) - Size of a block in bytes.
- [`Lba`](#lba) - Logical block address.
- [`LbaLe`](#lbale) - Logical block address stored as a [`U64Le`].
- [`LbaRangeInclusive`](#lbarangeinclusive) - Inclusive range of logical block addresses.

---

## gpt_disk_types::block::BlockSize

*Struct*

Size of a block in bytes.

This type enforces some restrictions on the block size: it must be
at least 512 bytes and fit within a [`u32`].

# Minimum size

The [`MasterBootRecord`] size is 512 bytes and must fit within a
block, so the block size must be at least that large.

[`MasterBootRecord`]: crate::MasterBootRecord

**Tuple Struct**: `()`

**Methods:**

- `fn new(num_bytes: u32) -> Option<Self>` - Create a `BlockSize`.
- `fn from_usize(num_bytes: usize) -> Option<Self>` - Create a `BlockSize`.
- `fn to_u32(self: Self) -> u32` - Get the size in bytes as a [`u32`].
- `fn to_u64(self: Self) -> u64` - Get the size in bytes as a [`u64`].
- `fn to_usize(self: Self) -> Option<usize>` - Get the size in bytes as a [`usize`].
- `fn is_multiple_of_block_size<T>(self: &Self, value: T) -> bool` - Check if `value` is an even multiple of the block size.
- `fn assert_valid_block_buffer(self: &Self, buffer: &[u8])` - Assert that the `buffer` size is an even multiple of the block size.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockSize) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> BlockSize`
- **Default**
  - `fn default() -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockSize) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockSize) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gpt_disk_types::block::Lba

*Struct*

Logical block address.

**Tuple Struct**: `(u64)`

**Methods:**

- `fn to_u64(self: Self) -> u64` - Convert to a plain [`u64`].

**Traits:** Copy, Zeroable, Pod, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Lba`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Lba) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Lba) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Lba) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &u64) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Lba`
- **From**
  - `fn from(lba: LbaLe) -> Self`



## gpt_disk_types::block::LbaLe

*Struct*

Logical block address stored as a [`U64Le`].

**Tuple Struct**: `(crate::U64Le)`

**Methods:**

- `fn from_u64(v: u64) -> Self` - Create a logical block address from a [`u64`].
- `fn to_u64(self: Self) -> u64` - Get the logical block address as a [`u64`].

**Traits:** Copy, Pod, Eq, Zeroable

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &LbaLe) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &LbaLe) -> bool`
- **Default**
  - `fn default() -> LbaLe`
- **Clone**
  - `fn clone(self: &Self) -> LbaLe`
- **From**
  - `fn from(lba: Lba) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &LbaLe) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gpt_disk_types::block::LbaRangeInclusive

*Struct*

Inclusive range of logical block addresses.

**Methods:**

- `fn new(start: Lba, end: Lba) -> Option<LbaRangeInclusive>` - Create an LBA range. The end address must be greater than or
- `fn start(self: Self) -> Lba` - Starting LBA (inclusive).
- `fn end(self: Self) -> Lba` - Ending LBA (inclusive).
- `fn from_byte_range(byte_range: RangeInclusive<u64>, block_size: BlockSize) -> Option<Self>` - Create an LBA range from the corresponding byte range for the
- `fn to_byte_range(self: Self, block_size: BlockSize) -> Option<RangeInclusive<u64>>` - Convert the LBA range to the corresponding byte range for the
- `fn num_bytes(self: Self, block_size: BlockSize) -> Option<u64>` - Get the number of bytes in the LBA range for the given block
- `fn num_blocks(self: Self) -> u64` - Get the number of blocks in the LBA range.

**Traits:** Eq, Zeroable, Copy, Pod

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &LbaRangeInclusive) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &LbaRangeInclusive) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &LbaRangeInclusive) -> bool`
- **Default**
  - `fn default() -> LbaRangeInclusive`
- **Clone**
  - `fn clone(self: &Self) -> LbaRangeInclusive`



