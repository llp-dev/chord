# Crate `sel4_initialize_tls`

## Contents

- [Modules](#modules)
  - [`set_thread_pointer`](#set-thread-pointer)
  - [`static_allocation`](#static-allocation)
  - [`on_stack`](#on-stack)
  - [`on_heap`](#on-heap)
- [Structs](#structs)
  - [`UncheckedTlsImage`](#uncheckedtlsimage)
  - [`InvalidTlsImageError`](#invalidtlsimageerror)
  - [`TlsImage`](#tlsimage)
  - [`RegionLayoutError`](#regionlayouterror)
  - [`TlsReservationLayout`](#tlsreservationlayout)
  - [`Region`](#region)
  - [`TrimmedRegion`](#trimmedregion)
  - [`TrimRegionError`](#trimregionerror)
  - [`StaticTlsAllocation`](#statictlsallocation)
  - [`HeapTlsReservation`](#heaptlsreservation)
- [Type Aliases](#type-aliases)
  - [`SetThreadPointerFn`](#setthreadpointerfn)
- [Constants](#constants)
  - [`DEFAULT_SET_THREAD_POINTER_FN`](#default-set-thread-pointer-fn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`set_thread_pointer`](#set-thread-pointer) | mod |  |
| [`static_allocation`](#static-allocation) | mod |  |
| [`on_stack`](#on-stack) | mod |  |
| [`on_heap`](#on-heap) | mod |  |
| [`UncheckedTlsImage`](#uncheckedtlsimage) | struct |  |
| [`InvalidTlsImageError`](#invalidtlsimageerror) | struct |  |
| [`TlsImage`](#tlsimage) | struct |  |
| [`RegionLayoutError`](#regionlayouterror) | struct |  |
| [`TlsReservationLayout`](#tlsreservationlayout) | struct |  |
| [`Region`](#region) | struct |  |
| [`TrimmedRegion`](#trimmedregion) | struct |  |
| [`TrimRegionError`](#trimregionerror) | struct |  |
| [`StaticTlsAllocation`](#statictlsallocation) | struct |  |
| [`HeapTlsReservation`](#heaptlsreservation) | struct |  |
| [`SetThreadPointerFn`](#setthreadpointerfn) | type |  |
| [`DEFAULT_SET_THREAD_POINTER_FN`](#default-set-thread-pointer-fn) | const |  |

## Modules

- [`set_thread_pointer`](set_thread_pointer/index.md)
- [`static_allocation`](static_allocation/index.md)
- [`on_stack`](on_stack/index.md)
- [`on_heap`](on_heap/index.md)

## Structs

### `UncheckedTlsImage`

```rust
struct UncheckedTlsImage {
    pub vaddr: usize,
    pub filesz: usize,
    pub memsz: usize,
    pub align: usize,
}
```

#### Implementations

- <span id="uncheckedtlsimage-check"></span>`fn check(&self) -> Result<TlsImage, InvalidTlsImageError>` — [`TlsImage`](#tlsimage), [`InvalidTlsImageError`](#invalidtlsimageerror)

#### Trait Implementations

##### `impl Clone for UncheckedTlsImage`

- <span id="uncheckedtlsimage-clone"></span>`fn clone(&self) -> UncheckedTlsImage` — [`UncheckedTlsImage`](#uncheckedtlsimage)

##### `impl Copy for UncheckedTlsImage`

##### `impl Debug for UncheckedTlsImage`

- <span id="uncheckedtlsimage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UncheckedTlsImage`

##### `impl PartialEq for UncheckedTlsImage`

- <span id="uncheckedtlsimage-partialeq-eq"></span>`fn eq(&self, other: &UncheckedTlsImage) -> bool` — [`UncheckedTlsImage`](#uncheckedtlsimage)

##### `impl StructuralPartialEq for UncheckedTlsImage`

### `InvalidTlsImageError`

```rust
struct InvalidTlsImageError(());
```

#### Implementations

- <span id="invalidtlsimageerror-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for InvalidTlsImageError`

- <span id="invalidtlsimageerror-clone"></span>`fn clone(&self) -> InvalidTlsImageError` — [`InvalidTlsImageError`](#invalidtlsimageerror)

##### `impl Copy for InvalidTlsImageError`

##### `impl Debug for InvalidTlsImageError`

- <span id="invalidtlsimageerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for InvalidTlsImageError`

##### `impl PartialEq for InvalidTlsImageError`

- <span id="invalidtlsimageerror-partialeq-eq"></span>`fn eq(&self, other: &InvalidTlsImageError) -> bool` — [`InvalidTlsImageError`](#invalidtlsimageerror)

##### `impl StructuralPartialEq for InvalidTlsImageError`

### `TlsImage`

```rust
struct TlsImage {
    checked: UncheckedTlsImage,
}
```

#### Implementations

- <span id="cratetlsimage-with-initialize-on-stack"></span>`unsafe fn with_initialize_on_stack<R>(&self, set_thread_pointer_fn: SetThreadPointerFn, f: impl FnOnce() -> R) -> R` — [`SetThreadPointerFn`](set_thread_pointer/index.md#setthreadpointerfn)

#### Trait Implementations

##### `impl Clone for TlsImage`

- <span id="tlsimage-clone"></span>`fn clone(&self) -> TlsImage` — [`TlsImage`](#tlsimage)

##### `impl Copy for TlsImage`

##### `impl Debug for TlsImage`

- <span id="tlsimage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TlsImage`

##### `impl PartialEq for TlsImage`

- <span id="tlsimage-partialeq-eq"></span>`fn eq(&self, other: &TlsImage) -> bool` — [`TlsImage`](#tlsimage)

##### `impl StructuralPartialEq for TlsImage`

### `RegionLayoutError`

```rust
struct RegionLayoutError(());
```

#### Implementations

- <span id="regionlayouterror-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for RegionLayoutError`

- <span id="regionlayouterror-clone"></span>`fn clone(&self) -> RegionLayoutError` — [`RegionLayoutError`](#regionlayouterror)

##### `impl Copy for RegionLayoutError`

##### `impl Debug for RegionLayoutError`

- <span id="regionlayouterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RegionLayoutError`

##### `impl PartialEq for RegionLayoutError`

- <span id="regionlayouterror-partialeq-eq"></span>`fn eq(&self, other: &RegionLayoutError) -> bool` — [`RegionLayoutError`](#regionlayouterror)

##### `impl StructuralPartialEq for RegionLayoutError`

### `TlsReservationLayout`

```rust
struct TlsReservationLayout {
    footprint: core::alloc::Layout,
    segment_offset: usize,
    thread_pointer_offset: usize,
}
```

#### Implementations

- <span id="tlsreservationlayout-from-segment-layout"></span>`fn from_segment_layout(segment_layout: Layout) -> Self`

- <span id="tlsreservationlayout-footprint"></span>`fn footprint(&self) -> Layout`

- <span id="tlsreservationlayout-segment-offset"></span>`fn segment_offset(&self) -> usize`

- <span id="tlsreservationlayout-thread-pointer-offset"></span>`fn thread_pointer_offset(&self) -> usize`

#### Trait Implementations

##### `impl Clone for TlsReservationLayout`

- <span id="tlsreservationlayout-clone"></span>`fn clone(&self) -> TlsReservationLayout` — [`TlsReservationLayout`](#tlsreservationlayout)

##### `impl Copy for TlsReservationLayout`

##### `impl Debug for TlsReservationLayout`

- <span id="tlsreservationlayout-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TlsReservationLayout`

##### `impl PartialEq for TlsReservationLayout`

- <span id="tlsreservationlayout-partialeq-eq"></span>`fn eq(&self, other: &TlsReservationLayout) -> bool` — [`TlsReservationLayout`](#tlsreservationlayout)

##### `impl StructuralPartialEq for TlsReservationLayout`

### `Region`

```rust
struct Region {
    start: *mut u8,
    size: usize,
}
```

#### Implementations

- <span id="region-new"></span>`const fn new(start: *mut u8, size: usize) -> Self`

- <span id="region-start"></span>`const fn start(&self) -> *mut u8`

- <span id="region-size"></span>`const fn size(&self) -> usize`

- <span id="region-fits-exactly"></span>`fn fits_exactly(&self, layout: Layout) -> bool`

- <span id="region-trim"></span>`fn trim(&self, layout: Layout) -> Result<TrimmedRegion, TrimRegionError>` — [`TrimmedRegion`](#trimmedregion), [`TrimRegionError`](#trimregionerror)

#### Trait Implementations

##### `impl Clone for Region`

- <span id="region-clone"></span>`fn clone(&self) -> Region` — [`Region`](#region)

##### `impl Copy for Region`

##### `impl Debug for Region`

- <span id="region-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Region`

##### `impl PartialEq for Region`

- <span id="region-partialeq-eq"></span>`fn eq(&self, other: &Region) -> bool` — [`Region`](#region)

##### `impl StructuralPartialEq for Region`

### `TrimmedRegion`

```rust
struct TrimmedRegion {
    padding: Region,
    trimmed: Region,
    remainder: Region,
}
```

### `TrimRegionError`

```rust
struct TrimRegionError(());
```

#### Implementations

- <span id="trimregionerror-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for TrimRegionError`

- <span id="trimregionerror-clone"></span>`fn clone(&self) -> TrimRegionError` — [`TrimRegionError`](#trimregionerror)

##### `impl Copy for TrimRegionError`

##### `impl Debug for TrimRegionError`

- <span id="trimregionerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TrimRegionError`

##### `impl PartialEq for TrimRegionError`

- <span id="trimregionerror-partialeq-eq"></span>`fn eq(&self, other: &TrimRegionError) -> bool` — [`TrimRegionError`](#trimregionerror)

##### `impl StructuralPartialEq for TrimRegionError`

### `StaticTlsAllocation<const N: usize, A>`

```rust
struct StaticTlsAllocation<const N: usize, A> {
    _alignment: [A; 0],
    space: core::cell::UnsafeCell<[u8; N]>,
}
```

#### Implementations

- <span id="statictlsallocation-new"></span>`const fn new() -> Self`

- <span id="statictlsallocation-size"></span>`const fn size(&self) -> usize`

- <span id="statictlsallocation-start"></span>`const fn start(&self) -> *mut u8`

- <span id="statictlsallocation-region"></span>`const fn region(&self) -> Region` — [`Region`](#region)

#### Trait Implementations

##### `impl<A> Default for StaticTlsAllocation<N, A>`

- <span id="statictlsallocation-default"></span>`fn default() -> Self`

##### `impl<A> Sync for StaticTlsAllocation<N, A>`

### `HeapTlsReservation`

```rust
struct HeapTlsReservation {
    start: *mut u8,
    layout: core::alloc::Layout,
    thread_pointer: usize,
}
```

#### Implementations

- <span id="heaptlsreservation-initialize"></span>`fn initialize(tls_image: &TlsImage) -> Self` — [`TlsImage`](#tlsimage)

- <span id="heaptlsreservation-thread-pointer"></span>`fn thread_pointer(&self) -> usize`

#### Trait Implementations

##### `impl Drop for HeapTlsReservation`

- <span id="heaptlsreservation-drop"></span>`fn drop(&mut self)`

## Type Aliases

### `SetThreadPointerFn`

```rust
type SetThreadPointerFn = fn(usize);
```

## Constants

### `DEFAULT_SET_THREAD_POINTER_FN`
```rust
const DEFAULT_SET_THREAD_POINTER_FN: SetThreadPointerFn = {set_thread_pointer::default_set_thread_pointer as unsafe extern "C" fn(usize)};
```

