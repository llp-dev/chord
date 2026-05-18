**heapless > storage**

# Module: storage

## Contents

**Enums**

- [`OwnedStorage`](#ownedstorage) - Implementation of [`Storage`] that stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewStorage`](#viewstorage) - Implementation of [`Storage`] that stores the data in an unsized `[T]`.

**Traits**

- [`Storage`](#storage) - Trait defining how data for a container is stored.

---

## heapless::storage::OwnedStorage

*Enum*

Implementation of [`Storage`] that stores the data in an array `[T; N]` whose size is known at compile time.

**Generic Parameters:**
- const N

**Traits:** Storage



## heapless::storage::Storage

*Trait*

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedStorage`]: stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewStorage`]: stores the data in an unsized `[T]`.

This allows containers to be generic over either sized or unsized storage. For example,
the [`vec`](crate::vec) module contains a [`VecInner`](crate::vec::VecInner) struct
that's generic on [`Storage`], and two type aliases for convenience:

- [`Vec<T, N>`](crate::vec::Vec) = `VecInner<T, OwnedStorage<N>>`
- [`VecView<T>`](crate::vec::VecView) = `VecInner<T, ViewStorage>`

`Vec` can be unsized into `VecView`, either by unsizing coercions such as `&mut Vec -> &mut VecView` or
`Box<Vec> -> Box<VecView>`, or explicitly with [`.as_view()`](crate::vec::Vec::as_view) or [`.as_mut_view()`](crate::vec::Vec::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.



## heapless::storage::ViewStorage

*Enum*

Implementation of [`Storage`] that stores the data in an unsized `[T]`.

**Traits:** Storage



