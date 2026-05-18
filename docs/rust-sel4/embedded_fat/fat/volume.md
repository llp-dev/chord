**embedded_fat > fat > volume**

# Module: fat::volume

## Contents

**Structs**

- [`FatVolume`](#fatvolume) - Identifies a FAT16 or FAT32 Volume on the disk.
- [`VolumeName`](#volumename) - The name given to a particular FAT formatted volume.

**Functions**

- [`parse_volume`](#parse_volume) - Load the boot parameter block from the start of the given partition and

---

## embedded_fat::fat::volume::FatVolume

*Struct*

Identifies a FAT16 or FAT32 Volume on the disk.

**Methods:**

- `fn update_info_sector<D>(self: & mut Self, block_device: &D) -> Result<(), Error<<D as >::Error>>` - Write a new entry in the FAT

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FatVolume) -> bool`



## embedded_fat::fat::volume::VolumeName

*Struct*

The name given to a particular FAT formatted volume.

**Methods:**

- `fn new(data: [u8; 11]) -> VolumeName` - Create a new VolumeName

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &VolumeName) -> bool`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VolumeName`



## embedded_fat::fat::volume::parse_volume

*Function*

Load the boot parameter block from the start of the given partition and
determine if the partition contains a valid FAT16 or FAT32 file system.

```rust
fn parse_volume<D>(block_device: &D, lba_start: crate::BlockIdx, num_blocks: crate::BlockCount) -> Result<crate::VolumeType, crate::Error<<D as >::Error>>
```



