**sel4 > arch > imp > cap_type_arch**

# Module: arch::imp::cap_type_arch

## Contents

**Structs**

- [`HugePage`](#hugepage)
- [`IOPortControl`](#ioportcontrol)
- [`LargePage`](#largepage)
- [`PDPT`](#pdpt)
- [`PML4`](#pml4)
- [`PageDirectory`](#pagedirectory)
- [`PageTable`](#pagetable)
- [`_4k`](#_4k)

**Type Aliases**

- [`Granule`](#granule)
- [`VSpace`](#vspace)

---

## sel4::arch::imp::cap_type_arch::Granule

*Type Alias*: `_4k`



## sel4::arch::imp::cap_type_arch::HugePage

*Struct*

**Unit Struct**

**Traits:** CapTypeForFrameObjectOfFixedSize, Copy, Eq, CapTypeForFrameObject, CapType

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &HugePage) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> HugePage`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &HugePage) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &HugePage) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4::arch::imp::cap_type_arch::IOPortControl

*Struct*

**Unit Struct**

**Traits:** Eq, CapType, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IOPortControl) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IOPortControl) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> IOPortControl`
- **Ord**
  - `fn cmp(self: &Self, other: &IOPortControl) -> $crate::cmp::Ordering`



## sel4::arch::imp::cap_type_arch::LargePage

*Struct*

**Unit Struct**

**Traits:** CapTypeForFrameObjectOfFixedSize, Copy, Eq, CapTypeForFrameObject, CapType

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &LargePage) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> LargePage`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &LargePage) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &LargePage) -> bool`



## sel4::arch::imp::cap_type_arch::PDPT

*Struct*

**Unit Struct**

**Traits:** CapTypeForTranslationTableObject, Copy, Eq, CapType

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PDPT) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> PDPT`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &PDPT) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &PDPT) -> bool`



## sel4::arch::imp::cap_type_arch::PML4

*Struct*

**Unit Struct**

**Traits:** Copy, Eq, CapType, CapTypeForTranslationTableObject

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PML4`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &PML4) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &PML4) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PML4) -> $crate::option::Option<$crate::cmp::Ordering>`



## sel4::arch::imp::cap_type_arch::PageDirectory

*Struct*

**Unit Struct**

**Traits:** CapTypeForTranslationTableObject, Copy, Eq, CapType

**Trait Implementations:**

- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &PageDirectory) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &PageDirectory) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PageDirectory) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> PageDirectory`



## sel4::arch::imp::cap_type_arch::PageTable

*Struct*

**Unit Struct**

**Traits:** CapTypeForTranslationTableObject, Copy, Eq, CapType

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &PageTable) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> PageTable`
- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **Ord**
  - `fn cmp(self: &Self, other: &PageTable) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &PageTable) -> bool`



## sel4::arch::imp::cap_type_arch::VSpace

*Type Alias*: `PML4`



## sel4::arch::imp::cap_type_arch::_4k

*Struct*

**Unit Struct**

**Traits:** Copy, Eq, CapTypeForFrameObject, CapTypeForFrameObjectOfFixedSize, CapType

**Trait Implementations:**

- **CapTypeForObject**
  - `fn object_type() -> $crate::ObjectType`
- **CapTypeForObjectOfFixedSize**
  - `fn object_blueprint() -> $crate::ObjectBlueprint`
- **PartialEq**
  - `fn eq(self: &Self, other: &_4k) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &_4k) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> _4k`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &_4k) -> $crate::cmp::Ordering`



