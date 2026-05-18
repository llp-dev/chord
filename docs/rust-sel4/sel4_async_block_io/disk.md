**sel4_async_block_io > disk**

# Module: disk

## Contents

**Structs**

- [`Disk`](#disk)
- [`Mbr`](#mbr)
- [`MbrPartitionEntry`](#mbrpartitionentry)

**Enums**

- [`DiskError`](#diskerror)
- [`KnownPartitionId`](#knownpartitionid)
- [`PartitionId`](#partitionid)

---

## sel4_async_block_io::disk::Disk

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn partition_using_mbr(self: Self, entry: &MbrPartitionEntry) -> Partition<T>`
- `fn new(io: T) -> Self`
- `fn read_mbr(self: &Self) -> Result<Mbr, DiskError<<T as >::Error>>`
- `fn read_gpt_header(self: &Self) -> Result<GptHeader, <T as >::Error>`



## sel4_async_block_io::disk::DiskError

*Enum*

**Generic Parameters:**
- E

**Variants:**
- `IOError(E)`
- `MbrInvalidSignature`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(io_error: E) -> Self`



## sel4_async_block_io::disk::KnownPartitionId

*Enum*

**Variants:**
- `Free`
- `Fat32`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &KnownPartitionId) -> bool`
- **TryFromPrimitive**
  - `fn try_from_primitive(number: <Self as >::Primitive) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &KnownPartitionId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &KnownPartitionId) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> KnownPartitionId`
- **TryFrom**
  - `fn try_from(number: u8) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>>`



## sel4_async_block_io::disk::Mbr

*Struct*

**Methods:**

- `fn disk_signature(self: &Self) -> [u8; 4]`
- `fn partition(self: &Self, i: usize) -> Option<MbrPartitionEntry>`



## sel4_async_block_io::disk::MbrPartitionEntry

*Struct*

**Methods:**

- `fn partition_id(self: &Self) -> PartitionId`



## sel4_async_block_io::disk::PartitionId

*Enum*

**Variants:**
- `Known(KnownPartitionId)`
- `Unknown(u8)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(val: u8) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PartitionId) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &PartitionId) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &PartitionId) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PartitionId`



