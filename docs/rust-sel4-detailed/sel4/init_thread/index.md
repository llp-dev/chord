*[sel4](../index.md) / [init_thread](index.md)*

---

# Module `init_thread`

Items that are applicable within the context of the root task's initial thread's CSpace.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`slot`](#slot) | mod | Initial CSpace slot constants corresponding to `seL4_Cap*`. |
| [`Slot`](#slot) | struct | The index of a slot in the initial thread's root CNode. |
| [`SlotRegion`](#slotregion) | struct | Corresponds to `seL4_SlotRegion`. |
| [`suspend_self`](#suspend-self) | fn | Suspends the initial thread using [`slot::TCB`]. |

## Modules

- [`slot`](slot/index.md) — Initial CSpace slot constants corresponding to `seL4_Cap*`.

## Structs

### `Slot<T: CapType>`

```rust
struct Slot<T: CapType> {
    index: usize,
    _phantom: core::marker::PhantomData<T>,
}
```

The index of a slot in the initial thread's root CNode.

#### Implementations

- <span id="slot-from-sys"></span>`const fn from_sys(slot: u32) -> Self`

- <span id="slot-from-index"></span>`const fn from_index(index: usize) -> Self`

- <span id="slot-index"></span>`const fn index(&self) -> usize`

- <span id="slot-cptr-bits"></span>`const fn cptr_bits(&self) -> CPtrBits` — [`CPtrBits`](../cptr/index.md#cptrbits)

- <span id="slot-cptr"></span>`const fn cptr(&self) -> CPtr` — [`CPtr`](../cptr/index.md#cptr)

- <span id="slot-cap"></span>`const fn cap(&self) -> Cap<T>` — [`Cap`](../cptr/index.md#cap)

- <span id="slot-cast"></span>`const fn cast<T1: CapType>(&self) -> Slot<T1>` — [`Slot`](#slot)

- <span id="slot-upcast"></span>`const fn upcast(&self) -> Slot` — [`Slot`](#slot)

#### Trait Implementations

##### `impl<T: clone::Clone + CapType> Clone for Slot<T>`

- <span id="slot-clone"></span>`fn clone(&self) -> Slot<T>` — [`Slot`](#slot)

##### `impl<T: marker::Copy + CapType> Copy for Slot<T>`

##### `impl<T: fmt::Debug + CapType> Debug for Slot<T>`

- <span id="slot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + CapType> Eq for Slot<T>`

##### `impl<T: hash::Hash + CapType> Hash for Slot<T>`

- <span id="slot-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord + CapType> Ord for Slot<T>`

- <span id="slot-ord-cmp"></span>`fn cmp(&self, other: &Slot<T>) -> cmp::Ordering` — [`Slot`](#slot)

##### `impl<T: cmp::PartialEq + CapType> PartialEq for Slot<T>`

- <span id="slot-partialeq-eq"></span>`fn eq(&self, other: &Slot<T>) -> bool` — [`Slot`](#slot)

##### `impl<T: cmp::PartialOrd + CapType> PartialOrd for Slot<T>`

- <span id="slot-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Slot<T>) -> option::Option<cmp::Ordering>` — [`Slot`](#slot)

##### `impl<T: CapType> StructuralPartialEq for Slot<T>`

### `SlotRegion<T: CapType>`

```rust
struct SlotRegion<T: CapType> {
    range: core::ops::Range<usize>,
    _phantom: core::marker::PhantomData<T>,
}
```

Corresponds to `seL4_SlotRegion`.

#### Implementations

- <span id="slotregion-from-range"></span>`const fn from_range(range: Range<usize>) -> Self`

- <span id="slotregion-from-sys"></span>`const fn from_sys(sys: sys::seL4_SlotRegion) -> Self`

- <span id="slotregion-start"></span>`const fn start(&self) -> usize`

- <span id="slotregion-end"></span>`const fn end(&self) -> usize`

- <span id="slotregion-range"></span>`const fn range(&self) -> Range<usize>`

- <span id="slotregion-len"></span>`fn len(&self) -> usize`

- <span id="slotregion-index"></span>`fn index(&self, i: usize) -> Slot<T>` — [`Slot`](#slot)

#### Trait Implementations

##### `impl<T: clone::Clone + CapType> Clone for SlotRegion<T>`

- <span id="slotregion-clone"></span>`fn clone(&self) -> SlotRegion<T>` — [`SlotRegion`](#slotregion)

##### `impl<T: fmt::Debug + CapType> Debug for SlotRegion<T>`

- <span id="slotregion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + CapType> Eq for SlotRegion<T>`

##### `impl<T: cmp::PartialEq + CapType> PartialEq for SlotRegion<T>`

- <span id="slotregion-partialeq-eq"></span>`fn eq(&self, other: &SlotRegion<T>) -> bool` — [`SlotRegion`](#slotregion)

##### `impl<T: CapType> StructuralPartialEq for SlotRegion<T>`

## Functions

### `suspend_self`

```rust
fn suspend_self<T>() -> T
```

Suspends the initial thread using [`slot::TCB`](slot/index.md).

