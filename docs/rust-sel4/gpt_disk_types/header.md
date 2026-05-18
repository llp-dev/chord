**gpt_disk_types > header**

# Module: header

## Contents

**Structs**

- [`GptHeader`](#gptheader) - GPT header that appears near the start and end of a GPT-formatted disk.
- [`GptHeaderRevision`](#gptheaderrevision) - GPT header revision.
- [`GptHeaderSignature`](#gptheadersignature) - GPT header signature.

---

## gpt_disk_types::header::GptHeader

*Struct*

GPT header that appears near the start and end of a GPT-formatted disk.

**Fields:**
- `signature: GptHeaderSignature` - Magic signature for the header. In a valid header this must be
- `revision: GptHeaderRevision` - Revision number for the header. In a valid header this must be
- `header_size: crate::U32Le` - Size of the header in bytes. In a valid header this must be
- `header_crc32: crate::Crc32` - CRC32 checksum of the entire header. When calculating the
- `reserved: crate::U32Le` - Reserved bytes. In a valid header these must be all zero.
- `my_lba: crate::LbaLe` - The LBA that contains this header.
- `alternate_lba: crate::LbaLe` - The LBA that contains the alternate header.
- `first_usable_lba: crate::LbaLe` - First LBA that can be used for the data of a partition in the
- `last_usable_lba: crate::LbaLe` - Last LBA that can be used for the data of a partition in the
- `disk_guid: crate::Guid` - Unique ID for the disk.
- `partition_entry_lba: crate::LbaLe` - First LBA of the partition entry array.
- `number_of_partition_entries: crate::U32Le` - Number of partitions in the partition entry array.
- `size_of_partition_entry: crate::U32Le` - Size in bytes of each entry in the partition entry array.
- `partition_entry_array_crc32: crate::Crc32` - CRC32 checksum of the partition entry array.

**Methods:**

- `fn is_signature_valid(self: &Self) -> bool` - Check if the header's signature matches
- `fn calculate_header_crc32(self: &Self) -> Crc32` - Calculate the header's CRC32 checksum. This returns the checksum
- `fn update_header_crc32(self: & mut Self)` - Update the header's CRC32 checksum.
- `fn get_partition_entry_array_layout(self: &Self) -> Result<GptPartitionEntryArrayLayout, GptPartitionEntrySizeError>` - Get the [`GptPartitionEntryArrayLayout`] for this header.

**Traits:** Zeroable, Copy, Pod, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &GptHeader) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptHeader) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptHeader) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> GptHeader`



## gpt_disk_types::header::GptHeaderRevision

*Struct*

GPT header revision.

**Tuple Struct**: `(crate::U32Le)`

**Methods:**

- `fn major(self: Self) -> u16` - Get the major part of the version.
- `fn minor(self: Self) -> u16` - Get the minor part of the version.

**Traits:** Eq, Zeroable, Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GptHeaderRevision`
- **Default**
  - `fn default() -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &GptHeaderRevision) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptHeaderRevision) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptHeaderRevision) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gpt_disk_types::header::GptHeaderSignature

*Struct*

GPT header signature.

**Tuple Struct**: `(crate::U64Le)`

**Methods:**

- `fn to_u64(self: Self) -> u64` - Convert to [`u64`] with the host's endianness.

**Traits:** Zeroable, Copy, Pod, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GptHeaderSignature) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> GptHeaderSignature`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &GptHeaderSignature) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &GptHeaderSignature) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



