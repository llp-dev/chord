**dlmalloc**

# Module: dlmalloc

## Contents

**Structs**

- [`Dlmalloc`](#dlmalloc) - An allocator instance

**Traits**

- [`Allocator`](#allocator) - In order for this crate to efficiently manage memory, it needs a way to communicate with the

---

## dlmalloc::Allocator

*Trait*

In order for this crate to efficiently manage memory, it needs a way to communicate with the
underlying platform. This `Allocator` trait provides an interface for this communication.

**Methods:**

- `alloc`: Allocates system memory region of at least `size` bytes
- `remap`: Remaps system memory region at `ptr` with size `oldsize` to a potential new location with
- `free_part`: Frees a part of a memory chunk. The original memory chunk starts at `ptr` with size `oldsize`
- `free`: Frees an entire memory region. Returns `true` iff the operation succeeded. When `false` is
- `can_release_part`: Indicates if the system can release a part of memory. For the `flags` argument, see
- `allocates_zeros`: Indicates whether newly allocated regions contain zeros.
- `page_size`: Returns the page size. Must be a power of two



## dlmalloc::Dlmalloc

*Struct*

An allocator instance

Instances of this type are used to allocate blocks of memory. For best
results only use one of these. Currently doesn't implement `Drop` to release
lingering memory back to the OS. That may happen eventually though!

**Generic Parameters:**
- A

**Tuple Struct**: `()`

**Methods:**

- `fn new_with_allocator(sys_allocator: A) -> Dlmalloc<A>` - Creates a new instance of an allocator
- `fn malloc(self: & mut Self, size: usize, align: usize) -> *mut u8` - Allocates `size` bytes with `align` align.
- `fn calloc(self: & mut Self, size: usize, align: usize) -> *mut u8` - Same as `malloc`, except if the allocation succeeds it's guaranteed to
- `fn free(self: & mut Self, ptr: *mut u8, size: usize, align: usize)` - Deallocates a `ptr` with `size` and `align` as the previous request used
- `fn realloc(self: & mut Self, ptr: *mut u8, old_size: usize, old_align: usize, new_size: usize) -> *mut u8` - Reallocates `ptr`, a previous allocation with `old_size` and
- `fn trim(self: & mut Self, pad: usize) -> bool` - If possible, gives memory back to the system if there is unused memory
- `fn destroy(self: Self) -> usize` - Releases all allocations in this allocator back to the system,
- `fn allocator(self: &Self) -> &A` - Get a reference to the underlying [`Allocator`] that this `Dlmalloc` was
- `fn allocator_mut(self: & mut Self) -> & mut A` - Get a mutable reference to the underlying [`Allocator`] that this
- `fn new() -> Dlmalloc<System>` - Creates a new instance of an allocator



