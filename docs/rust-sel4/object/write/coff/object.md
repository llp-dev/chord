**object > write > coff > object**

# Module: write::coff::object

## Contents

**Enums**

- [`CoffExportStyle`](#coffexportstyle) - Internal format to use for the `.drectve` section containing linker

---

## object::write::coff::object::CoffExportStyle

*Enum*

Internal format to use for the `.drectve` section containing linker
directives for symbol exports.

**Variants:**
- `Msvc` - MSVC format supported by link.exe and LLD.
- `Gnu` - Gnu format supported by GNU LD and LLD.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CoffExportStyle`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CoffExportStyle) -> bool`



