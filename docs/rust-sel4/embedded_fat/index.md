# embedded_fat

# embedded-sdmmc

> An SD/MMC Library written in Embedded Rust

This crate is intended to allow you to read/write files on a FAT formatted
SD card on your Rust Embedded device, as easily as using the `SdFat` Arduino
library. It is written in pure-Rust, is `#![no_std]` and does not use
`alloc` or `collections` to keep the memory footprint low. In the first
instance it is designed for readability and simplicity over performance.

## Using the crate

You will need something that implements the `BlockDevice` trait, which can
read and write the 512-byte blocks (or sectors) from your card. If you were
to implement this over USB Mass Storage, there's no reason this crate
couldn't work with a USB Thumb Drive, but we only supply a `BlockDevice`
suitable for reading SD and SDHC cards over SPI.

## Features

* `log`: Enabled by default. Generates log messages using the `log` crate.
* `defmt-log`: By turning off the default features and enabling the
`defmt-log` feature you can configure this crate to log messages over defmt
instead.

You cannot enable both the `log` feature and the `defmt-log` feature.

## Modules

### [`embedded_fat`](embedded_fat.md)

*2 enums, 3 modules*

### [`blockdevice`](blockdevice.md)

*1 trait, 4 structs*

### [`fat`](fat.md)

*1 constant, 1 enum*

### [`fat::bpb`](fat/bpb.md)

*1 struct*

### [`fat::info`](fat/info.md)

*1 enum, 3 structs*

### [`fat::lfn`](fat/lfn.md)

*1 struct*

### [`fat::ondiskdirentry`](fat/ondiskdirentry.md)

*1 struct*

### [`fat::volume`](fat/volume.md)

*1 function, 2 structs*

### [`filesystem`](filesystem.md)

*1 constant*

### [`filesystem::attributes`](filesystem/attributes.md)

*1 struct*

### [`filesystem::cluster`](filesystem/cluster.md)

*1 struct*

### [`filesystem::directory`](filesystem/directory.md)

*2 structs*

### [`filesystem::filename`](filesystem/filename.md)

*1 enum, 1 struct, 1 trait*

### [`filesystem::files`](filesystem/files.md)

*1 struct, 2 enums*

### [`filesystem::search_id`](filesystem/search_id.md)

*2 structs*

### [`filesystem::timestamp`](filesystem/timestamp.md)

*1 struct, 1 trait*

### [`volume`](volume.md)

*1 struct*

