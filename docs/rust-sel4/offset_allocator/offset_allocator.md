**offset_allocator**

# Module: offset_allocator

## Contents

**Modules**

- [`ext`](#ext) - Extension functions not present in the original C++ `OffsetAllocator`.

**Structs**

- [`Allocation`](#allocation) - A single allocation.
- [`Allocator`](#allocator) - An allocator that manages a single contiguous chunk of space and hands out
- [`StorageReport`](#storagereport) - Provides a summary of the state of the allocator, including space remaining.
- [`StorageReportFull`](#storagereportfull) - Provides a detailed accounting of each bin within the allocator.
- [`StorageReportFullRegion`](#storagereportfullregion) - A detailed accounting of each allocator bin.

**Traits**

- [`NodeIndex`](#nodeindex) - Determines the number of allocations that the allocator supports.
- [`NodeIndexNonMax`](#nodeindexnonmax) - The `NonMax` version of the [`NodeIndex`].

---

## offset_allocator::Allocation

*Struct*

A single allocation.

**Generic Parameters:**
- NI

**Fields:**
- `offset: u32` - The location of this allocation within the buffer.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Allocation<NI>`



## offset_allocator::Allocator

*Struct*

An allocator that manages a single contiguous chunk of space and hands out
portions of it as requested.

**Generic Parameters:**
- NI

**Methods:**

- `fn new(size: u32) -> Self` - Creates a new allocator, managing a contiguous block of memory of `size`
- `fn with_max_allocs(size: u32, max_allocs: u32) -> Self` - Creates a new allocator, managing a contiguous block of memory of `size`
- `fn reset(self: & mut Self)` - Clears out all allocations.
- `fn allocate(self: & mut Self, size: u32) -> Option<Allocation<NI>>` - Allocates a block of `size` elements and returns its allocation.
- `fn free(self: & mut Self, allocation: Allocation<NI>)` - Frees an allocation, returning the data to the heap.
- `fn allocation_size(self: &Self, allocation: Allocation<NI>) -> u32` - Returns the *used* size of an allocation.
- `fn storage_report(self: &Self) -> StorageReport` - Returns a structure containing the amount of free space remaining, as
- `fn storage_report_full(self: &Self) -> StorageReportFull` - Returns detailed information about the number of allocations in each

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> FmtResult`



## offset_allocator::NodeIndex

*Trait*

Determines the number of allocations that the allocator supports.

By default, [`Allocator`] and related functions use `u32`, which allows for
`u32::MAX - 1` allocations. You can, however, use `u16` instead, which
causes the allocator to use less memory but limits the number of allocations
within a single allocator to at most 65,534.

**Methods:**

- `NonMax`: The `NonMax` version of this type.
- `MAX`: The maximum value representable in this type.
- `from_u32`: Converts from a unsigned 32-bit integer to an instance of this type.
- `to_usize`: Converts this type to an unsigned machine word.



## offset_allocator::NodeIndexNonMax

*Trait*

The `NonMax` version of the [`NodeIndex`].

For example, for `u32`, the `NonMax` version is [`NonMaxU32`].

**Methods:**

- `to_usize`: Converts this type to an unsigned machine word.



## offset_allocator::StorageReport

*Struct*

Provides a summary of the state of the allocator, including space remaining.

**Fields:**
- `total_free_space: u32` - The amount of free space left.
- `largest_free_region: u32` - The maximum potential size of a single contiguous allocation.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## offset_allocator::StorageReportFull

*Struct*

Provides a detailed accounting of each bin within the allocator.

**Fields:**
- `free_regions: [StorageReportFullRegion; 256]` - Each bin within the allocator.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`



## offset_allocator::StorageReportFullRegion

*Struct*

A detailed accounting of each allocator bin.

**Fields:**
- `size: u32` - The size of the bin, in units.
- `count: u32` - The number of allocations in the bin.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> StorageReportFullRegion`
- **Default**
  - `fn default() -> StorageReportFullRegion`



## Module: ext

Extension functions not present in the original C++ `OffsetAllocator`.



