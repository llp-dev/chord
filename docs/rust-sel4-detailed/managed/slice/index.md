*[managed](../index.md) / [slice](index.md)*

---

# Module `slice`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ManagedSlice`](#managedslice) | enum | A managed slice. |
| [`from_unboxed_slice!`](#from-unboxed-slice) | macro |  |

## Enums

### `ManagedSlice<'a, T: 'a>`

```rust
enum ManagedSlice<'a, T: 'a> {
    Borrowed(&'a mut [T]),
    Owned(alloc::vec::Vec<T>),
}
```

A managed slice.

This enum can be used to represent exclusive access to slices of objects.
In Rust, exclusive access to an object is obtained by either owning the object,
or owning a mutable pointer to the object; hence, "managed".

The purpose of this enum is providing good ergonomics with `std` present while making
it possible to avoid having a heap at all (which of course means that `std` is not present).
To achieve this, the variants other than `Borrowed` are only available when the corresponding
feature is opted in.

A function that requires a managed object should be generic over an `Into<ManagedSlice<'a, T>>`
argument; then, it will be possible to pass either a `Vec<T>`, or a `&'a mut [T]`
without any conversion at the call site.

See also [Managed](#managed).

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl<T> Debug for ManagedSlice<'a, T>`

- <span id="managedslice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: 'a> Deref for ManagedSlice<'a, T>`

- <span id="managedslice-deref-type-target"></span>`type Target = [T]`

- <span id="managedslice-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: 'a> DerefMut for ManagedSlice<'a, T>`

- <span id="managedslice-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> Receiver for ManagedSlice<'a, T>`

- <span id="managedslice-receiver-type-target"></span>`type Target = T`

## Macros

### `from_unboxed_slice!`

