**heapless > vec > storage**

# Module: vec::storage

## Contents

**Structs**

- [`VecStorageInner`](#vecstorageinner)

**Traits**

- [`VecSealedStorage`](#vecsealedstorage)
- [`VecStorage`](#vecstorage) - Trait defining how data for a container is stored.

**Type Aliases**

- [`OwnedVecStorage`](#ownedvecstorage) - Implementation of [`VecStorage`] that stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewVecStorage`](#viewvecstorage) - Implementation of [`VecStorage`] that stores the data in an unsized `[T]`.

---

## heapless::vec::storage::OwnedVecStorage

*Type Alias*: `VecStorageInner<[core::mem::MaybeUninit<T>; N]>`

Implementation of [`VecStorage`] that stores the data in an array `[T; N]` whose size is known at compile time.



## heapless::vec::storage::VecSealedStorage

*Trait*

**Methods:**

- `borrow`
- `borrow_mut`
- `as_vec_view`
- `as_vec_view_mut`
- `as_binary_heap_view`
- `as_binary_heap_view_mut`
- `as_deque_view`
- `as_deque_view_mut`



## heapless::vec::storage::VecStorage

*Trait*

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedVecStorage`]: stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewVecStorage`]: stores the data in an unsized `[T]`.

This allows [`Vec`] to be generic over either sized or unsized storage. The [`vec`](super)
module contains a [`VecInner`] struct that's generic on [`VecStorage`],
and two type aliases for convenience:

- [`Vec<T, N>`](crate::vec::Vec) = `VecInner<T, OwnedStorage<T, N>>`
- [`VecView<T>`](crate::vec::VecView) = `VecInner<T, ViewStorage<T>>`

`Vec` can be unsized into `VecView`, either by unsizing coercions such as `&mut Vec -> &mut VecView` or
`Box<Vec> -> Box<VecView>`, or explicitly with [`.as_view()`](crate::vec::Vec::as_view) or [`.as_mut_view()`](crate::vec::Vec::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.

[`VecInner`]: super::VecInner
[`Vec`]: super::Vec
[`VecView`]: super::VecView



## heapless::vec::storage::VecStorageInner

*Struct*

**Generic Parameters:**
- T



## heapless::vec::storage::ViewVecStorage

*Type Alias*: `VecStorageInner<[core::mem::MaybeUninit<T>]>`

Implementation of [`VecStorage`] that stores the data in an unsized `[T]`.



