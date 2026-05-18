**heapless > history_buf > storage**

# Module: history_buf::storage

## Contents

**Structs**

- [`HistoryBufStorageInner`](#historybufstorageinner)

**Traits**

- [`HistoryBufSealedStorage`](#historybufsealedstorage)
- [`HistoryBufStorage`](#historybufstorage) - Trait defining how data for a container is stored.

**Type Aliases**

- [`OwnedHistoryBufStorage`](#ownedhistorybufstorage) - Implementation of [`HistoryBufStorage`] that stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewHistoryBufStorage`](#viewhistorybufstorage) - Implementation of [`HistoryBufStorage`] that stores the data in an unsized `[T]`.

---

## heapless::history_buf::storage::HistoryBufSealedStorage

*Trait*

**Methods:**

- `borrow`
- `borrow_mut`
- `as_hist_buf_view`
- `as_hist_buf_mut_view`



## heapless::history_buf::storage::HistoryBufStorage

*Trait*

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedHistoryBufStorage`]: stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewHistoryBufStorage`]: stores the data in an unsized `[T]`.

This allows [`HistoryBuf`] to be generic over either sized or unsized storage. The [`histbuf`]
module contains a [`HistoryBufInner`] struct that's generic on [`HistoryBufStorage`],
and two type aliases for convenience:

- [`HistoryBuf<T, N>`](super::HistoryBuf) = `HistoryBufInner<T, OwnedHistoryBufStorage<T, N>>`
- [`HistoryBufView<T>`](super::HistoryBufView) = `HistoryBufInner<T, ViewHistoryBufStorage<T>>`

`HistoryBuf` can be unsized into `HistoryBufView`, either by unsizing coercions such as `&mut HistoryBuf -> &mut HistoryBufView` or
`Box<HistoryBuf> -> Box<HistoryBufView>`, or explicitly with [`.as_view()`](super::HistoryBuf::as_view) or [`.as_mut_view()`](super::HistoryBuf::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.

[`HistoryBufInner`]: super::HistoryBufInner
[`HistoryBuf`]: super::HistoryBuf
[`HistoryBufView`]: super::HistoryBufView
[`histbuf`]: super



## heapless::history_buf::storage::HistoryBufStorageInner

*Struct*

**Generic Parameters:**
- T



## heapless::history_buf::storage::OwnedHistoryBufStorage

*Type Alias*: `HistoryBufStorageInner<[core::mem::MaybeUninit<T>; N]>`

Implementation of [`HistoryBufStorage`] that stores the data in an array `[T; N]` whose size is known at compile time.



## heapless::history_buf::storage::ViewHistoryBufStorage

*Type Alias*: `HistoryBufStorageInner<[core::mem::MaybeUninit<T>]>`

Implementation of [`HistoryBufStorage`] that stores the data in an unsized `[T]`.



