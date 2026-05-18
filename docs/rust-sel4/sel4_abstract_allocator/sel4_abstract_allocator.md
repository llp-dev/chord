**sel4_abstract_allocator**

# Module: sel4_abstract_allocator

## Contents

**Modules**

- [`basic`](#basic)
- [`bump`](#bump)

**Structs**

- [`WithAlignmentBound`](#withalignmentbound)

**Enums**

- [`WithAlignmentBoundAllocationError`](#withalignmentboundallocationerror)

**Traits**

- [`AbstractAllocator`](#abstractallocator)
- [`AbstractAllocatorAllocation`](#abstractallocatorallocation)

---

## sel4_abstract_allocator::AbstractAllocator

*Trait*

**Methods:**

- `AllocationError`
- `Allocation`
- `allocate`
- `deallocate`



## sel4_abstract_allocator::AbstractAllocatorAllocation

*Trait*

**Methods:**

- `range`



## sel4_abstract_allocator::WithAlignmentBound

*Struct*

**Generic Parameters:**
- A

**Methods:**

- `fn new(inner: A, max_alignment: usize) -> Self`
- `fn max_alignment(self: &Self) -> usize`
- `fn is_suitably_aligned(self: &Self, region: *mut u8) -> bool`

**Trait Implementations:**

- **AbstractAllocator**
  - `fn allocate(self: & mut Self, layout: Layout) -> Result<<Self as >::Allocation, <Self as >::AllocationError>`
  - `fn deallocate(self: & mut Self, allocation: <Self as >::Allocation)`



## sel4_abstract_allocator::WithAlignmentBoundAllocationError

*Enum*

**Generic Parameters:**
- E

**Variants:**
- `InnerError(E)`
- `AlignmentExceedsBound`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &WithAlignmentBoundAllocationError<E>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &WithAlignmentBoundAllocationError<E>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &WithAlignmentBoundAllocationError<E>) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> WithAlignmentBoundAllocationError<E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: basic



## Module: bump



