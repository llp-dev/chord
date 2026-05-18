**rkyv > ser > allocator > core**

# Module: ser::allocator::core

## Contents

**Structs**

- [`SubAllocator`](#suballocator) - An allocator that sub-allocates a fixed-size memory space.

---

## rkyv::ser::allocator::core::SubAllocator

*Struct*

An allocator that sub-allocates a fixed-size memory space.

**Generic Parameters:**
- 'a

**Methods:**

- `fn empty() -> Self` - Creates an empty suballocator.
- `fn new(bytes: &'a  mut [MaybeUninit<u8>]) -> Self` - Creates a new sub-allocator from the given byte slice.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Allocator**
  - `fn push_alloc(self: & mut Self, layout: Layout) -> Result<NonNull<[u8]>, E>`
  - `fn pop_alloc(self: & mut Self, ptr: NonNull<u8>, _: Layout) -> Result<(), E>`



