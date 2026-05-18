*[heapless](../../index.md) / [history_buf](../index.md) / [storage](index.md)*

---

# Module `storage`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HistoryBufStorageInner`](#historybufstorageinner) | struct |  |
| [`HistoryBufStorage`](#historybufstorage) | trait | Trait defining how data for a container is stored. |
| [`HistoryBufSealedStorage`](#historybufsealedstorage) | trait |  |
| [`OwnedHistoryBufStorage`](#ownedhistorybufstorage) | type | Implementation of [`HistoryBufStorage`] that stores the data in an array `[T; N]` whose size is known at compile time. |
| [`ViewHistoryBufStorage`](#viewhistorybufstorage) | type | Implementation of [`HistoryBufStorage`] that stores the data in an unsized `[T]`. |

## Structs

### `HistoryBufStorageInner<T: ?Sized>`

```rust
struct HistoryBufStorageInner<T: ?Sized> {
    buffer: T,
}
```

## Traits

### `HistoryBufStorage<T>`

```rust
trait HistoryBufStorage<T>: HistoryBufSealedStorage<T> { ... }
```

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedHistoryBufStorage`](#ownedhistorybufstorage): stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewHistoryBufStorage`](#viewhistorybufstorage): stores the data in an unsized `[T]`.

This allows [`HistoryBuf`](../index.md) to be generic over either sized or unsized storage. The `histbuf`
module contains a [`HistoryBufInner`](../index.md) struct that's generic on [`HistoryBufStorage`](#historybufstorage),
and two type aliases for convenience:

- [`HistoryBuf<T, N>`](super::HistoryBuf) = `HistoryBufInner<T, OwnedHistoryBufStorage<T, N>>`
- [`HistoryBufView<T>`](super::HistoryBufView) = `HistoryBufInner<T, ViewHistoryBufStorage<T>>`

`HistoryBuf` can be unsized into `HistoryBufView`, either by unsizing coercions such as `&mut HistoryBuf -> &mut HistoryBufView` or
`Box<HistoryBuf> -> Box<HistoryBufView>`, or explicitly with [`.as_view()`](super::HistoryBuf::as_view) or [`.as_mut_view()`](super::HistoryBuf::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.





#### Implementors

- [`OwnedHistoryBufStorage`](#ownedhistorybufstorage)
- [`ViewHistoryBufStorage`](#viewhistorybufstorage)

### `HistoryBufSealedStorage<T>`

```rust
trait HistoryBufSealedStorage<T> { ... }
```

#### Required Methods

- `fn borrow(&self) -> &[MaybeUninit<T>]`

- `fn borrow_mut(&mut self) -> &mut [MaybeUninit<T>]`

- `fn as_hist_buf_view(this: &HistoryBufInner<T, Self>) -> &HistoryBufView<T>`

- `fn as_hist_buf_mut_view(this: &mut HistoryBufInner<T, Self>) -> &mut HistoryBufView<T>`

#### Implementors

- [`OwnedHistoryBufStorage`](#ownedhistorybufstorage)
- [`ViewHistoryBufStorage`](#viewhistorybufstorage)

## Type Aliases

### `OwnedHistoryBufStorage<T, const N: usize>`

```rust
type OwnedHistoryBufStorage<T, const N: usize> = HistoryBufStorageInner<[core::mem::MaybeUninit<T>; N]>;
```

Implementation of [`HistoryBufStorage`](#historybufstorage) that stores the data in an array `[T; N]` whose size is known at compile time.

### `ViewHistoryBufStorage<T>`

```rust
type ViewHistoryBufStorage<T> = HistoryBufStorageInner<[core::mem::MaybeUninit<T>]>;
```

Implementation of [`HistoryBufStorage`](#historybufstorage) that stores the data in an unsized `[T]`.

