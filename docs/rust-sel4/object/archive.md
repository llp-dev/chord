**object > archive**

# Module: archive

## Contents

**Structs**

- [`AixFileHeader`](#aixfileheader) - The AIX big archive's fixed length header at file beginning.
- [`AixHeader`](#aixheader) - The header at the start of an AIX big archive member, without name.
- [`AixMemberOffset`](#aixmemberoffset) - Offset of a member in an AIX big archive.
- [`Header`](#header) - The header at the start of an archive member.

**Constants**

- [`AIX_BIG_MAGIC`](#aix_big_magic) - File identification bytes at the beginning of AIX big archive.
- [`MAGIC`](#magic) - File identification bytes stored at the beginning of the file.
- [`TERMINATOR`](#terminator) - The terminator for each archive member header.
- [`THIN_MAGIC`](#thin_magic) - File identification bytes stored at the beginning of a thin archive.

---

## object::archive::AIX_BIG_MAGIC

*Constant*: `[u8; 8]`

File identification bytes at the beginning of AIX big archive.



## object::archive::AixFileHeader

*Struct*

The AIX big archive's fixed length header at file beginning.

**Fields:**
- `magic: [u8; 8]` - Archive magic string.
- `memoff: [u8; 20]` - Offset of member table.
- `gstoff: [u8; 20]` - Offset of global symbol table.
- `gst64off: [u8; 20]` - Offset of global symbol table for 64-bit objects.
- `fstmoff: [u8; 20]` - Offset of first member.
- `lstmoff: [u8; 20]` - Offset of last member.
- `freeoff: [u8; 20]` - Offset of first member on free list.

**Traits:** Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AixFileHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::archive::AixHeader

*Struct*

The header at the start of an AIX big archive member, without name.

**Fields:**
- `size: [u8; 20]` - File member size in decimal.
- `nxtmem: [u8; 20]` - Next member offset in decimal.
- `prvmem: [u8; 20]` - Previous member offset in decimal.
- `date: [u8; 12]` - File member date in decimal.
- `uid: [u8; 12]` - File member user id in decimal.
- `gid: [u8; 12]` - File member group id in decimal.
- `mode: [u8; 12]` - File member mode in octal.
- `namlen: [u8; 4]` - File member name length in decimal.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AixHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::archive::AixMemberOffset

*Struct*

Offset of a member in an AIX big archive.

This is used in the member index.

**Tuple Struct**: `([u8; 20])`

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AixMemberOffset`



## object::archive::Header

*Struct*

The header at the start of an archive member.

**Fields:**
- `name: [u8; 16]` - The file name.
- `date: [u8; 12]` - File modification timestamp in decimal.
- `uid: [u8; 6]` - User ID in decimal.
- `gid: [u8; 6]` - Group ID in decimal.
- `mode: [u8; 8]` - File mode in octal.
- `size: [u8; 10]` - File size in decimal.
- `terminator: [u8; 2]` - Must be equal to `TERMINATOR`.

**Traits:** Pod, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Header`



## object::archive::MAGIC

*Constant*: `[u8; 8]`

File identification bytes stored at the beginning of the file.



## object::archive::TERMINATOR

*Constant*: `[u8; 2]`

The terminator for each archive member header.



## object::archive::THIN_MAGIC

*Constant*: `[u8; 8]`

File identification bytes stored at the beginning of a thin archive.

A thin archive only contains a symbol table and file names.



