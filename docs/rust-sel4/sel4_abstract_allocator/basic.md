**sel4_abstract_allocator > basic**

# Module: basic

## Contents

**Structs**

- [`Allocation`](#allocation)
- [`BasicAllocator`](#basicallocator)
- [`InsufficientResources`](#insufficientresources)

---

## sel4_abstract_allocator::basic::Allocation

*Struct*

**Tuple Struct**: `()`

**Trait Implementations:**

- **AbstractAllocatorAllocation**
  - `fn range(self: &Self) -> Range<usize>`



## sel4_abstract_allocator::basic::BasicAllocator

*Struct*

**Methods:**

- `fn new(size: usize) -> Self`
- `fn with_granule_size(size: usize, granule_size: usize) -> Self`

**Trait Implementations:**

- **AbstractAllocator**
  - `fn allocate(self: & mut Self, orig_layout: Layout) -> Result<<Self as >::Allocation, <Self as >::AllocationError>`
  - `fn deallocate(self: & mut Self, allocation: <Self as >::Allocation)`



## sel4_abstract_allocator::basic::InsufficientResources

*Struct*

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &InsufficientResources) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &InsufficientResources) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &InsufficientResources) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> InsufficientResources`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



