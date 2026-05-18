**embedded_fat**

# Module: embedded_fat

## Contents

**Modules**

- [`blockdevice`](#blockdevice) - Block Device support
- [`fat`](#fat) - FAT16/FAT32 file system implementation
- [`filesystem`](#filesystem) - Generic File System structures

**Enums**

- [`Error`](#error) - Represents all the ways the functions in this crate can fail.
- [`VolumeType`](#volumetype) - This enum holds the data for the various different types of filesystems we

---

## embedded_fat::Error

*Enum*

Represents all the ways the functions in this crate can fail.

**Generic Parameters:**
- E

**Variants:**
- `DeviceError(E)` - The underlying block device threw an error.
- `FormatError(&'static str)` - The filesystem is badly formatted (or this code is buggy).
- `NoSuchVolume` - The given `VolumeIdx` was bad,
- `FilenameError(FilenameError)` - The given filename was bad
- `TooManyOpenVolumes` - Out of memory opening volumes
- `TooManyOpenDirs` - Out of memory opening directories
- `TooManyOpenFiles` - Out of memory opening files
- `BadHandle` - Bad handle given
- `FileNotFound` - That file doesn't exist
- `FileAlreadyOpen` - You can't open a file twice or delete an open file
- `DirAlreadyOpen` - You can't open a directory twice
- `OpenedDirAsFile` - You can't open a directory as a file
- `OpenedFileAsDir` - You can't open a file as a directory
- `DeleteDirAsFile` - You can't delete a directory as a file
- `VolumeStillInUse` - You can't close a volume with open files or directories
- `VolumeAlreadyOpen` - You can't open a volume twice
- `Unsupported` - We can't do that yet
- `EndOfFile` - Tried to read beyond end of file
- `BadCluster` - Found a bad cluster
- `ConversionError` - Error while converting types
- `NotEnoughSpace` - The device does not have enough space for the operation
- `AllocationError` - Cluster was not properly allocated by the library
- `UnterminatedFatChain` - Jumped to free space during FAT traversing
- `ReadOnly` - Tried to open Read-Only file with write mode
- `FileAlreadyExists` - Tried to create an existing file
- `BadBlockSize(u16)` - Bad block size - only 512 byte blocks supported
- `InvalidOffset` - Bad offset given when seeking

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error<E>`
- **From**
  - `fn from(value: E) -> Error<E>`



## embedded_fat::VolumeType

*Enum*

This enum holds the data for the various different types of filesystems we
support.

**Variants:**
- `Fat(FatVolume)` - FAT16/FAT32 formatted volumes.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VolumeType) -> bool`



## Module: blockdevice

Block Device support

Generic code for handling block devices, such as types for identifying
a particular block on a block device by its index.



## Module: fat

FAT16/FAT32 file system implementation

Implements the File Allocation Table file system. Supports FAT16 and FAT32 volumes.



## Module: filesystem

Generic File System structures

Implements generic file system components. These should be applicable to
most (if not all) supported filesystems.



