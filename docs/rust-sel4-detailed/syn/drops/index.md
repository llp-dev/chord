*[syn](../index.md) / [drops](index.md)*

---

# Module `drops`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoDrop`](#nodrop) | struct |  |
| [`TrivialDrop`](#trivialdrop) | trait |  |

## Structs

### `NoDrop<T: ?Sized>`

```rust
struct NoDrop<T: ?Sized>(core::mem::ManuallyDrop<T>);
```

#### Implementations

- <span id="nodrop-new"></span>`fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T: ?Sized> Deref for NoDrop<T>`

- <span id="nodrop-deref-type-target"></span>`type Target = T`

- <span id="nodrop-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: ?Sized> DerefMut for NoDrop<T>`

- <span id="nodrop-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> Receiver for NoDrop<T>`

- <span id="nodrop-receiver-type-target"></span>`type Target = T`

## Traits

### `TrivialDrop`

```rust
trait TrivialDrop { ... }
```

#### Implementors

- [`PrivateIterMut`](../punctuated/index.md#privateitermut)
- [`PrivateIter`](../punctuated/index.md#privateiter)
- `iter::Empty<T>`
- `option::IntoIter<&T>`
- `option::IntoIter<&mut T>`
- `slice::Iter<'_, T>`
- `slice::IterMut<'_, T>`

