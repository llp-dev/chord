# gpt_disk_types

Library of GPT disk data types.

# GPT disk components

```text
┌───┬───────┬─────────────────┬─────────┬───────────────────┬─────────┐
│MBR│Primary│Primary partition│Partition│Secondary partition│Secondary│
│   │header │entry array      │data     │entry array        │header   │
└───┴───────┴─────────────────┴─────────┴───────────────────┴─────────┘
```

1. The first block of the disk contains a protective MBR. See
   [`MasterBootRecord::protective_mbr`].
2. The second block of the disk contains the primary GPT header. See
   [`GptHeader`].
3. Additional blocks after the header contain the partition entry
   array. See [`GptPartitionEntry`] and [`GptPartitionEntryArray`].
4. At the end of the disk is a secondary GPT header and partition
   entry array.

# Endianness

The UEFI Specification specifies that data structures are little
endian (section 1.8.1 "Data Structure Descriptions"). Unless
otherwise noted, all fields in this library are little endian. This
is true even when running the code on a big-endian architecture; the
[`U16Le`], [`U32Le`], [`U64Le`], and [`LbaLe`] types help enforce
this. The little-endian convention is also used for [`Display`]
implementations. This means bytes within each field will appear
reversed when compared with a flat hex dump of GPT data.

[`Display`]: core::fmt::Display

# Features

* `bytemuck`: Implements bytemuck's `Pod` and `Zeroable` traits for
  many of the types in this crate. Also enables some methods that
  rely on byte access.
* `std`: Currently has no effect.

# Examples

Construct a GPT header:

```
use gpt_disk_types::{guid, Crc32, GptHeader, LbaLe, U32Le};

let header = GptHeader {
    header_crc32: Crc32(U32Le::from_u32(0xa4877843)),
    my_lba: LbaLe::from_u64(1),
    alternate_lba: LbaLe::from_u64(8191),
    first_usable_lba: LbaLe::from_u64(34),
    last_usable_lba: LbaLe::from_u64(8158),
    disk_guid: guid!("57a7feb6-8cd5-4922-b7bd-c78b0914e870"),
    partition_entry_lba: LbaLe::from_u64(2),
    number_of_partition_entries: U32Le::from_u32(128),
    partition_entry_array_crc32: Crc32(U32Le::from_u32(0x9206adff)),
    ..Default::default()
};
```

Construct a GPT partition entry:

```
use gpt_disk_types::{guid, GptPartitionEntry, GptPartitionType, LbaLe};

let entry = GptPartitionEntry {
    partition_type_guid: GptPartitionType(guid!(
        "ccf0994f-f7e0-4e26-a011-843e38aa2eac"
    )),
    unique_partition_guid: guid!("37c75ffd-8932-467a-9c56-8cf1f0456b12"),
    starting_lba: LbaLe::from_u64(2048),
    ending_lba: LbaLe::from_u64(4096),
    attributes: Default::default(),
    name: "hello world!".parse().unwrap(),
};
```

## Modules

### [`block`](block.md)

*4 structs*

### [`crc32`](crc32.md)

*1 struct*

### [`header`](header.md)

*3 structs*

### [`mbr`](mbr.md)

*4 structs*

### [`num`](num.md)

*3 structs*

### [`partition_array`](partition_array.md)

*1 enum, 2 structs*

### [`partition_entry`](partition_entry.md)

*2 enums, 6 structs*

