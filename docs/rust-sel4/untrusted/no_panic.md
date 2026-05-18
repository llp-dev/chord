**untrusted > no_panic**

# Module: no_panic

## Contents

**Structs**

- [`Slice`](#slice) - A wrapper around a slice that exposes no functions that can panic.

---

## untrusted::no_panic::Slice

*Struct*

A wrapper around a slice that exposes no functions that can panic.

Intentionally avoids implementing `Debug`, `Eq`, and `PartialEq` to avoid
creating a side channel that would leak information about the value.

**Generic Parameters:**
- 'a

**Fields:**
- `bytes: &'a [u8]`

**Methods:**

- `fn new(bytes: &'a [u8]) -> Self`
- `fn get(self: &Self, i: usize) -> Option<&u8>`
- `fn subslice(self: &Self, r: core::ops::Range<usize>) -> Option<Self>`
- `fn is_empty(self: &Self) -> bool`
- `fn len(self: &Self) -> usize`
- `fn as_slice_less_safe(self: &Self) -> &'a [u8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Slice<'a>`



