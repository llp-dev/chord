**rkyv > ser > allocator**

# Module: ser::allocator

## Contents

**Structs**

- [`AllocationStats`](#allocationstats) - Statistics for the allocations which occurred during serialization.
- [`AllocationTracker`](#allocationtracker) - A passthrough allocator that tracks usage.

**Traits**

- [`Allocator`](#allocator) - A serializer that can allocate scratch space.

---

## rkyv::ser::allocator::AllocationStats

*Struct*

Statistics for the allocations which occurred during serialization.

**Fields:**
- `max_bytes_allocated: usize` - Returns the maximum number of bytes that were concurrently allocated.
- `max_allocations: usize` - Returns the maximum number of concurrent allocations.
- `max_alignment: usize` - Returns the maximum alignment of requested allocations.

**Methods:**

- `fn min_arena_capacity(self: &Self) -> usize` - Returns the minimum arena capacity required to serialize the same data.
- `fn min_arena_capacity_max_error(self: &Self) -> usize` - Returns the maximum error term for the minimum arena capacity

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rkyv::ser::allocator::AllocationTracker

*Struct*

A passthrough allocator that tracks usage.

**Generic Parameters:**
- T

**Methods:**

- `fn new(inner: T) -> Self` - Returns a new allocation tracker wrapping the given allocator.
- `fn into_stats(self: Self) -> AllocationStats` - Returns the allocation stats accumulated during serialization.

**Trait Implementations:**

- **From**
  - `fn from(inner: T) -> Self`
- **Allocator**
  - `fn push_alloc(self: & mut Self, layout: Layout) -> Result<NonNull<[u8]>, E>`
  - `fn pop_alloc(self: & mut Self, ptr: NonNull<u8>, layout: Layout) -> Result<(), E>`



## rkyv::ser::allocator::Allocator

*Trait*

A serializer that can allocate scratch space.

# Safety

`push_alloc` must return a pointer to unaliased memory which fits the
provided layout.

**Methods:**

- `push_alloc`: Allocates scratch space of the requested size.
- `pop_alloc`: Deallocates previously allocated scratch space.



