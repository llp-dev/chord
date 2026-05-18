**heapless > linear_map > storage**

# Module: linear_map::storage

## Contents

**Traits**

- [`LinearMapStorage`](#linearmapstorage) - Trait defining how data for a [`LinearMap`](super::LinearMap) is stored.
- [`LinearMapStorageSealed`](#linearmapstoragesealed)

---

## heapless::linear_map::storage::LinearMapStorage

*Trait*

Trait defining how data for a [`LinearMap`](super::LinearMap) is stored.

There's two implementations available:

- [`OwnedStorage`]: stores the data in an array whose size is known at compile time.
- [`ViewStorage`]: stores the data in an unsized slice

This allows [`LinearMap`] to be generic over either sized or unsized storage. The [`linear_map`](super)
module contains a [`LinearMapInner`] struct that's generic on [`LinearMapStorage`],
and two type aliases for convenience:

- [`LinearMap<N>`](crate::linear_map::LinearMap) = `LinearMapInner<OwnedStorage<u8, N>>`
- [`LinearMapView<T>`](crate::linear_map::LinearMapView) = `LinearMapInner<ViewStorage<u8>>`

`LinearMap` can be unsized into `StrinsgView`, either by unsizing coercions such as `&mut LinearMap -> &mut LinearMapView` or
`Box<LinearMap> -> Box<LinearMapView>`, or explicitly with [`.as_view()`](crate::linear_map::LinearMap::as_view) or [`.as_mut_view()`](crate::linear_map::LinearMap::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.

[`LinearMapInner`]: super::LinearMapInner
[`LinearMap`]: super::LinearMap
[`OwnedStorage`]: super::OwnedStorage
[`ViewStorage`]: super::ViewStorage



## heapless::linear_map::storage::LinearMapStorageSealed

*Trait*

**Methods:**

- `as_linear_map_view`
- `as_linear_map_mut_view`



