**embedded_fat > filesystem > directory**

# Module: filesystem::directory

## Contents

**Structs**

- [`DirEntry`](#direntry) - Represents a directory entry, which tells you about
- [`Directory`](#directory) - Represents an open directory on disk.

---

## embedded_fat::filesystem::directory::DirEntry

*Struct*

Represents a directory entry, which tells you about
other files and directories.

**Fields:**
- `name: crate::filesystem::ShortFileName` - The name of the file
- `mtime: crate::filesystem::Timestamp` - When the file was last modified
- `ctime: crate::filesystem::Timestamp` - When the file was first created
- `attributes: crate::filesystem::Attributes` - The file attributes (Read Only, Archive, etc)
- `cluster: crate::filesystem::ClusterId` - The starting cluster of the file. The FAT tells us the following Clusters.
- `size: u32` - The size of the file in bytes.
- `entry_block: crate::blockdevice::BlockIdx` - The disk block of this entry
- `entry_offset: u32` - The offset on its block (in bytes)

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DirEntry) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> DirEntry`



## embedded_fat::filesystem::directory::Directory

*Struct*

Represents an open directory on disk.

Do NOT drop this object! It doesn't hold a reference to the Volume Manager
it was created from and if you drop it, the VolumeManager will think you
still have the directory open, and it won't let you open the directory
again.

Instead you must pass it to [`crate::VolumeManager::close_dir`] to close it
cleanly.

If you want your directories to close themselves on drop, create your own
`Directory` type that wraps this one and also holds a `VolumeManager`
reference. You'll then also need to put your `VolumeManager` in some kind of
Mutex or RefCell, and deal with the fact you can't put them both in the same
struct any more because one refers to the other. Basically, it's complicated
and there's a reason we did it this way.

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Directory) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Directory`



