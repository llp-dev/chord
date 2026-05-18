**embedded_fat > fat > ondiskdirentry**

# Module: fat::ondiskdirentry

## Contents

**Structs**

- [`OnDiskDirEntry`](#ondiskdirentry) - Represents a 32-byte directory entry as stored on-disk in a directory file.

---

## embedded_fat::fat::ondiskdirentry::OnDiskDirEntry

*Struct*

Represents a 32-byte directory entry as stored on-disk in a directory file.

**Generic Parameters:**
- 'a

**Methods:**

- `fn raw_attr(self: &Self) -> u8` - Get the value from the $name field
- `fn create_time(self: &Self) -> u16` - Get the value from the $name field
- `fn create_date(self: &Self) -> u16` - Get the value from the $name field
- `fn last_access_data(self: &Self) -> u16` - Get the value from the $name field
- `fn first_cluster_hi(self: &Self) -> u16` - Get the value from the $name field
- `fn write_time(self: &Self) -> u16` - Get the value from the $name field
- `fn write_date(self: &Self) -> u16` - Get the value from the $name field
- `fn first_cluster_lo(self: &Self) -> u16` - Get the value from the $name field
- `fn file_size(self: &Self) -> u32` - Get the $name field
- `fn new(data: &[u8]) -> OnDiskDirEntry` - Create a new on-disk directory entry from a block of 32 bytes read
- `fn is_end(self: &Self) -> bool` - Is this the last entry in the directory?
- `fn is_valid(self: &Self) -> bool` - Is this a valid entry?
- `fn is_lfn(self: &Self) -> bool` - Is this a Long Filename entry?
- `fn lfn_contents(self: &Self) -> Option<LfnEntry>` - If this is an LFN, get the contents so we can re-assemble the filename.
- `fn matches(self: &Self, sfn: &ShortFileName) -> bool` - Does this on-disk entry match the given filename?
- `fn first_cluster_fat32(self: &Self) -> ClusterId` - Which cluster, if any, does this file start at? Assumes this is from a FAT32 volume.
- `fn get_entry(self: &Self, fat_type: FatType, entry_block: BlockIdx, entry_offset: u32) -> DirEntry` - Convert the on-disk format into a DirEntry

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



