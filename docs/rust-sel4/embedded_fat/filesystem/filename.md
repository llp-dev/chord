**embedded_fat > filesystem > filename**

# Module: filesystem::filename

## Contents

**Structs**

- [`ShortFileName`](#shortfilename) - An MS-DOS 8.3 filename. 7-bit ASCII only. All lower-case is converted to

**Enums**

- [`FilenameError`](#filenameerror) - Various filename related errors that can occur.

**Traits**

- [`ToShortFileName`](#toshortfilename) - Describes things we can convert to short 8.3 filenames

---

## embedded_fat::filesystem::filename::FilenameError

*Enum*

Various filename related errors that can occur.

**Variants:**
- `InvalidCharacter` - Tried to create a file with an invalid character.
- `FilenameEmpty` - Tried to create a file with no file name.
- `NameTooLong` - Given name was too long (we are limited to 8.3).
- `MisplacedPeriod` - Can't start a file with a period, or after 8 characters.
- `Utf8Error` - Can't extract utf8 from file name

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FilenameError`



## embedded_fat::filesystem::filename::ShortFileName

*Struct*

An MS-DOS 8.3 filename. 7-bit ASCII only. All lower-case is converted to
upper-case by default.

**Methods:**

- `fn parent_dir() -> Self` - Get a short file name containing "..", which means "parent directory".
- `fn this_dir() -> Self` - Get a short file name containing "..", which means "this directory".
- `fn base_name(self: &Self) -> &[u8]` - Get base name (name without extension) of file name
- `fn extension(self: &Self) -> &[u8]` - Get base name (name without extension) of file name
- `fn create_from_str(name: &str) -> Result<ShortFileName, FilenameError>` - Create a new MS-DOS 8.3 space-padded file name as stored in the directory entry.
- `fn create_from_str_mixed_case(name: &str) -> Result<ShortFileName, FilenameError>` - Create a new MS-DOS 8.3 space-padded file name as stored in the directory entry.
- `fn lfn_checksum(self: &Self) -> u8`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ShortFileName) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ShortFileName`
- **ToShortFileName**
  - `fn to_short_filename(self: Self) -> Result<ShortFileName, FilenameError>`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## embedded_fat::filesystem::filename::ToShortFileName

*Trait*

Describes things we can convert to short 8.3 filenames

**Methods:**

- `to_short_filename`: Try and convert this value into a [`ShortFileName`].



