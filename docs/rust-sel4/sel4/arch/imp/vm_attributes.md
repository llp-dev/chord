**sel4 > arch > imp > vm_attributes**

# Module: arch::imp::vm_attributes

## Contents

**Structs**

- [`VmAttributes`](#vmattributes) - Corresponds to `seL4_X86_VMAttributes`.

---

## sel4::arch::imp::vm_attributes::VmAttributes

*Struct*

Corresponds to `seL4_X86_VMAttributes`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: sys::seL4_X86_VMAttributes::Type) -> Self`
- `fn into_inner(self: Self) -> sys::seL4_X86_VMAttributes::Type`
- `fn inner(self: &Self) -> &sys::seL4_X86_VMAttributes::Type`
- `fn inner_mut(self: & mut Self) -> & mut sys::seL4_X86_VMAttributes::Type`
- `fn has(self: Self, rhs: Self) -> bool`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, rhs: Self)`
- **Clone**
  - `fn clone(self: &Self) -> VmAttributes`
- **PartialEq**
  - `fn eq(self: &Self, other: &VmAttributes) -> bool`
- **Default**
  - `fn default() -> Self`
- **Not**
  - `fn not(self: Self) -> Self`
- **BitAnd**
  - `fn bitand(self: Self, rhs: Self) -> Self`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, rhs: Self)`
- **BitOr**
  - `fn bitor(self: Self, rhs: Self) -> Self`



