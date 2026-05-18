**sel4 > arch > imp > fault**

# Module: arch::imp::fault

## Contents

**Structs**

- [`CapFault`](#capfault) - Corresponds to `
- [`NullFault`](#nullfault) - Corresponds to `
- [`UnknownSyscall`](#unknownsyscall) - Corresponds to `
- [`UserException`](#userexception) - Corresponds to `
- [`VmFault`](#vmfault) - Corresponds to `

**Enums**

- [`Fault`](#fault)

---

## sel4::arch::imp::fault::CapFault

*Struct*

Corresponds to `
seL4_Fault_CapFault
`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: $crate::sys::seL4_Fault_CapFault) -> Self`
- `fn into_inner(self: Self) -> $crate::sys::seL4_Fault_CapFault`
- `fn inner(self: &Self) -> &$crate::sys::seL4_Fault_CapFault`
- `fn inner_mut(self: & mut Self) -> & mut $crate::sys::seL4_Fault_CapFault`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CapFault`
- **PartialEq**
  - `fn eq(self: &Self, other: &CapFault) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::arch::imp::fault::Fault

*Enum*

**Variants:**
- `NullFault(NullFault)`
- `CapFault(CapFault)`
- `UnknownSyscall(UnknownSyscall)`
- `UserException(UserException)`
- `VmFault(VmFault)`

**Methods:**

- `fn new(ipc_buffer: &IpcBuffer, info: &MessageInfo) -> Self`
- `fn from_sys(raw: sys::seL4_Fault) -> Self`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Fault`
- **PartialEq**
  - `fn eq(self: &Self, other: &Fault) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::arch::imp::fault::NullFault

*Struct*

Corresponds to `
seL4_Fault_NullFault
`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: $crate::sys::seL4_Fault_NullFault) -> Self`
- `fn into_inner(self: Self) -> $crate::sys::seL4_Fault_NullFault`
- `fn inner(self: &Self) -> &$crate::sys::seL4_Fault_NullFault`
- `fn inner_mut(self: & mut Self) -> & mut $crate::sys::seL4_Fault_NullFault`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NullFault`
- **PartialEq**
  - `fn eq(self: &Self, other: &NullFault) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::arch::imp::fault::UnknownSyscall

*Struct*

Corresponds to `
seL4_Fault_UnknownSyscall
`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: $crate::sys::seL4_Fault_UnknownSyscall) -> Self`
- `fn into_inner(self: Self) -> $crate::sys::seL4_Fault_UnknownSyscall`
- `fn inner(self: &Self) -> &$crate::sys::seL4_Fault_UnknownSyscall`
- `fn inner_mut(self: & mut Self) -> & mut $crate::sys::seL4_Fault_UnknownSyscall`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UnknownSyscall`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnknownSyscall) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::arch::imp::fault::UserException

*Struct*

Corresponds to `
seL4_Fault_UserException
`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: $crate::sys::seL4_Fault_UserException) -> Self`
- `fn into_inner(self: Self) -> $crate::sys::seL4_Fault_UserException`
- `fn inner(self: &Self) -> &$crate::sys::seL4_Fault_UserException`
- `fn inner_mut(self: & mut Self) -> & mut $crate::sys::seL4_Fault_UserException`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UserException`
- **PartialEq**
  - `fn eq(self: &Self, other: &UserException) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::arch::imp::fault::VmFault

*Struct*

Corresponds to `
seL4_Fault_VMFault
`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: $crate::sys::seL4_Fault_VMFault) -> Self`
- `fn into_inner(self: Self) -> $crate::sys::seL4_Fault_VMFault`
- `fn inner(self: &Self) -> &$crate::sys::seL4_Fault_VMFault`
- `fn inner_mut(self: & mut Self) -> & mut $crate::sys::seL4_Fault_VMFault`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VmFault`
- **PartialEq**
  - `fn eq(self: &Self, other: &VmFault) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



