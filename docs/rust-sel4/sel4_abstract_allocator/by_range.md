**sel4_abstract_allocator > by_range**

# Module: by_range

## Contents

**Structs**

- [`ByRange`](#byrange)

---

## sel4_abstract_allocator::by_range::ByRange

*Struct*

**Generic Parameters:**
- A

**Methods:**

- `fn new(inner: A) -> Self`
- `fn allocate(self: & mut Self, layout: Layout) -> Result<Range<usize>, <A as >::AllocationError>`
- `fn deallocate(self: & mut Self, range: Range<usize>)`



