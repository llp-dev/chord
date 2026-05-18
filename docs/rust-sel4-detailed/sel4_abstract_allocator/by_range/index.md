*[sel4_abstract_allocator](../index.md) / [by_range](index.md)*

---

# Module `by_range`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ByRange`](#byrange) | struct |  |
| [`RangeKey`](#rangekey) | struct |  |

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

- <span id="byrange-allocate"></span>`fn allocate(&mut self, layout: Layout) -> Result<Range<usize>, <A as >::AllocationError>` — [`AbstractAllocator`](../index.md#abstractallocator)

- <span id="byrange-deallocate"></span>`fn deallocate(&mut self, range: Range<usize>)`

### `RangeKey<T>`

```rust
struct RangeKey<T> {
    start: T,
    end: T,
}
```

#### Trait Implementations

##### `impl<T: cmp::Eq> Eq for RangeKey<T>`

##### `impl<T: cmp::Ord> Ord for RangeKey<T>`

- <span id="rangekey-ord-cmp"></span>`fn cmp(&self, other: &RangeKey<T>) -> cmp::Ordering` — [`RangeKey`](#rangekey)

##### `impl<T: cmp::PartialEq> PartialEq for RangeKey<T>`

- <span id="rangekey-partialeq-eq"></span>`fn eq(&self, other: &RangeKey<T>) -> bool` — [`RangeKey`](#rangekey)

##### `impl<T: cmp::PartialOrd> PartialOrd for RangeKey<T>`

- <span id="rangekey-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &RangeKey<T>) -> option::Option<cmp::Ordering>` — [`RangeKey`](#rangekey)

##### `impl<T> StructuralPartialEq for RangeKey<T>`

