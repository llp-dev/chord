**heapless > sorted_linked_list**

# Module: sorted_linked_list

## Contents

**Structs**

- [`FindMutView`](#findmutview) - Comes from [`SortedLinkedList::find_mut`].
- [`IterView`](#iterview) - Iterator for the linked list.
- [`Max`](#max) - Marker for Max sorted [`SortedLinkedList`].
- [`Min`](#min) - Marker for Min sorted [`SortedLinkedList`].
- [`Node`](#node) - A node in the [`SortedLinkedList`].
- [`SortedLinkedListInner`](#sortedlinkedlistinner) - Base struct for [`SortedLinkedList`] and [`SortedLinkedListView`], generic over the [`SortedLinkedListStorage`].

**Traits**

- [`Kind`](#kind) - The linked list kind: min-list or max-list

**Type Aliases**

- [`SortedLinkedList`](#sortedlinkedlist) - The linked list.
- [`SortedLinkedListView`](#sortedlinkedlistview) - The linked list.

---

## heapless::sorted_linked_list::FindMutView

*Struct*

Comes from [`SortedLinkedList::find_mut`].

**Generic Parameters:**
- 'a
- T
- Idx
- K

**Methods:**

- `fn pop(self: Self) -> T` - This will pop the element from the list.
- `fn finish(self: Self)` - This will resort the element into the correct position in the list if needed. The resorting

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## heapless::sorted_linked_list::IterView

*Struct*

Iterator for the linked list.

**Generic Parameters:**
- 'a
- T
- Idx
- K

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## heapless::sorted_linked_list::Kind

*Trait*

The linked list kind: min-list or max-list



## heapless::sorted_linked_list::Max

*Struct*

Marker for Max sorted [`SortedLinkedList`].

**Unit Struct**

**Traits:** Kind



## heapless::sorted_linked_list::Min

*Struct*

Marker for Min sorted [`SortedLinkedList`].

**Unit Struct**

**Traits:** Kind



## heapless::sorted_linked_list::Node

*Struct*

A node in the [`SortedLinkedList`].

**Generic Parameters:**
- T
- Idx



## heapless::sorted_linked_list::SortedLinkedList

*Type Alias*: `SortedLinkedListInner<T, Idx, K, OwnedSortedLinkedListStorage<T, Idx, N>>`

The linked list.



## heapless::sorted_linked_list::SortedLinkedListInner

*Struct*

Base struct for [`SortedLinkedList`] and [`SortedLinkedListView`], generic over the [`SortedLinkedListStorage`].

In most cases you should use [`SortedLinkedList`] or [`SortedLinkedListView`] directly. Only use this
struct if you want to write code that's generic over both.

**Generic Parameters:**
- T
- Idx
- K
- S

**Methods:**

- `fn new_usize() -> Self` - Create a new linked list.
- `fn new_u8() -> Self` - Create a new linked list.
- `fn iter(self: &Self) -> IterView<T, Idx, K>` - Get an iterator over the sorted list.
- `fn find_mut<F>(self: & mut Self, f: F) -> Option<FindMutView<T, Idx, K>>` - Find an element in the list that can be changed and resorted.
- `fn new_u16() -> Self` - Create a new linked list.
- `fn push_unchecked(self: & mut Self, value: T)` - Pushes a value onto the list without checking if the list is full.
- `fn push(self: & mut Self, value: T) -> Result<(), T>` - Pushes an element to the linked list and sorts it into place.
- `fn peek(self: &Self) -> Option<&T>` - Peek at the first element.
- `fn pop_unchecked(self: & mut Self) -> T` - Pop an element from the list without checking so the list is not empty.
- `fn pop(self: & mut Self) -> Option<T>` - Pops the first element in the list.
- `fn is_full(self: &Self) -> bool` - Checks if the linked list is full.
- `fn is_empty(self: &Self) -> bool` - Checks if the linked list is empty.
- `fn as_view(self: &Self) -> &SortedLinkedListView<T, K, Idx>` - Get a reference to the `SortedLinkedList`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut SortedLinkedListView<T, K, Idx>` - Get a mutable reference to the `Vec`, erasing the `N` const-generic.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## heapless::sorted_linked_list::SortedLinkedListView

*Type Alias*: `SortedLinkedListInner<T, Idx, K, ViewSortedLinkedListStorage<T, Idx>>`

The linked list.



