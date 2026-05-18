**embedded_fat > fat > info**

# Module: fat::info

## Contents

**Structs**

- [`Fat16Info`](#fat16info) - FAT16 specific data
- [`Fat32Info`](#fat32info) - FAT32 specific data
- [`InfoSector`](#infosector) - File System Information structure is only present on FAT32 partitions. It

**Enums**

- [`FatSpecificInfo`](#fatspecificinfo) - Indentifies the supported types of FAT format

---

## embedded_fat::fat::info::Fat16Info

*Struct*

FAT16 specific data

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Fat16Info) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Fat16Info`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## embedded_fat::fat::info::Fat32Info

*Struct*

FAT32 specific data

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Fat32Info) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Fat32Info`



## embedded_fat::fat::info::FatSpecificInfo

*Enum*

Indentifies the supported types of FAT format

**Variants:**
- `Fat16(Fat16Info)` - Fat16 Format
- `Fat32(Fat32Info)` - Fat32 Format

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FatSpecificInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FatSpecificInfo`



## embedded_fat::fat::info::InfoSector

*Struct*

File System Information structure is only present on FAT32 partitions. It
may contain a valid number of free clusters and the number of the next
free cluster. The information contained in the structure must be
considered as advisory only. File system driver implementations are not
required to ensure that information within the structure is kept
consistent.

**Generic Parameters:**
- 'a

**Methods:**

- `fn create_from_bytes(data: &[u8; 512]) -> Result<InfoSector, &'static str>` - Try and create a new Info Sector from a block.
- `fn lead_sig(self: &Self) -> u32` - Get the $name field
- `fn struc_sig(self: &Self) -> u32` - Get the $name field
- `fn free_count(self: &Self) -> u32` - Get the $name field
- `fn next_free(self: &Self) -> u32` - Get the $name field
- `fn trail_sig(self: &Self) -> u32` - Get the $name field
- `fn free_clusters_count(self: &Self) -> Option<u32>` - Return how many free clusters are left in this volume, if known.
- `fn next_free_cluster(self: &Self) -> Option<ClusterId>` - Return the number of the next free cluster, if known.



