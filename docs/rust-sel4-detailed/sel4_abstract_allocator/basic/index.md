*[sel4_abstract_allocator](../index.md) / [basic](index.md)*

---

# Module `basic`

## Contents

- [Structs](#structs)
  - [`BasicAllocator`](#basicallocator)
  - [`Allocation`](#allocation)
  - [`InsufficientResources`](#insufficientresources)
- [Functions](#functions)
  - [`copy_typle_fields`](#copy-typle-fields)
- [Type Aliases](#type-aliases)
  - [`Offset`](#offset)
  - [`Size`](#size)
- [Constants](#constants)
  - [`DEFAULT_GRANULE_SIZE`](#default-granule-size)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BasicAllocator`](#basicallocator) | struct |  |
| [`Allocation`](#allocation) | struct |  |
| [`InsufficientResources`](#insufficientresources) | struct |  |
| [`copy_typle_fields`](#copy-typle-fields) | fn |  |
| [`Offset`](#offset) | type |  |
| [`Size`](#size) | type |  |
| [`DEFAULT_GRANULE_SIZE`](#default-granule-size) | const |  |

## Structs

### `BasicAllocator`

```rust
struct BasicAllocator {
    granule_size: usize,
    holes: alloc::collections::BTreeMap<usize, usize>,
}
```

#### Implementations

- <span id="basicallocator-new"></span>`fn new(size: usize) -> Self`

- <span id="basicallocator-with-granule-size"></span>`fn with_granule_size(size: usize, granule_size: usize) -> Self`

- <span id="basicallocator-granule-size"></span>`fn granule_size(&self) -> usize`

#### Trait Implementations

##### `impl AbstractAllocator for BasicAllocator`

- <span id="basicallocator-abstractallocator-type-allocationerror"></span>`type AllocationError = InsufficientResources`

- <span id="basicallocator-abstractallocator-type-allocation"></span>`type Allocation = Allocation`

- <span id="basicallocator-abstractallocator-allocate"></span>`fn allocate(&mut self, orig_layout: Layout) -> Result<<Self as >::Allocation, <Self as >::AllocationError>` — [`AbstractAllocator`](../index.md#abstractallocator)

- <span id="basicallocator-abstractallocator-deallocate"></span>`fn deallocate(&mut self, allocation: <Self as >::Allocation)` — [`AbstractAllocator`](../index.md#abstractallocator)

### `Allocation`

```rust
struct Allocation(core::ops::Range<usize>);
```

#### Implementations

- <span id="allocation-new"></span>`fn new(range: Range<usize>) -> Self`

#### Trait Implementations

##### `impl AbstractAllocatorAllocation for Allocation`

- <span id="allocation-abstractallocatorallocation-range"></span>`fn range(&self) -> Range<usize>`

### `InsufficientResources`

```rust
struct InsufficientResources(());
```

#### Implementations

- <span id="insufficientresources-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for InsufficientResources`

- <span id="insufficientresources-clone"></span>`fn clone(&self) -> InsufficientResources` — [`InsufficientResources`](#insufficientresources)

##### `impl Copy for InsufficientResources`

##### `impl Debug for InsufficientResources`

- <span id="insufficientresources-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for InsufficientResources`

##### `impl Hash for InsufficientResources`

- <span id="insufficientresources-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for InsufficientResources`

- <span id="insufficientresources-ord-cmp"></span>`fn cmp(&self, other: &InsufficientResources) -> cmp::Ordering` — [`InsufficientResources`](#insufficientresources)

##### `impl PartialEq for InsufficientResources`

- <span id="insufficientresources-partialeq-eq"></span>`fn eq(&self, other: &InsufficientResources) -> bool` — [`InsufficientResources`](#insufficientresources)

##### `impl PartialOrd for InsufficientResources`

- <span id="insufficientresources-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &InsufficientResources) -> option::Option<cmp::Ordering>` — [`InsufficientResources`](#insufficientresources)

##### `impl StructuralPartialEq for InsufficientResources`

## Functions

### `copy_typle_fields`

```rust
fn copy_typle_fields<T: Copy, U: Copy>((t, u): (&T, &U)) -> (T, U)
```

## Type Aliases

### `Offset`

```rust
type Offset = usize;
```

### `Size`

```rust
type Size = usize;
```

## Constants

### `DEFAULT_GRANULE_SIZE`
```rust
const DEFAULT_GRANULE_SIZE: usize = 512usize;
```

