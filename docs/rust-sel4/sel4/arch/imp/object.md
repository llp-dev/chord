**sel4 > arch > imp > object**

# Module: arch::imp::object

## Contents

**Enums**

- [`ObjectBlueprintX86`](#objectblueprintx86)
- [`ObjectTypeX86`](#objecttypex86)

**Type Aliases**

- [`ObjectBlueprintArch`](#objectblueprintarch)
- [`ObjectTypeArch`](#objecttypearch)

---

## sel4::arch::imp::object::ObjectBlueprintArch

*Type Alias*: `ObjectBlueprintX86`



## sel4::arch::imp::object::ObjectBlueprintX86

*Enum*

**Variants:**
- `_4k`
- `LargePage`
- `PageTable`
- `PageDirectory`
- `SeL4Arch(crate::ObjectBlueprintSeL4Arch)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &ObjectBlueprintX86) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectBlueprintX86) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ObjectBlueprintX86) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> ObjectBlueprintX86`



## sel4::arch::imp::object::ObjectTypeArch

*Type Alias*: `ObjectTypeX86`



## sel4::arch::imp::object::ObjectTypeX86

*Enum*

**Variants:**
- `_4k`
- `LargePage`
- `PageTable`
- `PageDirectory`
- `SeL4Arch(crate::ObjectTypeSeL4Arch)`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ObjectTypeX86`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectTypeX86) -> bool`



