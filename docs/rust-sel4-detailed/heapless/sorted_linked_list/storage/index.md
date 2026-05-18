*[heapless](../../index.md) / [sorted_linked_list](../index.md) / [storage](index.md)*

---

# Module `storage`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SortedLinkedListStorageInner`](#sortedlinkedliststorageinner) | struct |  |
| [`SortedLinkedListStorage`](#sortedlinkedliststorage) | trait | Trait defining how data for a container is stored. |
| [`SortedLinkedListSealedStorage`](#sortedlinkedlistsealedstorage) | trait |  |
| [`OwnedSortedLinkedListStorage`](#ownedsortedlinkedliststorage) | type | Implementation of [`SortedLinkedListStorage`] that stores the data in an array `[T; N]` whose size is known at compile time. |
| [`ViewSortedLinkedListStorage`](#viewsortedlinkedliststorage) | type | Implementation of [`SortedLinkedListStorage`] that stores the data in an unsized `[T]`. |

## Structs

### `SortedLinkedListStorageInner<T: ?Sized>`

```rust
struct SortedLinkedListStorageInner<T: ?Sized> {
    buffer: T,
}
```

## Traits

### `SortedLinkedListStorage<T, Idx>`

```rust
trait SortedLinkedListStorage<T, Idx>: SortedLinkedListSealedStorage<T, Idx> { ... }
```

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedSortedLinkedListStorage`](#ownedsortedlinkedliststorage): stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewSortedLinkedListStorage`](#viewsortedlinkedliststorage): stores the data in an unsized `[T]`.

This allows [`SortedLinkedList`](../index.md) to be generic over either sized or unsized storage. The [`sorted_linked_list`](super)
module contains a [`SortedLinkedListInner`](../index.md) struct that's generic on [`SortedLinkedListStorage`](#sortedlinkedliststorage),
and two type aliases for convenience:

- [`SortedLinkedList<T, Idx, N>`](super::SortedLinkedList) = `SortedLinkedListInner<T, OwnedSortedLinkedListStorage<T, Idx, N>>`
- [`SortedLinkedListView<T, Idx>`](super::SortedLinkedListView) = `SortedLinkedListInner<T, ViewSortedLinkedListStorage<T, Idx>>`

`SortedLinkedList` can be unsized into `SortedLinkedListView`, either by unsizing coercions such as `&mut SortedLinkedList -> &mut SortedLinkedListView` or
`Box<SortedLinkedList> -> Box<SortedLinkedListView>`, or explicitly with [`.as_view()`](super::SortedLinkedList::as_view) or [`.as_mut_view()`](super::SortedLinkedList::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.




#### Implementors

- [`OwnedSortedLinkedListStorage`](#ownedsortedlinkedliststorage)
- [`ViewSortedLinkedListStorage`](#viewsortedlinkedliststorage)

### `SortedLinkedListSealedStorage<T, Idx>`

```rust
trait SortedLinkedListSealedStorage<T, Idx> { ... }
```

#### Required Methods

- `fn borrow(&self) -> &[Node<T, Idx>]`

- `fn borrow_mut(&mut self) -> &mut [Node<T, Idx>]`

- `fn as_view<K>(this: &SortedLinkedListInner<T, Idx, K, Self>) -> &SortedLinkedListView<T, K, Idx>`

- `fn as_mut_view<K>(this: &mut SortedLinkedListInner<T, Idx, K, Self>) -> &mut SortedLinkedListView<T, K, Idx>`

#### Implementors

- [`OwnedSortedLinkedListStorage`](#ownedsortedlinkedliststorage)
- [`ViewSortedLinkedListStorage`](#viewsortedlinkedliststorage)

## Type Aliases

### `OwnedSortedLinkedListStorage<T, Idx, const N: usize>`

```rust
type OwnedSortedLinkedListStorage<T, Idx, const N: usize> = SortedLinkedListStorageInner<[super::Node<T, Idx>; N]>;
```

Implementation of [`SortedLinkedListStorage`](#sortedlinkedliststorage) that stores the data in an array `[T; N]` whose size is known at compile time.

### `ViewSortedLinkedListStorage<T, Idx>`

```rust
type ViewSortedLinkedListStorage<T, Idx> = SortedLinkedListStorageInner<[super::Node<T, Idx>]>;
```

Implementation of [`SortedLinkedListStorage`](#sortedlinkedliststorage) that stores the data in an unsized `[T]`.

