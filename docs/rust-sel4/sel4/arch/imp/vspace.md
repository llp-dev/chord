**sel4 > arch > imp > vspace**

# Module: arch::imp::vspace

## Contents

**Enums**

- [`FrameObjectType`](#frameobjecttype) - Frame object types for this kernel configuration.
- [`TranslationTableObjectType`](#translationtableobjecttype) - Translation table object types for this kernel configuration.

---

## sel4::arch::imp::vspace::FrameObjectType

*Enum*

Frame object types for this kernel configuration.

**Variants:**
- `_4k`
- `LargePage`
- `HugePage`

**Methods:**

- `fn blueprint(self: Self) -> ObjectBlueprint`
- `fn from_bits(bits: usize) -> Option<Self>`
- `fn bits(self: Self) -> usize`
- `fn bytes(self: Self) -> usize`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FrameObjectType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> FrameObjectType`



## sel4::arch::imp::vspace::TranslationTableObjectType

*Enum*

Translation table object types for this kernel configuration.

**Variants:**
- `PML4`
- `PDPT`
- `PageDirectory`
- `PageTable`

**Methods:**

- `fn blueprint(self: &Self) -> ObjectBlueprint`
- `fn index_bits(self: &Self) -> usize`
- `fn from_level(level: usize) -> Option<Self>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &TranslationTableObjectType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> TranslationTableObjectType`



