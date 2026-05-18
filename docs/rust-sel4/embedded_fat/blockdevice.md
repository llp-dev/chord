**embedded_fat > blockdevice**

# Module: blockdevice

## Contents

**Structs**

- [`Block`](#block) - Represents a standard 512 byte block (also known as a sector). IBM PC
- [`BlockCount`](#blockcount) - Represents the a number of blocks (or sectors). Add this to a `BlockIdx`
- [`BlockIdx`](#blockidx) - Represents the linear numeric address of a block (or sector). The first
- [`BlockIter`](#blockiter) - An iterator returned from `Block::range`.

**Traits**

- [`BlockDevice`](#blockdevice) - Represents a block device - a device which can read and write blocks (or

---

## embedded_fat::blockdevice::Block

*Struct*

Represents a standard 512 byte block (also known as a sector). IBM PC
formatted 5.25" and 3.5" floppy disks, SD/MMC cards up to 1 GiB in size
and IDE/SATA Hard Drives up to about 2 TiB all have 512 byte blocks.

This library does not support devices with a block size other than 512
bytes.

**Fields:**
- `contents: [u8; 512]` - The 512 bytes in this block (or sector).

**Methods:**

- `fn new() -> Block` - Create a new block full of zeros.

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &[u8; 512]`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut core::fmt::Formatter) -> core::fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut [u8; 512]`
- **Clone**
  - `fn clone(self: &Self) -> Block`



## embedded_fat::blockdevice::BlockCount

*Struct*

Represents the a number of blocks (or sectors). Add this to a `BlockIdx`
to get an actual address on disk.

**Tuple Struct**: `(u32)`

**Methods:**

- `fn from_bytes(byte_count: u32) -> BlockCount` - How many blocks are required to hold this many bytes.
- `fn offset_bytes(self: Self, offset: u32) -> Self` - Take a number of blocks and increment by the integer number of blocks

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockCount) -> $crate::option::Option<$crate::cmp::Ordering>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: BlockCount)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Sub**
  - `fn sub(self: Self, rhs: BlockCount) -> BlockCount`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockCount) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> BlockCount`
- **Add**
  - `fn add(self: Self, rhs: BlockCount) -> BlockCount`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockCount) -> bool`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: BlockCount)`



## embedded_fat::blockdevice::BlockDevice

*Trait*

Represents a block device - a device which can read and write blocks (or
sectors). Only supports devices which are <= 2 TiB in size.

**Methods:**

- `Error`: The errors that the `BlockDevice` can return. Must be debug formattable.
- `read`: Read one or more blocks, starting at the given block index.
- `write`: Write one or more blocks, starting at the given block index.
- `num_blocks`: Determine how many blocks this device can hold.



## embedded_fat::blockdevice::BlockIdx

*Struct*

Represents the linear numeric address of a block (or sector). The first
block on a disk gets `BlockIdx(0)` (which usually contains the Master Boot
Record).

**Tuple Struct**: `(u32)`

**Methods:**

- `fn into_bytes(self: Self) -> u64` - Convert a block index into a 64-bit byte offset from the start of the
- `fn range(self: Self, num: BlockCount) -> BlockIter` - Create an iterator from the current `BlockIdx` through the given

**Traits:** Eq, Copy

**Trait Implementations:**

- **Add**
  - `fn add(self: Self, rhs: BlockCount) -> BlockIdx`
- **Ord**
  - `fn cmp(self: &Self, other: &BlockIdx) -> $crate::cmp::Ordering`
- **Sub**
  - `fn sub(self: Self, rhs: BlockCount) -> BlockIdx`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: BlockCount)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, rhs: BlockCount)`
- **PartialEq**
  - `fn eq(self: &Self, other: &BlockIdx) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BlockIdx`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &BlockIdx) -> $crate::option::Option<$crate::cmp::Ordering>`



## embedded_fat::blockdevice::BlockIter

*Struct*

An iterator returned from `Block::range`.

**Methods:**

- `fn new(start: BlockIdx, inclusive_end: BlockIdx) -> BlockIter` - Create a new `BlockIter`, from the given start block, through (and

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



