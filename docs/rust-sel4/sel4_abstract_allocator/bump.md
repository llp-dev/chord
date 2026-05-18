**sel4_abstract_allocator > bump**

# Module: bump

## Contents

**Structs**

- [`Allocation`](#allocation)
- [`BumpAllocator`](#bumpallocator)
- [`InsufficientResources`](#insufficientresources)

---

## sel4_abstract_allocator::bump::Allocation

*Struct*

**Tuple Struct**: `()`

**Trait Implementations:**

- **AbstractAllocatorAllocation**
  - `fn range(self: &Self) -> Range<usize>`



## sel4_abstract_allocator::bump::BumpAllocator

*Struct*

**Methods:**

- `fn new(size: usize) -> Self`

**Trait Implementations:**

- **AbstractAllocator**
  - `fn allocate(self: & mut Self, layout: Layout) -> Result<<Self as >::Allocation, <Self as >::AllocationError>`
  - `fn deallocate(self: & mut Self, _allocation: <Self as >::Allocation)`



## sel4_abstract_allocator::bump::InsufficientResources

*Struct*

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
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



