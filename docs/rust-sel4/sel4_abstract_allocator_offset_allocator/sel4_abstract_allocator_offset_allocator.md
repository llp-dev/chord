**sel4_abstract_allocator_offset_allocator**

# Module: sel4_abstract_allocator_offset_allocator

## Contents

**Structs**

- [`Allocation`](#allocation)
- [`InsufficientResources`](#insufficientresources)
- [`OffsetAllocator`](#offsetallocator)

---

## sel4_abstract_allocator_offset_allocator::Allocation

*Struct*

**Generic Parameters:**
- NI

**Trait Implementations:**

- **AbstractAllocatorAllocation**
  - `fn range(self: &Self) -> Range<usize>`



## sel4_abstract_allocator_offset_allocator::InsufficientResources

*Struct*

**Tuple Struct**: `()`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &InsufficientResources) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &InsufficientResources) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &InsufficientResources) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> InsufficientResources`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## sel4_abstract_allocator_offset_allocator::OffsetAllocator

*Struct*

**Generic Parameters:**
- NI

**Methods:**

- `fn new(size: usize) -> Self`
- `fn with_max_allocs(size: usize, max_allocs: u32) -> Self`

**Trait Implementations:**

- **AbstractAllocator**
  - `fn allocate(self: & mut Self, layout: Layout) -> Result<<Self as >::Allocation, <Self as >::AllocationError>`
  - `fn deallocate(self: & mut Self, allocation: <Self as >::Allocation)`



