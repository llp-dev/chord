*[sel4_abstract_allocator](../index.md) / [bump](index.md)*

---

# Module `bump`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BumpAllocator`](#bumpallocator) | struct |  |
| [`Allocation`](#allocation) | struct |  |
| [`InsufficientResources`](#insufficientresources) | struct |  |

## Structs

### `BumpAllocator`

```rust
struct BumpAllocator {
    watermark: usize,
    size: usize,
}
```

#### Implementations

- <span id="bumpallocator-new"></span>`fn new(size: usize) -> Self`

#### Trait Implementations

##### `impl AbstractAllocator for BumpAllocator`

- <span id="bumpallocator-abstractallocator-type-allocationerror"></span>`type AllocationError = InsufficientResources`

- <span id="bumpallocator-abstractallocator-type-allocation"></span>`type Allocation = Allocation`

- <span id="bumpallocator-abstractallocator-allocate"></span>`fn allocate(&mut self, layout: Layout) -> Result<<Self as >::Allocation, <Self as >::AllocationError>` — [`AbstractAllocator`](../index.md#abstractallocator)

- <span id="bumpallocator-abstractallocator-deallocate"></span>`fn deallocate(&mut self, _allocation: <Self as >::Allocation)` — [`AbstractAllocator`](../index.md#abstractallocator)

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

