**embedded_fat > fat > bpb**

# Module: fat::bpb

## Contents

**Structs**

- [`Bpb`](#bpb) - Represents a Boot Parameter Block. This is the first sector of a FAT

---

## embedded_fat::fat::bpb::Bpb

*Struct*

Represents a Boot Parameter Block. This is the first sector of a FAT
formatted partition, and it describes various properties of the FAT
filesystem.

**Generic Parameters:**
- 'a

**Methods:**

- `fn create_from_bytes(data: &[u8; 512]) -> Result<Bpb, &'static str>` - Attempt to parse a Boot Parameter Block from a 512 byte sector.
- `fn bytes_per_block(self: &Self) -> u16` - Get the value from the $name field
- `fn blocks_per_cluster(self: &Self) -> u8` - Get the value from the $name field
- `fn reserved_block_count(self: &Self) -> u16` - Get the value from the $name field
- `fn num_fats(self: &Self) -> u8` - Get the value from the $name field
- `fn root_entries_count(self: &Self) -> u16` - Get the value from the $name field
- `fn total_blocks16(self: &Self) -> u16` - Get the value from the $name field
- `fn media(self: &Self) -> u8` - Get the value from the $name field
- `fn fat_size16(self: &Self) -> u16` - Get the value from the $name field
- `fn blocks_per_track(self: &Self) -> u16` - Get the value from the $name field
- `fn num_heads(self: &Self) -> u16` - Get the value from the $name field
- `fn hidden_blocks(self: &Self) -> u32` - Get the $name field
- `fn total_blocks32(self: &Self) -> u32` - Get the $name field
- `fn footer(self: &Self) -> u16` - Get the value from the $name field
- `fn fat_size32(self: &Self) -> u32` - Get the $name field
- `fn fs_ver(self: &Self) -> u16` - Get the value from the $name field
- `fn first_root_dir_cluster(self: &Self) -> u32` - Get the $name field
- `fn fs_info(self: &Self) -> u16` - Get the value from the $name field
- `fn backup_boot_block(self: &Self) -> u16` - Get the value from the $name field
- `fn oem_name(self: &Self) -> &[u8]` - Get the OEM name string for this volume
- `fn volume_label(self: &Self) -> &[u8]` - Get the Volume Label string for this volume
- `fn fs_info_block(self: &Self) -> Option<BlockCount>` - On a FAT32 volume, return the free block count from the Info Block. On
- `fn fat_size(self: &Self) -> u32` - Get the size of the File Allocation Table in blocks.
- `fn total_blocks(self: &Self) -> u32` - Get the total number of blocks in this filesystem.
- `fn total_clusters(self: &Self) -> u32` - Get the total number of clusters in this filesystem.



