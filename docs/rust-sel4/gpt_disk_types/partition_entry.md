**gpt_disk_types > partition_entry**

# Module: partition_entry

## Contents

**Structs**

- [`GptPartitionAttributes`](#gptpartitionattributes) - Partition attribute bits.
- [`GptPartitionEntry`](#gptpartitionentry) - An entry within the GPT partition array.
- [`GptPartitionEntrySize`](#gptpartitionentrysize) - Size in bytes of entries in the partition entry array.
- [`GptPartitionEntrySizeError`](#gptpartitionentrysizeerror) - Error returned by [`GptPartitionEntrySize::new`].
- [`GptPartitionName`](#gptpartitionname) - Human readable partition label encoded as a null-terminated UCS-2
- [`GptPartitionType`](#gptpartitiontype) - Unique ID representing the type of a partition.

**Enums**

- [`GptPartitionNameFromStrError`](#gptpartitionnamefromstrerror) - Error type for [`GptPartitionName::from_str`].
- [`GptPartitionNameSetCharError`](#gptpartitionnamesetcharerror) - Error type for [`GptPartitionName::set_char`].

---

## gpt_disk_types::partition_entry::GptPartitionAttributes

*Struct*

Partition attribute bits.

**Tuple Struct**: `(crate::U64Le)`

**Methods:**

- `fn required_partition(self: Self) -> bool` - Get the [`REQUIRED_PARTITION_BIT`] attribute value.
- `fn update_required_partition(self: & mut Self, required: bool)` - Update the [`REQUIRED_PARTITION_BIT`] attribute value.
- `fn no_block_io_protocol(self: Self) -> bool` - Get the [`NO_BLOCK_IO_PROTOCOL_BIT`] attribute value.
- `fn update_no_block_io_protocol(self: & mut Self, no_block_io_protocol: bool)` - Update the [`NO_BLOCK_IO_PROTOCOL_BIT`] attribute value.
- `fn legacy_bios_bootable(self: Self) -> bool` - Get the [`LEGACY_BIOS_BOOTABLE_BIT`] attribute value.
- `fn update_legacy_bios_bootable(self: & mut Self, legacy_bios_bootable: bool)` - Update the [`LEGACY_BIOS_BOOTABLE_BIT`] attribute value.
- `fn type_specific_attributes(self: Self) -> U16Le` - Bits `48..=63` represented as a [`U16Le`]. These bits are
- `fn update_type_specific_attributes(self: & mut Self, attrs: U16Le)` - Set bits `48..=63`. These bits are reserved for custom use by

**Traits:** Eq, Zeroable, Copy, Pod

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionAttributes) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionAttributes) -> bool`
- **Default**
  - `fn default() -> GptPartitionAttributes`
- **Clone**
  - `fn clone(self: &Self) -> GptPartitionAttributes`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionAttributes) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gpt_disk_types::partition_entry::GptPartitionEntry

*Struct*

An entry within the GPT partition array.

**Fields:**
- `partition_type_guid: GptPartitionType` - Unique ID representing the partition's type. If the type is
- `unique_partition_guid: crate::Guid` - GUID that is unique for every partition entry.
- `starting_lba: crate::LbaLe` - LBA of the partition's first block.
- `ending_lba: crate::LbaLe` - LBA of the partition's last block.
- `attributes: GptPartitionAttributes` - Attribute bit flags.
- `name: GptPartitionName` - Human readable partition label encoded as a null-terminated

**Methods:**

- `fn lba_range(self: &Self) -> Option<LbaRangeInclusive>` - Get the range of blocks covered by this partition. Returns
- `fn is_used(self: &Self) -> bool` - Check if the entry is in use. If the [`partition_type_guid`] is

**Traits:** Copy, Pod, Eq, Zeroable

**Trait Implementations:**

- **Default**
  - `fn default() -> GptPartitionEntry`
- **Clone**
  - `fn clone(self: &Self) -> GptPartitionEntry`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionEntry) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionEntry) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionEntry) -> bool`



## gpt_disk_types::partition_entry::GptPartitionEntrySize

*Struct*

Size in bytes of entries in the partition entry array.

A valid partition entry size must be a value of 128×2ⁿ, where n is
an integer ≥0.

**Tuple Struct**: `()`

**Methods:**

- `fn new(val: u32) -> Result<Self, GptPartitionEntrySizeError>` - Create a new `GptPartitionEntrySize`. Returns
- `fn to_u32(self: Self) -> u32` - Get the entry size in bytes as a [`u32`].
- `fn to_u64(self: Self) -> u64` - Get the entry size in bytes as a [`u64`].
- `fn to_usize(self: Self) -> Option<usize>` - Get the entry size in bytes as a [`usize`].

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionEntrySize) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionEntrySize) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> GptPartitionEntrySize`
- **Default**
  - `fn default() -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionEntrySize) -> $crate::cmp::Ordering`



## gpt_disk_types::partition_entry::GptPartitionEntrySizeError

*Struct*

Error returned by [`GptPartitionEntrySize::new`].

**Unit Struct**

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GptPartitionEntrySizeError`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionEntrySizeError) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionEntrySizeError) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionEntrySizeError) -> bool`
- **Default**
  - `fn default() -> GptPartitionEntrySizeError`



## gpt_disk_types::partition_entry::GptPartitionName

*Struct*

Human readable partition label encoded as a null-terminated UCS-2
string.

# Examples

Construct from a UTF-8 string:

```
use gpt_disk_types::GptPartitionName;

let partition_name: GptPartitionName = "hacktheplanet".parse().unwrap();
```

**Tuple Struct**: `([u8; 72])`

**Methods:**

- `fn is_empty(self: &Self) -> bool` - True if the first character is a null terminator, false otherwise.
- `fn chars(self: &Self) -> impl Trait` - Get an iterator over the characters in the partition name, using
- `fn set_char(self: & mut Self, index: usize, c: char) -> Result<(), GptPartitionNameSetCharError>` - Set a UCS-2 character. The `index` is by UCS-2 character rather

**Traits:** Zeroable, Pod, Copy, Eq

**Trait Implementations:**

- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`
- **Default**
  - `fn default() -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionName) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionName) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionName) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> GptPartitionName`



## gpt_disk_types::partition_entry::GptPartitionNameFromStrError

*Enum*

Error type for [`GptPartitionName::from_str`].

**Variants:**
- `Length` - Input string is too long.
- `InvalidChar` - Input string contains a character that cannot be represented in UCS-2.

**Traits:** Eq, Error, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GptPartitionNameFromStrError`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionNameFromStrError) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionNameFromStrError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(err: ucs2::Error) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionNameFromStrError) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gpt_disk_types::partition_entry::GptPartitionNameSetCharError

*Enum*

Error type for [`GptPartitionName::set_char`].

**Variants:**
- `Index` - Character index is outside the range `0..36`.
- `InvalidChar` - Character cannot be represented in UCS-2.

**Traits:** Copy, Error, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionNameSetCharError) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionNameSetCharError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionNameSetCharError) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> GptPartitionNameSetCharError`



## gpt_disk_types::partition_entry::GptPartitionType

*Struct*

Unique ID representing the type of a partition.

**Tuple Struct**: `(crate::Guid)`

**Methods:**


**Traits:** Copy, Pod, Eq, Zeroable

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &GptPartitionType) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptPartitionType) -> bool`
- **Default**
  - `fn default() -> GptPartitionType`
- **Clone**
  - `fn clone(self: &Self) -> GptPartitionType`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>` - Parse from a GUID string. See [`Guid::from_str`].
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptPartitionType) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`



