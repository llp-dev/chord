**managed > object**

# Module: object

## Contents

**Enums**

- [`Managed`](#managed) - A managed object.

---

## managed::object::Managed

*Enum*

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
[ManagedSlice](enum.ManagedSlice.html), which does not have this drawback.

**Generic Parameters:**
- 'a
- T

**Variants:**
- `Borrowed(&'a  mut T)` - Borrowed variant.
- `Owned(alloc::boxed::Box<T>)` - Owned variant, only available with the `std` or `alloc` feature enabled.

**Trait Implementations:**

- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **From**
  - `fn from(value: Vec<T>) -> Self`
- **From**
  - `fn from(value: Box<T>) -> Self`
- **From**
  - `fn from(value: &'a  mut T) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



