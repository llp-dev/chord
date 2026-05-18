**sel4 > arch > imp > arch > imp > object**

# Module: arch::imp::arch::imp::object

## Contents

**Enums**

- [`ObjectBlueprintX64`](#objectblueprintx64)
- [`ObjectTypeX64`](#objecttypex64)

**Type Aliases**

- [`ObjectBlueprintSeL4Arch`](#objectblueprintsel4arch)
- [`ObjectTypeSeL4Arch`](#objecttypesel4arch)

---

## sel4::arch::imp::arch::imp::object::ObjectBlueprintSeL4Arch

*Type Alias*: `ObjectBlueprintX64`



## sel4::arch::imp::arch::imp::object::ObjectBlueprintX64

*Enum*

**Variants:**
- `HugePage`
- `PDPT`
- `PML4`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ObjectBlueprintX64) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &ObjectBlueprintX64) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectBlueprintX64) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ObjectBlueprintX64`



## sel4::arch::imp::arch::imp::object::ObjectTypeSeL4Arch

*Type Alias*: `ObjectTypeX64`



## sel4::arch::imp::arch::imp::object::ObjectTypeX64

*Enum*

**Variants:**
- `HugePage`
- `PDPT`
- `PML4`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ObjectTypeX64`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectTypeX64) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



