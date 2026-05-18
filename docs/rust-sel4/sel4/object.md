**sel4 > object**

# Module: object

## Contents

**Enums**

- [`ObjectBlueprint`](#objectblueprint) - An object description for [`Untyped::untyped_retype`](crate::cap::Untyped::untyped_retype).
- [`ObjectType`](#objecttype) - Corresponds to `seL4_ObjectType`.

**Traits**

- [`CapTypeForObject`](#captypeforobject) - Trait for [`CapType`]s which correspond to kernel objects.
- [`CapTypeForObjectOfFixedSize`](#captypeforobjectoffixedsize) - Trait for [`CapType`]s which correspond to kernel objects of fixed size.
- [`CapTypeForObjectOfVariableSize`](#captypeforobjectofvariablesize) - Trait for [`CapType`]s which correspond to kernel objects of variable size.

---

## sel4::object::CapTypeForObject

*Trait*

Trait for [`CapType`]s which correspond to kernel objects.

**Methods:**

- `object_type`



## sel4::object::CapTypeForObjectOfFixedSize

*Trait*

Trait for [`CapType`]s which correspond to kernel objects of fixed size.

**Methods:**

- `object_blueprint`



## sel4::object::CapTypeForObjectOfVariableSize

*Trait*

Trait for [`CapType`]s which correspond to kernel objects of variable size.

**Methods:**

- `object_blueprint`



## sel4::object::ObjectBlueprint

*Enum*

An object description for [`Untyped::untyped_retype`](crate::cap::Untyped::untyped_retype).

**Variants:**
- `Untyped{ size_bits: usize }`
- `Endpoint`
- `Notification`
- `CNode{ size_bits: usize }`
- `Tcb`
- `Arch(crate::ObjectBlueprintArch)`

**Methods:**

- `fn ty(self: Self) -> ObjectType`
- `fn api_size_bits(self: Self) -> Option<usize>`
- `fn physical_size_bits(self: Self) -> usize`
- `fn asid_pool() -> Self`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &ObjectBlueprint) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectBlueprint) -> bool`
- **From**
  - `fn from(ty: ObjectBlueprintSeL4Arch) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ObjectBlueprint) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> ObjectBlueprint`
- **From**
  - `fn from(blueprint: ObjectBlueprintArch) -> Self`



## sel4::object::ObjectType

*Enum*

Corresponds to `seL4_ObjectType`.

**Variants:**
- `Untyped`
- `Endpoint`
- `Notification`
- `CNode`
- `Tcb`
- `Arch(crate::ObjectTypeArch)`

**Methods:**

- `fn into_sys(self: Self) -> c_uint`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ObjectType`
- **From**
  - `fn from(ty: ObjectTypeArch) -> Self`
- **From**
  - `fn from(ty: ObjectTypeSeL4Arch) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



