**gpt_disk_types > partition_array**

# Module: partition_array

## Contents

**Structs**

- [`GptPartitionEntryArray`](#gptpartitionentryarray) - Storage for a GPT partition entry array.
- [`GptPartitionEntryArrayLayout`](#gptpartitionentryarraylayout) - Disk layout of a GPT partition entry array.

**Enums**

- [`GptPartitionEntryArrayError`](#gptpartitionentryarrayerror) - Errors used by [`GptPartitionEntryArray`].

---

## gpt_disk_types::partition_array::GptPartitionEntryArray

*Struct*

Storage for a GPT partition entry array.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(layout: GptPartitionEntryArrayLayout, block_size: BlockSize, storage: &'a  mut [u8]) -> Result<Self, GptPartitionEntryArrayError>` - Create a new `GptPartitionEntryArray` with the given
- `fn storage(self: &Self) -> &[u8]` - Get a reference to the storage buffer.
- `fn storage_mut(self: & mut Self) -> & mut [u8]` - Get a mutable reference to the storage buffer.
- `fn layout(self: &Self) -> &GptPartitionEntryArrayLayout` - Get the partition entry array layout.
- `fn set_start_lba(self: & mut Self, start_lba: Lba)` - Change the partition entry array's start [`Lba`].
- `fn get_partition_entry(self: &Self, index: u32) -> Option<&GptPartitionEntry>` - Get a partition entry reference. The `index` is zero-based.
- `fn get_partition_entry_mut(self: & mut Self, index: u32) -> Option<& mut GptPartitionEntry>` - Get a mutable partition entry reference. The `index` is zero-based.
- `fn calculate_crc32(self: &Self) -> Crc32` - Calculate the CRC32 checksum for the partition entry array. The



## gpt_disk_types::partition_array::GptPartitionEntryArrayError

*Enum*

Errors used by [`GptPartitionEntryArray`].

**Variants:**
- `BufferTooSmall` - The storage buffer is not large enough. It must be at least
- `Overflow` - Numeric overflow occurred.

**Traits:** Eq, Copy, Error

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GptPartitionEntryArrayError`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionEntryArrayError) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionEntryArrayError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionEntryArrayError) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gpt_disk_types::partition_array::GptPartitionEntryArrayLayout

*Struct*

Disk layout of a GPT partition entry array.

**Fields:**
- `start_lba: crate::Lba` - First block of the array.
- `entry_size: crate::GptPartitionEntrySize` - Size in bytes of each entry.
- `num_entries: u32` - Number of entries in the array.

**Methods:**

- `fn num_blocks(self: &Self, block_size: BlockSize) -> Option<u64>` - Get the number of blocks needed for this layout. Returns `None`
- `fn num_blocks_as_usize(self: &Self, block_size: BlockSize) -> Option<usize>` - Get the number of blocks needed for this layout. Returns `None`
- `fn num_bytes_exact(self: &Self) -> Option<u64>` - Get the number of bytes needed for the entries in this layout,
- `fn num_bytes_exact_as_usize(self: &Self) -> Option<usize>` - Get the number of bytes needed for the entries in this layout,
- `fn num_bytes_rounded_to_block(self: &Self, block_size: BlockSize) -> Option<u64>` - Get the number of bytes needed for this layout, rounded up to
- `fn num_bytes_rounded_to_block_as_usize(self: &Self, block_size: BlockSize) -> Option<usize>` - Get the number of bytes needed for this layout, rounded up to

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GptPartitionEntryArrayLayout`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionEntryArrayLayout) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionEntryArrayLayout) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionEntryArrayLayout) -> bool`
- **Default**
  - `fn default() -> GptPartitionEntryArrayLayout`



