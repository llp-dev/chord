**allocator_api2 > stable > alloc > global**

# Module: stable::alloc::global

## Contents

**Structs**

- [`Global`](#global) - The global memory allocator.

---

## allocator_api2::stable::alloc::global::Global

*Struct*

The global memory allocator.

This type implements the [`Allocator`] trait by forwarding calls
to the allocator registered with the `#[global_allocator]` attribute
if there is one, or the `std` crate’s default.

Note: while this type is unstable, the functionality it provides can be
accessed through the [free functions in `alloc`](crate#functions).

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Allocator**
  - `fn allocate(self: &Self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>`
  - `fn allocate_zeroed(self: &Self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>`
  - `fn deallocate(self: &Self, ptr: NonNull<u8>, layout: Layout)`
  - `fn grow(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`
  - `fn grow_zeroed(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`
  - `fn shrink(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`
- **Default**
  - `fn default() -> Global`
- **Clone**
  - `fn clone(self: &Self) -> Global`



