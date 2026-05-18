*[rancor](../index.md) / [thin_box](index.md)*

---

# Module `thin_box`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ThinBox`](#thinbox) | struct |  |

## Structs

### `ThinBox<T: Pointee + ?Sized>`

```rust
struct ThinBox<T: Pointee + ?Sized> {
    ptr: core::ptr::NonNull<()>,
    _phantom: core::marker::PhantomData<T>,
}
```

#### Implementations

- <span id="thinbox-layout-for"></span>`fn layout_for(value_layout: Layout) -> (Layout, usize)`

- <span id="thinbox-new-unchecked"></span>`unsafe fn new_unchecked<U, F>(value: U, cast: F) -> Self`

  # Safety

  

  `cast` must return the same pointer _unsized_ to a pointer to `T`.

- <span id="thinbox-as-ptr"></span>`fn as_ptr(&self) -> *mut T`

#### Trait Implementations

##### `impl<T: fmt::Debug + Pointee + ?Sized> Debug for ThinBox<T>`

- <span id="thinbox-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Pointee + ?Sized> Deref for ThinBox<T>`

- <span id="thinbox-deref-type-target"></span>`type Target = T`

- <span id="thinbox-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: Pointee + ?Sized> DerefMut for ThinBox<T>`

- <span id="thinbox-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T: fmt::Display + Pointee + ?Sized> Display for ThinBox<T>`

- <span id="thinbox-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Pointee + ?Sized> Drop for ThinBox<T>`

- <span id="thinbox-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointee for ThinBox<T>`

- <span id="thinbox-pointee-type-metadata"></span>`type Metadata = ()`

##### `impl<T> Receiver for ThinBox<T>`

- <span id="thinbox-receiver-type-target"></span>`type Target = T`

##### `impl<T: Pointee + Send + ?Sized> Send for ThinBox<T>`

##### `impl<T: Pointee + Sync + ?Sized> Sync for ThinBox<T>`

##### `impl<T> ToString for ThinBox<T>`

- <span id="thinbox-tostring-to-string"></span>`fn to_string(&self) -> String`

