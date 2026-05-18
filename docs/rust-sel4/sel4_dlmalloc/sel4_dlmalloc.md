**sel4_dlmalloc**

# Module: sel4_dlmalloc

## Contents

**Structs**

- [`BoundsAlreadySetError`](#boundsalreadyseterror)
- [`DeferredStaticDlmalloc`](#deferredstaticdlmalloc)
- [`StaticDlmalloc`](#staticdlmalloc)
- [`StaticHeap`](#staticheap)
- [`StaticHeapBounds`](#staticheapbounds)

---

## sel4_dlmalloc::BoundsAlreadySetError

*Struct*

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BoundsAlreadySetError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> BoundsAlreadySetError`



## sel4_dlmalloc::DeferredStaticDlmalloc

*Struct*

**Generic Parameters:**
- R

**Tuple Struct**: `()`

**Methods:**

- `fn new_with_raw_mutex(raw_mutex: R) -> Self`
- `fn raw_mutex(self: &Self) -> &R`
- `fn set_bounds(self: &Self, bounds: StaticHeapBounds) -> Result<(), BoundsAlreadySetError>`
- `fn new() -> Self`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **GlobalAlloc**
  - `fn alloc(self: &Self, layout: Layout) -> *mut u8`
  - `fn alloc_zeroed(self: &Self, layout: Layout) -> *mut u8`
  - `fn dealloc(self: &Self, ptr: *mut u8, layout: Layout)`
  - `fn realloc(self: &Self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8`



## sel4_dlmalloc::StaticDlmalloc

*Struct*

**Generic Parameters:**
- R

**Tuple Struct**: `()`

**Methods:**

- `fn raw_mutex(self: &Self) -> &R`
- `fn new(bounds: StaticHeapBounds) -> Self`
- `fn new_with_raw_mutex(raw_mutex: R, bounds: StaticHeapBounds) -> Self`

**Trait Implementations:**

- **GlobalAlloc**
  - `fn alloc(self: &Self, layout: Layout) -> *mut u8`
  - `fn alloc_zeroed(self: &Self, layout: Layout) -> *mut u8`
  - `fn dealloc(self: &Self, ptr: *mut u8, layout: Layout)`
  - `fn realloc(self: &Self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8`



## sel4_dlmalloc::StaticHeap

*Struct*

**Generic Parameters:**
- const N
- A

**Methods:**

- `fn new() -> Self`
- `fn bounds(self: &Self) -> StaticHeapBounds`

**Traits:** Sync

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## sel4_dlmalloc::StaticHeapBounds

*Struct*

**Methods:**

- `fn new(ptr: *mut u8, size: usize) -> Self`
- `fn start(self: &Self) -> *mut u8`
- `fn end(self: &Self) -> *mut u8`
- `fn size(self: &Self) -> usize`

**Traits:** Eq, Copy, Send

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> StaticHeapBounds`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &StaticHeapBounds) -> bool`



