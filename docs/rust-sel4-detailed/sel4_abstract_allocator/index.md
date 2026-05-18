# Crate `sel4_abstract_allocator`

## Contents

- [Modules](#modules)
  - [`bump`](#bump)
  - [`basic`](#basic)
  - [`by_range`](#by-range)
- [Structs](#structs)
  - [`ByRange`](#byrange)
  - [`WithAlignmentBound`](#withalignmentbound)
- [Enums](#enums)
  - [`WithAlignmentBoundAllocationError`](#withalignmentboundallocationerror)
- [Traits](#traits)
  - [`AbstractAllocatorAllocation`](#abstractallocatorallocation)
  - [`AbstractAllocator`](#abstractallocator)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bump`](#bump) | mod |  |
| [`basic`](#basic) | mod |  |
| [`by_range`](#by-range) | mod |  |
| [`ByRange`](#byrange) | struct |  |
| [`WithAlignmentBound`](#withalignmentbound) | struct |  |
| [`WithAlignmentBoundAllocationError`](#withalignmentboundallocationerror) | enum |  |
| [`AbstractAllocatorAllocation`](#abstractallocatorallocation) | trait |  |
| [`AbstractAllocator`](#abstractallocator) | trait |  |

## Modules

- [`bump`](bump/index.md)
- [`basic`](basic/index.md)
- [`by_range`](by_range/index.md)

## Structs

### `ByRange<A: AbstractAllocator>`

```rust
struct ByRange<A: AbstractAllocator> {
    inner: A,
    allocations: alloc::collections::BTreeMap<RangeKey<usize>, <A as >::Allocation>,
}
```

#### Implementations

- <span id="byrange-new"></span>`const fn new(inner: A) -> Self`

- <span id="byrange-allocate"></span>`fn allocate(&mut self, layout: Layout) -> Result<Range<usize>, <A as >::AllocationError>` — [`AbstractAllocator`](#abstractallocator)

- <span id="byrange-deallocate"></span>`fn deallocate(&mut self, range: Range<usize>)`

### `WithAlignmentBound<A>`

```rust
struct WithAlignmentBound<A> {
    inner: A,
    max_alignment: usize,
}
```

#### Implementations

- <span id="withalignmentbound-new"></span>`const fn new(inner: A, max_alignment: usize) -> Self`

- <span id="withalignmentbound-max-alignment"></span>`const fn max_alignment(&self) -> usize`

- <span id="withalignmentbound-is-suitably-aligned"></span>`fn is_suitably_aligned(&self, region: *mut u8) -> bool`

#### Trait Implementations

##### `impl<A: AbstractAllocator> AbstractAllocator for WithAlignmentBound<A>`

- <span id="withalignmentbound-abstractallocator-type-allocationerror"></span>`type AllocationError = WithAlignmentBoundAllocationError<<A as AbstractAllocator>::AllocationError>`

- <span id="withalignmentbound-abstractallocator-type-allocation"></span>`type Allocation = <A as AbstractAllocator>::Allocation`

- <span id="withalignmentbound-abstractallocator-allocate"></span>`fn allocate(&mut self, layout: Layout) -> Result<<Self as >::Allocation, <Self as >::AllocationError>` — [`AbstractAllocator`](#abstractallocator)

- <span id="withalignmentbound-abstractallocator-deallocate"></span>`fn deallocate(&mut self, allocation: <Self as >::Allocation)` — [`AbstractAllocator`](#abstractallocator)

## Enums

### `WithAlignmentBoundAllocationError<E>`

```rust
enum WithAlignmentBoundAllocationError<E> {
    InnerError(E),
    AlignmentExceedsBound,
}
```

#### Trait Implementations

##### `impl<E: clone::Clone> Clone for WithAlignmentBoundAllocationError<E>`

- <span id="withalignmentboundallocationerror-clone"></span>`fn clone(&self) -> WithAlignmentBoundAllocationError<E>` — [`WithAlignmentBoundAllocationError`](#withalignmentboundallocationerror)

##### `impl<E: marker::Copy> Copy for WithAlignmentBoundAllocationError<E>`

##### `impl<E: fmt::Debug> Debug for WithAlignmentBoundAllocationError<E>`

- <span id="withalignmentboundallocationerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: cmp::Eq> Eq for WithAlignmentBoundAllocationError<E>`

##### `impl<E: hash::Hash> Hash for WithAlignmentBoundAllocationError<E>`

- <span id="withalignmentboundallocationerror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord> Ord for WithAlignmentBoundAllocationError<E>`

- <span id="withalignmentboundallocationerror-ord-cmp"></span>`fn cmp(&self, other: &WithAlignmentBoundAllocationError<E>) -> cmp::Ordering` — [`WithAlignmentBoundAllocationError`](#withalignmentboundallocationerror)

##### `impl<E: cmp::PartialEq> PartialEq for WithAlignmentBoundAllocationError<E>`

- <span id="withalignmentboundallocationerror-partialeq-eq"></span>`fn eq(&self, other: &WithAlignmentBoundAllocationError<E>) -> bool` — [`WithAlignmentBoundAllocationError`](#withalignmentboundallocationerror)

##### `impl<E: cmp::PartialOrd> PartialOrd for WithAlignmentBoundAllocationError<E>`

- <span id="withalignmentboundallocationerror-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &WithAlignmentBoundAllocationError<E>) -> option::Option<cmp::Ordering>` — [`WithAlignmentBoundAllocationError`](#withalignmentboundallocationerror)

##### `impl<E> StructuralPartialEq for WithAlignmentBoundAllocationError<E>`

## Traits

### `AbstractAllocatorAllocation`

```rust
trait AbstractAllocatorAllocation { ... }
```

#### Required Methods

- `fn range(&self) -> Range<usize>`

#### Implementors

- [`Allocation`](basic/index.md#allocation)
- [`Allocation`](bump/index.md#allocation)

### `AbstractAllocator`

```rust
trait AbstractAllocator { ... }
```

#### Associated Types

- `type AllocationError: 1`

- `type Allocation: 1`

#### Required Methods

- `fn allocate(&mut self, layout: Layout) -> Result<<Self as >::Allocation, <Self as >::AllocationError>`

- `fn deallocate(&mut self, allocation: <Self as >::Allocation)`

#### Implementors

- [`BasicAllocator`](basic/index.md#basicallocator)
- [`BumpAllocator`](bump/index.md#bumpallocator)
- [`WithAlignmentBound`](#withalignmentbound)

