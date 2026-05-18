**allocator_api2 > stable > alloc**

# Module: stable::alloc

## Contents

**Structs**

- [`AllocError`](#allocerror) - The `AllocError` error indicates an allocation failure

**Traits**

- [`Allocator`](#allocator) - An implementation of `Allocator` can allocate, grow, shrink, and deallocate arbitrary blocks of

---

## allocator_api2::stable::alloc::AllocError

*Struct*

The `AllocError` error indicates an allocation failure
that may be due to resource exhaustion or to
something wrong when combining the given input arguments with this
allocator.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &AllocError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AllocError`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## allocator_api2::stable::alloc::Allocator

*Trait*

An implementation of `Allocator` can allocate, grow, shrink, and deallocate arbitrary blocks of
data described via [`Layout`][].

`Allocator` is designed to be implemented on ZSTs, references, or smart pointers because having
an allocator like `MyAlloc([u8; N])` cannot be moved, without updating the pointers to the
allocated memory.

Unlike [`GlobalAlloc`][], zero-sized allocations are allowed in `Allocator`. If an underlying
allocator does not support this (like jemalloc) or return a null pointer (such as
`libc::malloc`), this must be caught by the implementation.

### Currently allocated memory

Some of the methods require that a memory block be *currently allocated* via an allocator. This
means that:

* the starting address for that memory block was previously returned by [`allocate`], [`grow`], or
  [`shrink`], and

* the memory block has not been subsequently deallocated, where blocks are either deallocated
  directly by being passed to [`deallocate`] or were changed by being passed to [`grow`] or
  [`shrink`] that returns `Ok`. If `grow` or `shrink` have returned `Err`, the passed pointer
  remains valid.

[`allocate`]: Allocator::allocate
[`grow`]: Allocator::grow
[`shrink`]: Allocator::shrink
[`deallocate`]: Allocator::deallocate

### Memory fitting

Some of the methods require that a layout *fit* a memory block. What it means for a layout to
"fit" a memory block means (or equivalently, for a memory block to "fit" a layout) is that the
following conditions must hold:

* The block must be allocated with the same alignment as [`layout.align()`], and

* The provided [`layout.size()`] must fall in the range `min ..= max`, where:
  - `min` is the size of the layout most recently used to allocate the block, and
  - `max` is the latest actual size returned from [`allocate`], [`grow`], or [`shrink`].

[`layout.align()`]: Layout::align
[`layout.size()`]: Layout::size

# Safety

* Memory blocks returned from an allocator must point to valid memory and retain their validity
  until the instance and all of its clones are dropped,

* cloning or moving the allocator must not invalidate memory blocks returned from this
  allocator. A cloned allocator must behave like the same allocator, and

* any pointer to a memory block which is [*currently allocated*] may be passed to any other
  method of the allocator.

[*currently allocated*]: #currently-allocated-memory

**Methods:**

- `allocate`: Attempts to allocate a block of memory.
- `allocate_zeroed`: Behaves like `allocate`, but also ensures that the returned memory is zero-initialized.
- `deallocate`: Deallocates the memory referenced by `ptr`.
- `grow`: Attempts to extend the memory block.
- `grow_zeroed`: Behaves like `grow`, but also ensures that the new contents are set to zero before being
- `shrink`: Attempts to shrink the memory block.
- `by_ref`: Creates a "by reference" adapter for this instance of `Allocator`.



