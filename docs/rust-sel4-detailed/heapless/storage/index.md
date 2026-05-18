*[heapless](../index.md) / [storage](index.md)*

---

# Module `storage`

`Storage` trait defining how data is stored in a container.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OwnedStorage`](#ownedstorage) | enum | Implementation of [`Storage`] that stores the data in an array `[T; N]` whose size is known at compile time. |
| [`ViewStorage`](#viewstorage) | enum | Implementation of [`Storage`] that stores the data in an unsized `[T]`. |
| [`SealedStorage`](#sealedstorage) | trait |  |
| [`Storage`](#storage) | trait | Trait defining how data for a container is stored. |

## Enums

### `OwnedStorage<const N: usize>`

```rust
enum OwnedStorage<const N: usize> {
}
```

Implementation of [`Storage`](#storage) that stores the data in an array `[T; N]` whose size is known at compile time.

#### Trait Implementations

##### `impl SealedStorage for OwnedStorage<N>`

- <span id="ownedstorage-sealedstorage-type-buffer"></span>`type Buffer = [T; N]`

- <span id="ownedstorage-sealedstorage-len"></span>`fn len<T>(_: *const <Self as >::Buffer) -> usize` — [`SealedStorage`](#sealedstorage)

- <span id="ownedstorage-sealedstorage-as-ptr"></span>`fn as_ptr<T>(this: *mut <Self as >::Buffer) -> *mut T` — [`SealedStorage`](#sealedstorage)

- <span id="ownedstorage-sealedstorage-as-mpmc-view"></span>`fn as_mpmc_view<T>(this: &mpmc::Queue<T, N>) -> &mpmc::QueueView<T>` — [`Queue`](../mpmc/index.md#queue), [`QueueView`](../mpmc/index.md#queueview)

- <span id="ownedstorage-sealedstorage-as-mpmc-mut-view"></span>`fn as_mpmc_mut_view<T>(this: &mut mpmc::Queue<T, N>) -> &mut mpmc::QueueView<T>` — [`Queue`](../mpmc/index.md#queue), [`QueueView`](../mpmc/index.md#queueview)

- <span id="ownedstorage-sealedstorage-as-queue-view"></span>`fn as_queue_view<T>(this: &spsc::QueueInner<T, Self>) -> &spsc::QueueView<T>` — [`QueueInner`](../spsc/index.md#queueinner), [`QueueView`](../spsc/index.md#queueview)

  Convert a `Queue` to a `QueueView`

- <span id="ownedstorage-sealedstorage-as-mut-queue-view"></span>`fn as_mut_queue_view<T>(this: &mut spsc::QueueInner<T, Self>) -> &mut spsc::QueueView<T>` — [`QueueInner`](../spsc/index.md#queueinner), [`QueueView`](../spsc/index.md#queueview)

  Convert a `Queue` to a `QueueView`

##### `impl Storage for OwnedStorage<N>`

### `ViewStorage`

```rust
enum ViewStorage {
}
```

Implementation of [`Storage`](#storage) that stores the data in an unsized `[T]`.

#### Trait Implementations

##### `impl SealedStorage for ViewStorage`

- <span id="viewstorage-sealedstorage-type-buffer"></span>`type Buffer = [T]`

- <span id="viewstorage-sealedstorage-len"></span>`fn len<T>(this: *const <Self as >::Buffer) -> usize` — [`SealedStorage`](#sealedstorage)

- <span id="viewstorage-sealedstorage-as-ptr"></span>`fn as_ptr<T>(this: *mut <Self as >::Buffer) -> *mut T` — [`SealedStorage`](#sealedstorage)

- <span id="viewstorage-sealedstorage-as-mpmc-view"></span>`fn as_mpmc_view<T>(this: &mpmc::QueueInner<T, Self>) -> &mpmc::QueueView<T>` — [`QueueInner`](../mpmc/index.md#queueinner), [`QueueView`](../mpmc/index.md#queueview)

- <span id="viewstorage-sealedstorage-as-mpmc-mut-view"></span>`fn as_mpmc_mut_view<T>(this: &mut mpmc::QueueInner<T, Self>) -> &mut mpmc::QueueView<T>` — [`QueueInner`](../mpmc/index.md#queueinner), [`QueueView`](../mpmc/index.md#queueview)

- <span id="viewstorage-sealedstorage-as-queue-view"></span>`fn as_queue_view<T>(this: &spsc::QueueInner<T, Self>) -> &spsc::QueueView<T>` — [`QueueInner`](../spsc/index.md#queueinner), [`QueueView`](../spsc/index.md#queueview)

  Convert a `Queue` to a `QueueView`

- <span id="viewstorage-sealedstorage-as-mut-queue-view"></span>`fn as_mut_queue_view<T>(this: &mut spsc::QueueInner<T, Self>) -> &mut spsc::QueueView<T>` — [`QueueInner`](../spsc/index.md#queueinner), [`QueueView`](../spsc/index.md#queueview)

  Convert a `Queue` to a `QueueView`

##### `impl Storage for ViewStorage`

## Traits

### `SealedStorage`

```rust
trait SealedStorage { ... }
```

#### Associated Types

- `type Buffer: 3`

#### Required Methods

- `fn len<T>(this: *const <Self as >::Buffer) -> usize`

  Obtain the length of the buffer

- `fn as_ptr<T>(this: *mut <Self as >::Buffer) -> *mut T`

  Obtain access to the first element of the buffer

- `fn as_mpmc_view<T>(this: &mpmc::QueueInner<T, Self>) -> &mpmc::QueueView<T>`

- `fn as_mpmc_mut_view<T>(this: &mut mpmc::QueueInner<T, Self>) -> &mut mpmc::QueueView<T>`

- `fn as_queue_view<T>(this: &spsc::QueueInner<T, Self>) -> &spsc::QueueView<T>`

  Convert a `Queue` to a `QueueView`

- `fn as_mut_queue_view<T>(this: &mut spsc::QueueInner<T, Self>) -> &mut spsc::QueueView<T>`

  Convert a `Queue` to a `QueueView`

#### Implementors

- [`OwnedStorage`](#ownedstorage)
- [`ViewStorage`](#viewstorage)

### `Storage`

```rust
trait Storage: SealedStorage { ... }
```

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedStorage`](#ownedstorage): stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewStorage`](#viewstorage): stores the data in an unsized `[T]`.

This allows containers to be generic over either sized or unsized storage. For example,
the [`vec`](crate::vec) module contains a [`VecInner`](crate::vec::VecInner) struct
that's generic on [`Storage`](#storage), and two type aliases for convenience:

- [`Vec<T, N>`](crate::vec::Vec) = `VecInner<T, OwnedStorage<N>>`
- [`VecView<T>`](crate::vec::VecView) = `VecInner<T, ViewStorage>`

`Vec` can be unsized into `VecView`, either by unsizing coercions such as `&mut Vec -> &mut VecView` or
`Box<Vec> -> Box<VecView>`, or explicitly with [`.as_view()`](crate::vec::Vec::as_view) or [`.as_mut_view()`](crate::vec::Vec::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.

#### Implementors

- [`OwnedStorage`](#ownedstorage)
- [`ViewStorage`](#viewstorage)

