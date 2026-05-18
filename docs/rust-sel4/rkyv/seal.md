**rkyv > seal**

# Module: seal

## Contents

**Structs**

- [`Seal`](#seal) - A mutable reference which may not be moved or assigned.

---

## rkyv::seal::Seal

*Struct*

A mutable reference which may not be moved or assigned.

A `Seal` restricts a mutable reference so that the referenced value cannot
be moved or assigned unless it is `Unpin` and `NoUndef`. These properties
allow the safe use of mutable archived values.

Unlike `Pin`, all fields of `Seal`ed values are also sealed. There is no
notion of "structural sealing" as there is structural pinning. This has the
upside that a `Seal` can be uniformly destructured with `munge`, which is
the recommended replacement for `Pin`'s `map_unchecked_mut` function. Also
unlike `Pin`, `Seal`ing a reference does not require upholding the invariant
that the sealed value is dropped before its backing memory is reused. This
means that creating a `Seal` from a mutable reference is completely safe to
do.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn new(inner: &'a  mut T) -> Self` - Returns a new `Seal` wrapping the given reference.
- `fn unseal(self: Self) -> &'a  mut T` - Returns the underlying reference for types that implement `NoUndef`
- `fn unseal_ref(self: Self) -> &'a T` - Returns the underlying reference as shared for types that implement
- `fn unseal_unchecked(self: Self) -> &'a  mut T` - Returns the underlying reference.
- `fn as_mut(self: & mut Self) -> Seal<T>` - Mutably reborrows the `Seal`.
- `fn index<I>(self: Self, index: I) -> Seal<'a, <I as SliceIndex>::Output>` - Indexes the `Seal`.

**Trait Implementations:**

- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **AsRef**
  - `fn as_ref(self: &Self) -> &T`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Destructure**
  - `fn underlying(self: & mut Self) -> *mut <Self as >::Underlying`
- **Restructure**
  - `fn restructure(self: &Self, ptr: *mut U) -> <Self as >::Restructured`



