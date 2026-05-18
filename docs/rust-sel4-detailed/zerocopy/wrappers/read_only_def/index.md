*[zerocopy](../../index.md) / [wrappers](../index.md) / [read_only_def](index.md)*

---

# Module `read_only_def`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReadOnly`](#readonly) | struct | A read-only wrapper. |

## Structs

### `ReadOnly<T: ?Sized>`

```rust
struct ReadOnly<T: ?Sized> {
    inner: T,
}
```

A read-only wrapper.

A `ReadOnly<T>` disables any interior mutability in `T`, ensuring that
a `&ReadOnly<T>` is genuinely read-only. Thus, `ReadOnly<T>` is
[`Immutable`](../../index.md) regardless of whether `T` is.

Note that `&mut ReadOnly<T>` still permits mutation – the read-only
property only applies to shared references.


#### Implementations

- <span id="readonly-new"></span>`const fn new(t: T) -> ReadOnly<T>` — [`ReadOnly`](#readonly)

  Creates a new `ReadOnly`.

- <span id="readonly-into-inner"></span>`fn into_inner(r: ReadOnly<T>) -> T` — [`ReadOnly`](#readonly)

  Returns the inner value.

#### Trait Implementations

##### `impl<T: ?Sized + Immutable + Debug> Debug for ReadOnly<T>`

- <span id="readonly-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Immutable> Deref for ReadOnly<T>`

- <span id="readonly-deref-type-target"></span>`type Target = T`

- <span id="readonly-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: ?Sized + Immutable> DerefMut for ReadOnly<T>`

- <span id="readonly-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T: ?Sized + FromBytes> FromBytes for ReadOnly<T>`

##### `impl<T: ?Sized + FromZeros> FromZeros for ReadOnly<T>`

##### `impl<T: ?Sized> Immutable for ReadOnly<T>`

##### `impl<T: ?Sized + IntoBytes> IntoBytes for ReadOnly<T>`

##### `impl<T: ?Sized + KnownLayout> KnownLayout for ReadOnly<T>`

- <span id="readonly-knownlayout-type-pointermetadata"></span>`type PointerMetadata = <T as KnownLayout>::PointerMetadata`

##### `impl<T> Receiver for ReadOnly<T>`

- <span id="readonly-receiver-type-target"></span>`type Target = T`

##### `impl<T: ?Sized + TryFromBytes> TryFromBytes for ReadOnly<T>`

##### `impl<T: ?Sized + Unaligned> Unaligned for ReadOnly<T>`

