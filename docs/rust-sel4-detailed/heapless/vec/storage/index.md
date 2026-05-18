*[heapless](../../index.md) / [vec](../index.md) / [storage](index.md)*

---

# Module `storage`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`VecStorageInner`](#vecstorageinner) | struct |  |
| [`VecStorage`](#vecstorage) | trait | Trait defining how data for a container is stored. |
| [`VecSealedStorage`](#vecsealedstorage) | trait |  |
| [`OwnedVecStorage`](#ownedvecstorage) | type | Implementation of [`VecStorage`] that stores the data in an array `[T; N]` whose size is known at compile time. |
| [`ViewVecStorage`](#viewvecstorage) | type | Implementation of [`VecStorage`] that stores the data in an unsized `[T]`. |

## Structs

### `VecStorageInner<T: ?Sized>`

```rust
struct VecStorageInner<T: ?Sized> {
    buffer: T,
}
```

## Traits

### `VecStorage<T>`

```rust
trait VecStorage<T>: VecSealedStorage<T> { ... }
```

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedVecStorage`](#ownedvecstorage): stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewVecStorage`](#viewvecstorage): stores the data in an unsized `[T]`.

This allows [`Vec`](../index.md) to be generic over either sized or unsized storage. The [`vec`](super)
module contains a [`VecInner`](../index.md) struct that's generic on [`VecStorage`](#vecstorage),
and two type aliases for convenience:

- [`Vec<T, N>`](crate::vec::Vec) = `VecInner<T, OwnedStorage<T, N>>`
- [`VecView<T>`](crate::vec::VecView) = `VecInner<T, ViewStorage<T>>`

`Vec` can be unsized into `VecView`, either by unsizing coercions such as `&mut Vec -> &mut VecView` or
`Box<Vec> -> Box<VecView>`, or explicitly with [`.as_view()`](crate::vec::Vec::as_view) or [`.as_mut_view()`](crate::vec::Vec::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.




#### Implementors

- [`OwnedVecStorage`](#ownedvecstorage)
- [`ViewVecStorage`](#viewvecstorage)

### `VecSealedStorage<T>`

```rust
trait VecSealedStorage<T> { ... }
```

#### Required Methods

- `fn borrow(&self) -> &[MaybeUninit<T>]`

- `fn borrow_mut(&mut self) -> &mut [MaybeUninit<T>]`

- `fn as_vec_view<LenT: LenType>(this: &VecInner<T, LenT, Self>) -> &VecView<T, LenT>`

- `fn as_vec_view_mut<LenT: LenType>(this: &mut VecInner<T, LenT, Self>) -> &mut VecView<T, LenT>`

- `fn as_binary_heap_view<K>(this: &BinaryHeapInner<T, K, Self>) -> &BinaryHeapView<T, K>`

- `fn as_binary_heap_view_mut<K>(this: &mut BinaryHeapInner<T, K, Self>) -> &mut BinaryHeapView<T, K>`

- `fn as_deque_view(this: &DequeInner<T, Self>) -> &DequeView<T>`

- `fn as_deque_view_mut(this: &mut DequeInner<T, Self>) -> &mut DequeView<T>`

#### Implementors

- [`OwnedVecStorage`](#ownedvecstorage)
- [`ViewVecStorage`](#viewvecstorage)

## Type Aliases

### `OwnedVecStorage<T, const N: usize>`

```rust
type OwnedVecStorage<T, const N: usize> = VecStorageInner<[core::mem::MaybeUninit<T>; N]>;
```

Implementation of [`VecStorage`](#vecstorage) that stores the data in an array `[T; N]` whose size is known at compile time.

### `ViewVecStorage<T>`

```rust
type ViewVecStorage<T> = VecStorageInner<[core::mem::MaybeUninit<T>]>;
```

Implementation of [`VecStorage`](#vecstorage) that stores the data in an unsized `[T]`.

