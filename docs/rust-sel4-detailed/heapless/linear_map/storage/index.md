*[heapless](../../index.md) / [linear_map](../index.md) / [storage](index.md)*

---

# Module `storage`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LinearMapStorage`](#linearmapstorage) | trait | Trait defining how data for a [`LinearMap`](super::LinearMap) is stored. |
| [`LinearMapStorageSealed`](#linearmapstoragesealed) | trait |  |

## Traits

### `LinearMapStorage<K, V>`

```rust
trait LinearMapStorage<K, V>: LinearMapStorageSealed<K, V> { ... }
```

Trait defining how data for a [`LinearMap`](super::LinearMap) is stored.

There's two implementations available:

- [`OwnedStorage`](../index.md): stores the data in an array whose size is known at compile time.
- [`ViewStorage`](../../string/index.md): stores the data in an unsized slice

This allows [`LinearMap`](../index.md) to be generic over either sized or unsized storage. The [`linear_map`](super)
module contains a [`LinearMapInner`](../index.md) struct that's generic on [`LinearMapStorage`](#linearmapstorage),
and two type aliases for convenience:

- [`LinearMap<N>`](crate::linear_map::LinearMap) = `LinearMapInner<OwnedStorage<u8, N>>`
- [`LinearMapView<T>`](crate::linear_map::LinearMapView) = `LinearMapInner<ViewStorage<u8>>`

`LinearMap` can be unsized into `StrinsgView`, either by unsizing coercions such as `&mut LinearMap -> &mut LinearMapView` or
`Box<LinearMap> -> Box<LinearMapView>`, or explicitly with [`.as_view()`](crate::linear_map::LinearMap::as_view) or [`.as_mut_view()`](crate::linear_map::LinearMap::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.





#### Implementors

- [`OwnedVecStorage`](../../vec/storage/index.md#ownedvecstorage)
- [`ViewVecStorage`](../../vec/storage/index.md#viewvecstorage)

### `LinearMapStorageSealed<K, V>`

```rust
trait LinearMapStorageSealed<K, V>: VecStorage<(K, V)> { ... }
```

#### Required Methods

- `fn as_linear_map_view(this: &LinearMapInner<K, V, Self>) -> &LinearMapView<K, V>`

- `fn as_linear_map_mut_view(this: &mut LinearMapInner<K, V, Self>) -> &mut LinearMapView<K, V>`

#### Implementors

- [`OwnedVecStorage`](../../vec/storage/index.md#ownedvecstorage)
- [`ViewVecStorage`](../../vec/storage/index.md#viewvecstorage)

