**sel4 > bootinfo**

# Module: bootinfo

## Contents

**Structs**

- [`BootInfo`](#bootinfo) - Corresponds to `seL4_BootInfo`.
- [`BootInfoExtra`](#bootinfoextra) - An extra bootinfo chunk along with its ID.
- [`BootInfoExtraIter`](#bootinfoextraiter) - An iterator for accessing the [`BootInfoExtra`] entires associated with a [`BootInfoPtr`].
- [`BootInfoPtr`](#bootinfoptr) - A wrapped pointer to a [`BootInfo`] block.
- [`UntypedDesc`](#untypeddesc) - Corresponds to `seL4_UntypedDesc`.

**Enums**

- [`BootInfoExtraId`](#bootinfoextraid) - Corresponds to `seL4_BootInfoID`.

---

## sel4::bootinfo::BootInfo

*Struct*

Corresponds to `seL4_BootInfo`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: sys::seL4_BootInfo) -> Self`
- `fn into_inner(self: Self) -> sys::seL4_BootInfo`
- `fn inner(self: &Self) -> &sys::seL4_BootInfo`
- `fn inner_mut(self: & mut Self) -> & mut sys::seL4_BootInfo`
- `fn ipc_buffer(self: &Self) -> *mut IpcBuffer`
- `fn empty(self: &Self) -> SlotRegion<cap_type::Null>`
- `fn user_image_frames(self: &Self) -> SlotRegion<cap_type::Granule>`
- `fn untyped(self: &Self) -> SlotRegion<cap_type::Untyped>`
- `fn untyped_list(self: &Self) -> &[UntypedDesc]`
- `fn device_untyped_range(self: &Self) -> Range<usize>`
- `fn kernel_untyped_range(self: &Self) -> Range<usize>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::bootinfo::BootInfoExtra

*Struct*

An extra bootinfo chunk along with its ID.

**Generic Parameters:**
- 'a

**Fields:**
- `id: BootInfoExtraId`
- `content_with_header: &'a [u8]`

**Methods:**

- `fn content_with_header(self: &Self) -> &[u8]`
- `fn content(self: &Self) -> &[u8]`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &BootInfoExtra<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BootInfoExtra<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::bootinfo::BootInfoExtraId

*Enum*

Corresponds to `seL4_BootInfoID`.

**Variants:**
- `Padding`
- `Fdt`

**Methods:**

- `fn from_sys(id: sys::seL4_BootInfoID::Type) -> Option<Self>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BootInfoExtraId) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BootInfoExtraId`



## sel4::bootinfo::BootInfoExtraIter

*Struct*

An iterator for accessing the [`BootInfoExtra`] entires associated with a [`BootInfoPtr`].

**Generic Parameters:**
- 'a

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## sel4::bootinfo::BootInfoPtr

*Struct*

A wrapped pointer to a [`BootInfo`] block.

Access [`BootInfo`] via `Deref`, and [`BootInfoExtraIter`] via [`extra`](BootInfoPtr::extra).

**Methods:**

- `fn new(ptr: *const BootInfo) -> Self`
- `fn ptr(self: &Self) -> *const BootInfo`
- `fn extra(self: &Self) -> BootInfoExtraIter`
- `fn footprint_size(self: &Self) -> usize`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`



## sel4::bootinfo::UntypedDesc

*Struct*

Corresponds to `seL4_UntypedDesc`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: sys::seL4_UntypedDesc) -> Self`
- `fn into_inner(self: Self) -> sys::seL4_UntypedDesc`
- `fn inner(self: &Self) -> &sys::seL4_UntypedDesc`
- `fn inner_mut(self: & mut Self) -> & mut sys::seL4_UntypedDesc`
- `fn paddr(self: &Self) -> usize`
- `fn size_bits(self: &Self) -> usize`
- `fn is_device(self: &Self) -> bool`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> UntypedDesc`
- **PartialEq**
  - `fn eq(self: &Self, other: &UntypedDesc) -> bool`



