**gpt_disk_types > mbr**

# Module: mbr

## Contents

**Structs**

- [`Chs`](#chs) - Legacy MBR cylinder/head/sector.
- [`DiskGeometry`](#diskgeometry) - Legacy disk geometry used for converting between [`Lba`] and [`Chs`].
- [`MasterBootRecord`](#masterbootrecord) - Legacy master boot record.
- [`MbrPartitionRecord`](#mbrpartitionrecord) - Legacy MBR partition record.

---

## gpt_disk_types::mbr::Chs

*Struct*

Legacy MBR cylinder/head/sector.

**Tuple Struct**: `([u8; 3])`

**Methods:**

- `fn cylinder(self: Self) -> u16` - Get the 10 cylinder bits as a [`u16`].
- `fn head(self: Self) -> u8` - Get the 8 head bits as a [`u8`].
- `fn sector(self: Self) -> u8` - Get the 6 sector bits as a [`u8`].
- `fn as_tuple(self: Self) -> (u16, u8, u8)` - Get a tuple of `(cylinder, head, sector)`.
- `fn new(cylinder: u16, head: u8, sector: u8) -> Option<Self>` - Create a new `Chs`. Returns `None` if `cylinder` can't fit in 10
- `fn from_lba(lba: Lba, geom: DiskGeometry) -> Option<Self>` - Convert LBA to CHS address. Returns `None` if the LBA value

**Traits:** Zeroable, Copy, Pod, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Chs) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Chs) -> bool`
- **Default**
  - `fn default() -> Chs`
- **Clone**
  - `fn clone(self: &Self) -> Chs`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Chs) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gpt_disk_types::mbr::DiskGeometry

*Struct*

Legacy disk geometry used for converting between [`Lba`] and [`Chs`].

**Fields:**
- `heads_per_cylinder: u32` - Heads per cylinder.
- `sectors_per_track: u32` - Sectors per track.

**Methods:**


**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DiskGeometry`
- **Default**
  - `fn default() -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &DiskGeometry) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &DiskGeometry) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DiskGeometry) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## gpt_disk_types::mbr::MasterBootRecord

*Struct*

Legacy master boot record.

See Table 5-1 "Legacy MBR" in the UEFI Specification.

**Fields:**
- `boot_strap_code: [u8; 440]` - Executable code used on non-UEFI systems select a partition and
- `unique_mbr_disk_signature: [u8; 4]` - Unique identifier for the disk. This value is not used by UEFI
- `unknown: [u8; 2]` - Reserved field that is not used by UEFI firmware.
- `partitions: [MbrPartitionRecord; 4]` - Four legacy MBR partitions.
- `signature: [u8; 2]` - MBR signature, set to `0xaa55`.

**Methods:**

- `fn is_boot_strap_code_zero(self: &Self) -> bool` - Return whether the [`boot_strap_code`] field is all zeros or not.
- `fn protective_mbr(num_blocks: u64) -> Self` - Create a protective MBR for the given disk size.

**Traits:** Zeroable, Copy, Pod, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &MasterBootRecord) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &MasterBootRecord) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &MasterBootRecord) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> MasterBootRecord`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`



## gpt_disk_types::mbr::MbrPartitionRecord

*Struct*

Legacy MBR partition record.

See Table 5-2 "Legacy MBR Partition Record" in the UEFI Specification.

**Fields:**
- `boot_indicator: u8` - A value of `0x80` indicates this is a legacy bootable
- `start_chs: Chs` - Start of the partition. UEFI firmware does not use this field's
- `os_indicator: u8` - Type of partition. A value of `0xef` defines a system
- `end_chs: Chs` - End of the partition. UEFI firmware does not use this field's
- `starting_lba: crate::U32Le` - Starting LBA of the partition. UEFI firmware uses this field to
- `size_in_lba: crate::U32Le` - Size of the partition in logical blocks. UEFI firmware uses this

**Traits:** Eq, Zeroable, Copy, Pod

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &MbrPartitionRecord) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &MbrPartitionRecord) -> bool`
- **Default**
  - `fn default() -> MbrPartitionRecord`
- **Clone**
  - `fn clone(self: &Self) -> MbrPartitionRecord`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &MbrPartitionRecord) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



