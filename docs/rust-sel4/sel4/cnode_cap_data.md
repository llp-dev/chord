**sel4 > cnode_cap_data**

# Module: cnode_cap_data

## Contents

**Structs**

- [`CNodeCapData`](#cnodecapdata) - Corresponds to `seL4_CNode_CapData`.

---

## sel4::cnode_cap_data::CNodeCapData

*Struct*

Corresponds to `seL4_CNode_CapData`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: sys::seL4_CNode_CapData) -> Self`
- `fn into_inner(self: Self) -> sys::seL4_CNode_CapData`
- `fn inner(self: &Self) -> &sys::seL4_CNode_CapData`
- `fn inner_mut(self: & mut Self) -> & mut sys::seL4_CNode_CapData`
- `fn new(guard: Word, guard_size: usize) -> Self`
- `fn skip(num_bits: usize) -> Self`
- `fn skip_high_bits(cnode_size_bits: usize) -> Self`
- `fn into_word(self: Self) -> Word`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CNodeCapData`
- **PartialEq**
  - `fn eq(self: &Self, other: &CNodeCapData) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



