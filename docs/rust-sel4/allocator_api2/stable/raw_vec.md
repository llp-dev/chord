**allocator_api2 > stable > raw_vec**

# Module: stable::raw_vec

## Contents

**Structs**

- [`TryReserveError`](#tryreserveerror) - The error type for `try_reserve` methods.

**Enums**

- [`TryReserveErrorKind`](#tryreserveerrorkind) - Details of the allocation that caused a `TryReserveError`

---

## allocator_api2::stable::raw_vec::TryReserveError

*Struct*

The error type for `try_reserve` methods.

**Methods:**

- `fn kind(self: &Self) -> TryReserveErrorKind` - Details about the allocation that caused the error

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(kind: TryReserveErrorKind) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TryReserveError`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &TryReserveError) -> bool`



## allocator_api2::stable::raw_vec::TryReserveErrorKind

*Enum*

Details of the allocation that caused a `TryReserveError`

**Variants:**
- `CapacityOverflow` - Error due to the computed capacity exceeding the collection's maximum
- `AllocError{ layout: super::alloc::Layout }` - The memory allocator returned an error

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TryReserveErrorKind`
- **PartialEq**
  - `fn eq(self: &Self, other: &TryReserveErrorKind) -> bool`
- **From**
  - `fn from(_: LayoutError) -> Self` - Always evaluates to [`TryReserveErrorKind::CapacityOverflow`].
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



