# Crate `sel4_dlmalloc`

## Contents

- [Structs](#structs)
  - [`StaticDlmalloc`](#staticdlmalloc)
  - [`DeferredStaticDlmalloc`](#deferredstaticdlmalloc)
  - [`StaticDlmallocAllocator`](#staticdlmallocallocator)
  - [`DeferredStaticDlmallocAllocator`](#deferredstaticdlmallocallocator)
  - [`BoundsAlreadySetError`](#boundsalreadyseterror)
  - [`SyncDlmalloc`](#syncdlmalloc)
  - [`SimpleDlmallocAllocatorWrapper`](#simpledlmallocallocatorwrapper)
  - [`StaticHeapBounds`](#staticheapbounds)
  - [`StaticHeap`](#staticheap)
- [Traits](#traits)
  - [`SimpleDlmallocAllocator`](#simpledlmallocallocator)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StaticDlmalloc`](#staticdlmalloc) | struct |  |
| [`DeferredStaticDlmalloc`](#deferredstaticdlmalloc) | struct |  |
| [`StaticDlmallocAllocator`](#staticdlmallocallocator) | struct |  |
| [`DeferredStaticDlmallocAllocator`](#deferredstaticdlmallocallocator) | struct |  |
| [`BoundsAlreadySetError`](#boundsalreadyseterror) | struct |  |
| [`SyncDlmalloc`](#syncdlmalloc) | struct |  |
| [`SimpleDlmallocAllocatorWrapper`](#simpledlmallocallocatorwrapper) | struct |  |
| [`StaticHeapBounds`](#staticheapbounds) | struct |  |
| [`StaticHeap`](#staticheap) | struct |  |
| [`SimpleDlmallocAllocator`](#simpledlmallocallocator) | trait |  |

## Structs

### `StaticDlmalloc<R>`

```rust
struct StaticDlmalloc<R>(SyncDlmalloc<R, SimpleDlmallocAllocatorWrapper<StaticDlmallocAllocator>>);
```

#### Implementations

- <span id="staticdlmalloc-new-with-raw-mutex"></span>`const fn new_with_raw_mutex(raw_mutex: R, bounds: StaticHeapBounds) -> Self` — [`StaticHeapBounds`](#staticheapbounds)

#### Trait Implementations

##### `impl<R: RawMutex> GlobalAlloc for StaticDlmalloc<R>`

- <span id="staticdlmalloc-globalalloc-alloc"></span>`unsafe fn alloc(&self, layout: Layout) -> *mut u8`

- <span id="staticdlmalloc-globalalloc-alloc-zeroed"></span>`unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8`

- <span id="staticdlmalloc-globalalloc-dealloc"></span>`unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)`

- <span id="staticdlmalloc-globalalloc-realloc"></span>`unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8`

### `DeferredStaticDlmalloc<R>`

```rust
struct DeferredStaticDlmalloc<R>(SyncDlmalloc<R, SimpleDlmallocAllocatorWrapper<DeferredStaticDlmallocAllocator<StaticDlmallocAllocator>>>);
```

#### Implementations

- <span id="deferredstaticdlmalloc-new-with-raw-mutex"></span>`const fn new_with_raw_mutex(raw_mutex: R) -> Self`

#### Trait Implementations

##### `impl<R: RawMutex> Default for DeferredStaticDlmalloc<R>`

- <span id="deferredstaticdlmalloc-default"></span>`fn default() -> Self`

##### `impl<R: RawMutex> GlobalAlloc for DeferredStaticDlmalloc<R>`

- <span id="deferredstaticdlmalloc-globalalloc-alloc"></span>`unsafe fn alloc(&self, layout: Layout) -> *mut u8`

- <span id="deferredstaticdlmalloc-globalalloc-alloc-zeroed"></span>`unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8`

- <span id="deferredstaticdlmalloc-globalalloc-dealloc"></span>`unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)`

- <span id="deferredstaticdlmalloc-globalalloc-realloc"></span>`unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8`

### `StaticDlmallocAllocator`

```rust
struct StaticDlmallocAllocator {
    bounds: StaticHeapBounds,
    watermark: core::sync::atomic::AtomicUsize,
}
```

#### Implementations

- <span id="staticdlmallocallocator-new"></span>`const fn new(bounds: StaticHeapBounds) -> Self` — [`StaticHeapBounds`](#staticheapbounds)

#### Trait Implementations

##### `impl SimpleDlmallocAllocator for StaticDlmallocAllocator`

- <span id="staticdlmallocallocator-simpledlmallocallocator-alloc-simple"></span>`fn alloc_simple(&self, size: usize) -> Option<*mut u8>`

### `DeferredStaticDlmallocAllocator<T>`

```rust
struct DeferredStaticDlmallocAllocator<T> {
    state: Option<T>,
}
```

#### Implementations

- <span id="deferredstaticdlmallocallocator-new"></span>`const fn new() -> Self`

- <span id="deferredstaticdlmallocallocator-set"></span>`fn set(&mut self, state: T) -> Result<(), BoundsAlreadySetError>` — [`BoundsAlreadySetError`](#boundsalreadyseterror)

#### Trait Implementations

##### `impl<T: SimpleDlmallocAllocator> SimpleDlmallocAllocator for DeferredStaticDlmallocAllocator<T>`

- <span id="deferredstaticdlmallocallocator-simpledlmallocallocator-alloc-simple"></span>`fn alloc_simple(&self, size: usize) -> Option<*mut u8>`

### `BoundsAlreadySetError`

```rust
struct BoundsAlreadySetError(());
```

#### Trait Implementations

##### `impl Clone for BoundsAlreadySetError`

- <span id="boundsalreadyseterror-clone"></span>`fn clone(&self) -> BoundsAlreadySetError` — [`BoundsAlreadySetError`](#boundsalreadyseterror)

##### `impl Copy for BoundsAlreadySetError`

##### `impl Debug for BoundsAlreadySetError`

- <span id="boundsalreadyseterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BoundsAlreadySetError`

##### `impl PartialEq for BoundsAlreadySetError`

- <span id="boundsalreadyseterror-partialeq-eq"></span>`fn eq(&self, other: &BoundsAlreadySetError) -> bool` — [`BoundsAlreadySetError`](#boundsalreadyseterror)

##### `impl StructuralPartialEq for BoundsAlreadySetError`

### `SyncDlmalloc<R, T>`

```rust
struct SyncDlmalloc<R, T> {
    dlmalloc: lock_api::Mutex<R, dlmalloc::Dlmalloc<T>>,
}
```

#### Implementations

- <span id="syncdlmalloc-new"></span>`const fn new(raw_mutex: R, state: T) -> Self`

#### Trait Implementations

##### `impl<R: RawMutex, T: Allocator> GlobalAlloc for SyncDlmalloc<R, T>`

- <span id="syncdlmalloc-globalalloc-alloc"></span>`unsafe fn alloc(&self, layout: Layout) -> *mut u8`

- <span id="syncdlmalloc-globalalloc-alloc-zeroed"></span>`unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8`

- <span id="syncdlmalloc-globalalloc-dealloc"></span>`unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)`

- <span id="syncdlmalloc-globalalloc-realloc"></span>`unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8`

### `SimpleDlmallocAllocatorWrapper<T>`

```rust
struct SimpleDlmallocAllocatorWrapper<T>(T);
```

#### Implementations

- <span id="simpledlmallocallocatorwrapper-new"></span>`const fn new(inner: T) -> Self`

#### Trait Implementations

##### `impl<T: SimpleDlmallocAllocator> Allocator for SimpleDlmallocAllocatorWrapper<T>`

- <span id="simpledlmallocallocatorwrapper-allocator-alloc"></span>`fn alloc(&self, size: usize) -> (*mut u8, usize, u32)`

- <span id="simpledlmallocallocatorwrapper-allocator-remap"></span>`fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8`

- <span id="simpledlmallocallocatorwrapper-allocator-free-part"></span>`fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool`

- <span id="simpledlmallocallocatorwrapper-allocator-free"></span>`fn free(&self, _ptr: *mut u8, _size: usize) -> bool`

- <span id="simpledlmallocallocatorwrapper-allocator-can-release-part"></span>`fn can_release_part(&self, _flags: u32) -> bool`

- <span id="simpledlmallocallocatorwrapper-allocator-allocates-zeros"></span>`fn allocates_zeros(&self) -> bool`

- <span id="simpledlmallocallocatorwrapper-allocator-page-size"></span>`fn page_size(&self) -> usize`

### `StaticHeapBounds`

```rust
struct StaticHeapBounds {
    ptr: *mut u8,
    size: usize,
}
```

#### Implementations

- <span id="staticheapbounds-new"></span>`const fn new(ptr: *mut u8, size: usize) -> Self`

- <span id="staticheapbounds-start"></span>`const fn start(&self) -> *mut u8`

- <span id="staticheapbounds-end"></span>`fn end(&self) -> *mut u8`

- <span id="staticheapbounds-size"></span>`const fn size(&self) -> usize`

#### Trait Implementations

##### `impl Clone for StaticHeapBounds`

- <span id="staticheapbounds-clone"></span>`fn clone(&self) -> StaticHeapBounds` — [`StaticHeapBounds`](#staticheapbounds)

##### `impl Copy for StaticHeapBounds`

##### `impl Debug for StaticHeapBounds`

- <span id="staticheapbounds-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StaticHeapBounds`

##### `impl PartialEq for StaticHeapBounds`

- <span id="staticheapbounds-partialeq-eq"></span>`fn eq(&self, other: &StaticHeapBounds) -> bool` — [`StaticHeapBounds`](#staticheapbounds)

##### `impl Send for StaticHeapBounds`

##### `impl StructuralPartialEq for StaticHeapBounds`

### `StaticHeap<const N: usize, A>`

```rust
struct StaticHeap<const N: usize, A> {
    _alignment: [A; 0],
    space: core::cell::UnsafeCell<[u8; N]>,
}
```

#### Implementations

- <span id="staticheap-new"></span>`const fn new() -> Self`

- <span id="staticheap-bounds"></span>`const fn bounds(&self) -> StaticHeapBounds` — [`StaticHeapBounds`](#staticheapbounds)

#### Trait Implementations

##### `impl<A> Default for StaticHeap<N, A>`

- <span id="staticheap-default"></span>`fn default() -> Self`

##### `impl<A> Sync for StaticHeap<N, A>`

## Traits

### `SimpleDlmallocAllocator`

```rust
trait SimpleDlmallocAllocator: Send { ... }
```

#### Required Methods

- `fn alloc_simple(&self, size: usize) -> Option<*mut u8>`

#### Implementors

- [`DeferredStaticDlmallocAllocator`](#deferredstaticdlmallocallocator)
- [`StaticDlmallocAllocator`](#staticdlmallocallocator)

