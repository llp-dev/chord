**heapless > sorted_linked_list > storage**

# Module: sorted_linked_list::storage

## Contents

**Structs**

- [`SortedLinkedListStorageInner`](#sortedlinkedliststorageinner)

**Traits**

- [`SortedLinkedListSealedStorage`](#sortedlinkedlistsealedstorage)
- [`SortedLinkedListStorage`](#sortedlinkedliststorage) - Trait defining how data for a container is stored.

**Type Aliases**

- [`OwnedSortedLinkedListStorage`](#ownedsortedlinkedliststorage) - Implementation of [`SortedLinkedListStorage`] that stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewSortedLinkedListStorage`](#viewsortedlinkedliststorage) - Implementation of [`SortedLinkedListStorage`] that stores the data in an unsized `[T]`.

---

## heapless::sorted_linked_list::storage::OwnedSortedLinkedListStorage

*Type Alias*: `SortedLinkedListStorageInner<[super::Node<T, Idx>; N]>`

Implementation of [`SortedLinkedListStorage`] that stores the data in an array `[T; N]` whose size is known at compile time.



## heapless::sorted_linked_list::storage::SortedLinkedListSealedStorage

*Trait*

**Methods:**

- `borrow`
- `borrow_mut`
- `as_view`
- `as_mut_view`



## heapless::sorted_linked_list::storage::SortedLinkedListStorage

*Trait*

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedSortedLinkedListStorage`]: stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewSortedLinkedListStorage`]: stores the data in an unsized `[T]`.

This allows [`SortedLinkedList`] to be generic over either sized or unsized storage. The [`sorted_linked_list`](super)
module contains a [`SortedLinkedListInner`] struct that's generic on [`SortedLinkedListStorage`],
and two type aliases for convenience:

- [`SortedLinkedList<T, Idx, N>`](super::SortedLinkedList) = `SortedLinkedListInner<T, OwnedSortedLinkedListStorage<T, Idx, N>>`
- [`SortedLinkedListView<T, Idx>`](super::SortedLinkedListView) = `SortedLinkedListInner<T, ViewSortedLinkedListStorage<T, Idx>>`

`SortedLinkedList` can be unsized into `SortedLinkedListView`, either by unsizing coercions such as `&mut SortedLinkedList -> &mut SortedLinkedListView` or
`Box<SortedLinkedList> -> Box<SortedLinkedListView>`, or explicitly with [`.as_view()`](super::SortedLinkedList::as_view) or [`.as_mut_view()`](super::SortedLinkedList::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.

[`SortedLinkedListInner`]: super::SortedLinkedListInner
[`SortedLinkedList`]: super::SortedLinkedList
[`SortedLinkedListView`]: super::SortedLinkedListView



## heapless::sorted_linked_list::storage::SortedLinkedListStorageInner

*Struct*

**Generic Parameters:**
- T



## heapless::sorted_linked_list::storage::ViewSortedLinkedListStorage

*Type Alias*: `SortedLinkedListStorageInner<[super::Node<T, Idx>]>`

Implementation of [`SortedLinkedListStorage`] that stores the data in an unsized `[T]`.



