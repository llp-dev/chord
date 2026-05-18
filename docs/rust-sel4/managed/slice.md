**managed > slice**

# Module: slice

## Contents

**Enums**

- [`ManagedSlice`](#managedslice) - A managed slice.

---

## managed::slice::ManagedSlice

*Enum*

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

See also [Managed](enum.Managed.html).

**Generic Parameters:**
- 'a
- T

**Variants:**
- `Borrowed(&'a  mut [T])` - Borrowed variant.
- `Owned(alloc::vec::Vec<T>)` - Owned variant, only available with the `std` or `alloc` feature enabled.

**Trait Implementations:**

- **From**
  - `fn from(value: [T; 28]) -> Self`
- **From**
  - `fn from(value: [T; 15]) -> Self`
- **From**
  - `fn from(value: [T; 2]) -> Self`
- **From**
  - `fn from(value: [T; 27]) -> Self`
- **From**
  - `fn from(value: [T; 14]) -> Self`
- **From**
  - `fn from(value: [T; 1]) -> Self`
- **From**
  - `fn from(value: [T; 26]) -> Self`
- **From**
  - `fn from(value: [T; 13]) -> Self`
- **From**
  - `fn from(value: [T; 0]) -> Self`
- **From**
  - `fn from(value: [T; 25]) -> Self`
- **From**
  - `fn from(value: [T; 12]) -> Self`
- **From**
  - `fn from(value: &'a  mut [T]) -> Self`
- **From**
  - `fn from(value: [T; 24]) -> Self`
- **From**
  - `fn from(value: [T; 11]) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(value: [T; 23]) -> Self`
- **From**
  - `fn from(value: [T; 10]) -> Self`
- **From**
  - `fn from(value: [T; 22]) -> Self`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **From**
  - `fn from(value: [T; 9]) -> Self`
- **From**
  - `fn from(value: [T; 21]) -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **From**
  - `fn from(value: [T; 8]) -> Self`
- **From**
  - `fn from(value: [T; 20]) -> Self`
- **From**
  - `fn from(value: [T; 7]) -> Self`
- **From**
  - `fn from(value: Vec<T>) -> Self`
- **From**
  - `fn from(value: [T; 19]) -> Self`
- **From**
  - `fn from(value: [T; 6]) -> Self`
- **From**
  - `fn from(value: [T; 31]) -> Self`
- **From**
  - `fn from(value: [T; 18]) -> Self`
- **From**
  - `fn from(value: [T; 5]) -> Self`
- **From**
  - `fn from(value: [T; 30]) -> Self`
- **From**
  - `fn from(value: [T; 17]) -> Self`
- **From**
  - `fn from(value: [T; 4]) -> Self`
- **From**
  - `fn from(value: [T; 29]) -> Self`
- **From**
  - `fn from(value: [T; 16]) -> Self`
- **From**
  - `fn from(value: [T; 3]) -> Self`



