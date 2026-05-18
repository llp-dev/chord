**embedded_fat > filesystem > files**

# Module: filesystem::files

## Contents

**Structs**

- [`File`](#file) - Represents an open file on disk.

**Enums**

- [`FileError`](#fileerror) - Errors related to file operations
- [`Mode`](#mode) - The different ways we can open a file.

---

## embedded_fat::filesystem::files::File

*Struct*

Represents an open file on disk.

Do NOT drop this object! It doesn't hold a reference to the Volume Manager
it was created from and cannot update the directory entry if you drop it.
Additionally, the VolumeManager will think you still have the file open if
you just drop it, and it won't let you open the file again.

Instead you must pass it to [`crate::VolumeManager::close_file`] to close it
cleanly.

If you want your files to close themselves on drop, create your own File
type that wraps this one and also holds a `VolumeManager` reference. You'll
then also need to put your `VolumeManager` in some kind of Mutex or RefCell,
and deal with the fact you can't put them both in the same struct any more
because one refers to the other. Basically, it's complicated and there's a
reason we did it this way.

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> File`
- **PartialEq**
  - `fn eq(self: &Self, other: &File) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## embedded_fat::filesystem::files::FileError

*Enum*

Errors related to file operations

**Variants:**
- `InvalidOffset` - Tried to use an invalid offset.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &FileError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FileError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## embedded_fat::filesystem::files::Mode

*Enum*

The different ways we can open a file.

**Variants:**
- `ReadOnly` - Open a file for reading, if it exists.
- `ReadWriteAppend` - Open a file for appending (writing to the end of the existing file), if it exists.
- `ReadWriteTruncate` - Open a file and remove all contents, before writing to the start of the existing file, if it exists.
- `ReadWriteCreate` - Create a new empty file. Fail if it exists.
- `ReadWriteCreateOrTruncate` - Create a new empty file, or truncate an existing file.
- `ReadWriteCreateOrAppend` - Create a new empty file, or append to an existing file.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Mode`
- **PartialEq**
  - `fn eq(self: &Self, other: &Mode) -> bool`



