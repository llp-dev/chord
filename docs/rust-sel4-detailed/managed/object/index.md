*[managed](../index.md) / [object](index.md)*

---

# Module `object`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Managed`](#managed) | enum | A managed object. |

## Enums

### `Managed<'a, T: 'a + ?Sized>`

```rust
enum Managed<'a, T: 'a + ?Sized> {
    Borrowed(&'a mut T),
    Owned(alloc::boxed::Box<T>),
}
```

A managed object.

This enum can be used to represent exclusive access to objects. In Rust, exclusive access
to an object is obtained by either owning the object, or owning a mutable pointer
to the object; hence, "managed".

The purpose of this enum is providing good ergonomics with `std` present while making
it possible to avoid having a heap at all (which of course means that `std` is not present).
To achieve this, the variants other than `Borrow` are only available when the corresponding
feature is opted in.

A function that requires a managed object should be generic over an `Into<Managed<'a, T>>`
argument; then, it will be possible to pass either a `Box<T>`, `Vec<T>`, or a `&'a mut T`
without any conversion at the call site.

Note that a `Vec<T>` converted into an `Into<Managed<'a, [T]>>` gets transformed
into a boxed slice, and can no longer be resized. See also
[ManagedSlice](#managedslice), which does not have this drawback.

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl<T> Debug for Managed<'a, T>`

- <span id="managed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: 'a + ?Sized> Deref for Managed<'a, T>`

- <span id="managed-deref-type-target"></span>`type Target = T`

- <span id="managed-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: 'a + ?Sized> DerefMut for Managed<'a, T>`

- <span id="managed-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> Receiver for Managed<'a, T>`

- <span id="managed-receiver-type-target"></span>`type Target = T`

