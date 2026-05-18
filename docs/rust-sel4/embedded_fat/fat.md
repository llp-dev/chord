**embedded_fat > fat**

# Module: fat

## Contents

**Enums**

- [`FatType`](#fattype) - Indentifies the supported types of FAT format

**Constants**

- [`RESERVED_ENTRIES`](#reserved_entries) - Number of entries reserved at the start of a File Allocation Table

---

## embedded_fat::fat::FatType

*Enum*

Indentifies the supported types of FAT format

**Variants:**
- `Fat16` - FAT16 Format
- `Fat32` - FAT32 Format

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FatType`
- **PartialEq**
  - `fn eq(self: &Self, other: &FatType) -> bool`



## embedded_fat::fat::RESERVED_ENTRIES

*Constant*: `u32`

Number of entries reserved at the start of a File Allocation Table



