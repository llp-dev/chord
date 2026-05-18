**rkyv > ser > allocator > alloc**

# Module: ser::allocator::alloc

## Contents

**Structs**

- [`Arena`](#arena) - An arena allocator for allocations.
- [`ArenaHandle`](#arenahandle) - A handle which can allocate within an arena.

---

## rkyv::ser::allocator::alloc::Arena

*Struct*

An arena allocator for allocations.

Reusing the same arena for multiple serializations will reduce the number of
global allocations, which can save a considerable amount of time.

**Methods:**

- `fn new() -> Self` - Creates a new `Arena` with the default capacity.
- `fn with_capacity(cap: usize) -> Self` - Creates a new `Arena` with at least the requested capacity.
- `fn shrink(self: & mut Self) -> usize` - Cleans up allocated blocks which are no longer in use.
- `fn capacity(self: &Self) -> usize` - Returns the available capacity of the arena.
- `fn acquire(self: & mut Self) -> ArenaHandle` - Acquires a handle to the arena.
- `fn into_raw(self: Self) -> NonNull<()>` - Consumes the `Arena`, returning a raw pointer.
- `fn from_raw(raw: NonNull<()>) -> Self` - Constructs an arena from a raw pointer.

**Traits:** Send

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Default**
  - `fn default() -> Self`



## rkyv::ser::allocator::alloc::ArenaHandle

*Struct*

A handle which can allocate within an arena.

**Generic Parameters:**
- 'a

**Traits:** Send

**Trait Implementations:**

- **Allocator**
  - `fn push_alloc(self: & mut Self, layout: Layout) -> Result<NonNull<[u8]>, E>`
  - `fn pop_alloc(self: & mut Self, ptr: NonNull<u8>, _: Layout) -> Result<(), E>`



