**object > read > macho > fat**

# Module: read::macho::fat

## Contents

**Structs**

- [`MachOFatFile`](#machofatfile) - A Mach-O universal binary.

**Traits**

- [`FatArch`](#fatarch) - A trait for generic access to [`macho::FatArch32`] and [`macho::FatArch64`].

**Type Aliases**

- [`MachOFatFile32`](#machofatfile32) - A 32-bit Mach-O universal binary.
- [`MachOFatFile64`](#machofatfile64) - A 64-bit Mach-O universal binary.

---

## object::read::macho::fat::FatArch

*Trait*

A trait for generic access to [`macho::FatArch32`] and [`macho::FatArch64`].

**Methods:**

- `Word`
- `MAGIC`
- `cputype`
- `cpusubtype`
- `offset`
- `size`
- `align`
- `architecture`
- `file_range`
- `data`



## object::read::macho::fat::MachOFatFile

*Struct*

A Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`], and corresponds
to [`crate::FileKind::MachOFat32`] or [`crate::FileKind::MachOFat64`].

**Generic Parameters:**
- 'data
- Fat

**Methods:**

- `fn parse<R>(data: R) -> Result<Self>` - Attempt to parse the fat header and fat arches.
- `fn header(self: &Self) -> &'data macho::FatHeader` - Return the fat header
- `fn arches(self: &Self) -> &'data [Fat]` - Return the array of fat arches.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MachOFatFile<'data, Fat>`



## object::read::macho::fat::MachOFatFile32

*Type Alias*: `MachOFatFile<'data, macho::FatArch32>`

A 32-bit Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`], and corresponds
to [`crate::FileKind::MachOFat32`].



## object::read::macho::fat::MachOFatFile64

*Type Alias*: `MachOFatFile<'data, macho::FatArch64>`

A 64-bit Mach-O universal binary.

This is a file that starts with [`macho::FatHeader`], and corresponds
to [`crate::FileKind::MachOFat64`].



