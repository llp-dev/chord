**sel4_shared_ring_buffer > descriptor**

# Module: descriptor

## Contents

**Structs**

- [`Descriptor`](#descriptor)

---

## sel4_shared_ring_buffer::descriptor::Descriptor

*Struct*

**Methods:**

- `fn new(encoded_addr: usize, len: u32, cookie: usize) -> Self`
- `fn from_encoded_addr_range(encoded_addr_range: Range<usize>, cookie: usize) -> Self`
- `fn encoded_addr(self: &Self) -> usize`
- `fn set_encoded_addr(self: & mut Self, encoded_addr: usize)`
- `fn len(self: &Self) -> u32`
- `fn set_len(self: & mut Self, len: u32)`
- `fn cookie(self: &Self) -> usize`
- `fn set_cookie(self: & mut Self, cookie: usize)`
- `fn encoded_addr_range(self: &Self) -> Range<usize>`

**Traits:** Copy, Immutable, IntoBytes, FromBytes, FromZeros, TryFromBytes, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Descriptor) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Descriptor) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Descriptor`
- **PartialEq**
  - `fn eq(self: &Self, other: &Descriptor) -> bool`



