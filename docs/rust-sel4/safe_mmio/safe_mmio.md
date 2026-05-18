**safe_mmio**

# Module: safe_mmio

## Contents

**Modules**

- [`fields`](#fields) - Wrapper types for MMIO fields.

**Macros**

- [`field`](#field) - Gets a `UniqueMmioPointer` to a field of a type wrapped in a `UniqueMmioPointer`.
- [`field_shared`](#field_shared) - Gets a `SharedMmioPointer` to a field of a type wrapped in a `SharedMmioPointer`.
- [`split_fields`](#split_fields) - Gets `UniqueMmioPointer`s to several fields of a type wrapped in a `UniqueMmioPointer`.

**Structs**

- [`SharedMmioPointer`](#sharedmmiopointer) - A shared pointer to the registers of some MMIO device.
- [`SharedMmioPointerIterator`](#sharedmmiopointeriterator) - Iterator over a `SharedMmioPointer` slice, yielding pointers to items.
- [`UniqueMmioPointer`](#uniquemmiopointer) - A unique owned pointer to the registers of some MMIO device.
- [`UniqueMmioPointerIterator`](#uniquemmiopointeriterator) - Iterator over a `UniqueMmioPointer` slice, yielding pointers to items.

---

## safe_mmio::SharedMmioPointer

*Struct*

A shared pointer to the registers of some MMIO device.

It is guaranteed to be valid but unlike [`UniqueMmioPointer`] may not be unique.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn read_unsafe(self: &Self) -> T` - Performs an MMIO read of the entire `T`.
- `fn split(self: Self) -> [SharedMmioPointer<'a, T>; LEN]` - Splits a `SharedMmioPointer` to an array into an array of `SharedMmioPointer`s.
- `fn as_slice(self: &Self) -> SharedMmioPointer<'a, [T]>` - Converts this array pointer to an equivalent slice pointer.
- `fn get(self: &Self, index: usize) -> Option<SharedMmioPointer<'a, T>>` - Returns a `SharedMmioPointer` to an element of this array, or `None` if the index is out of
- `fn get_range(self: &Self, range: Range<usize>) -> Option<SharedMmioPointer<[T]>>` - Returns a `SharedMmioPointer` to a range of elements of this array, or `None` if the range
- `fn iter(self: &Self) -> SharedMmioPointerIterator<T>` - Returns a new iterator of the items of the array.
- `fn split_some<const N>(self: Self, chosen: [usize; N]) -> [UniqueMmioPointer<'a, T>; N]` - Splits a `UniqueMmioPointer` to a slice into an array of `UniqueMmioPointer`s, taking only
- `fn get(self: &Self, index: usize) -> Option<SharedMmioPointer<'a, T>>` - Returns a `SharedMmioPointer` to an element of this slice, or `None` if the index is out of
- `fn get_range(self: &Self, range: Range<usize>) -> Option<SharedMmioPointer<[T]>>` - Returns a `SharedMmioPointer` to a range of elements of this slice, or `None` if the range
- `fn iter(self: &Self) -> SharedMmioPointerIterator<T>` - Returns a new iterator of the items of the slice.
- `fn len(self: &Self) -> usize` - Returns the length of the slice.
- `fn is_empty(self: &Self) -> bool` - Returns whether the slice is empty.
- `fn read(self: &Self) -> T` - Performs an MMIO read of the entire `T`.
- `fn read(self: &Self) -> T` - Performs an MMIO read of the entire `T`.
- `fn child<U>(self: &Self, regs: NonNull<U>) -> SharedMmioPointer<'a, U>` - Creates a new `SharedMmioPointer` with the same lifetime as this one.
- `fn ptr(self: &Self) -> *const T` - Returns a raw const pointer to the MMIO registers.

**Traits:** Send, Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: SharedMmioPointer<'a, T>) -> Self`
- **From**
  - `fn from(value: SharedMmioPointer<'a, [T; LEN]>) -> Self`
- **From**
  - `fn from(r: &'a T) -> Self`
- **From**
  - `fn from(unique: UniqueMmioPointer<'a, T>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **From**
  - `fn from(value: SharedMmioPointer<'a, T>) -> Self`



## safe_mmio::SharedMmioPointerIterator

*Struct*

Iterator over a `SharedMmioPointer` slice, yielding pointers to items.

This iterator advances by creating a head pointer and shortening the
remaining tail.

**Generic Parameters:**
- 'a
- T

**Traits:** Copy

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SharedMmioPointerIterator<'a, T>`



## safe_mmio::UniqueMmioPointer

*Struct*

A unique owned pointer to the registers of some MMIO device.

It is guaranteed to be valid and unique; no other access to the MMIO space of the device may
happen for the lifetime `'a`.

A `UniqueMmioPointer` may be created from a mutable reference, but this should only be used for
testing purposes, as references should never be constructed for real MMIO address space.

**Generic Parameters:**
- 'a
- T

**Tuple Struct**: `()`

**Methods:**

- `fn modify<impl FnOnce(T) -> T>(self: & mut Self, f: impl Trait)` - Performs an MMIO read of the entire `T`, applies the given function to it, and then performs
- `fn modify_mut<impl FnOnce(&mut T)>(self: & mut Self, f: impl Trait)` - Performs an MMIO read of the entire `T`, calls the given function to modify it, and then
- `fn write(self: & mut Self, value: T)` - Performs an MMIO write of the entire `T`.
- `fn modify<impl FnOnce(T) -> T>(self: & mut Self, f: impl Trait)` - Performs an MMIO read of the entire `T`, applies the given function to it, and then performs
- `fn modify_mut<impl FnOnce(&mut T)>(self: & mut Self, f: impl Trait)` - Performs an MMIO read of the entire `T`, calls the given function to modify it, and then
- `fn split(self: Self) -> [UniqueMmioPointer<'a, T>; LEN]` - Splits a `UniqueMmioPointer` to an array into an array of `UniqueMmioPointer`s.
- `fn split_some<const N>(self: Self, chosen: [usize; N]) -> [UniqueMmioPointer<'a, T>; N]` - Splits a `UniqueMmioPointer` to an array into an array of `UniqueMmioPointer`s, taking only
- `fn as_mut_slice(self: & mut Self) -> UniqueMmioPointer<[T]>` - Converts this array pointer to an equivalent slice pointer.
- `fn get(self: & mut Self, index: usize) -> Option<UniqueMmioPointer<T>>` - Returns a `UniqueMmioPointer` to an element of this array, or `None` if the index is out of
- `fn get_range(self: & mut Self, range: Range<usize>) -> Option<UniqueMmioPointer<[T]>>` - Returns a `UniqueMmioPointer` to a range of elements of this array, or `None` if the range
- `fn iter(self: & mut Self) -> UniqueMmioPointerIterator<T>` - Returns a new iterator to the items of the array.
- `fn take(self: Self, index: usize) -> Option<UniqueMmioPointer<'a, T>>` - Returns a `UniqueMmioPointer` to an element of this array, or `None` if the index is out of
- `fn write(self: & mut Self, value: T)` - Performs an MMIO write of the entire `T`.
- `fn read(self: & mut Self) -> T` - Performs an MMIO read of the entire `T`.
- `fn split_child<U>(self: & mut Self, regs: NonNull<U>) -> UniqueMmioPointer<'a, U>` - Creates a new `UniqueMmioPointer` with the same lifetime as this one, but not tied to the
- `fn get(self: & mut Self, index: usize) -> Option<UniqueMmioPointer<T>>` - Returns a `UniqueMmioPointer` to an element of this slice, or `None` if the index is out of
- `fn get_range(self: & mut Self, range: Range<usize>) -> Option<UniqueMmioPointer<[T]>>` - Returns a `UniqueMmioPointer` to a range of elements of this slice, or `None` if the range
- `fn iter(self: & mut Self) -> UniqueMmioPointerIterator<T>` - Returns a new iterator of the items of the slice.
- `fn take(self: Self, index: usize) -> Option<UniqueMmioPointer<'a, T>>` - Returns a `UniqueMmioPointer` to an element of this slice, or `None` if the index is out of
- `fn new(regs: NonNull<T>) -> Self` - Creates a new `UniqueMmioPointer` from a non-null raw pointer.
- `fn child<U>(self: & mut Self, regs: NonNull<U>) -> UniqueMmioPointer<U>` - Creates a new `UniqueMmioPointer` with the same lifetime as this one.
- `fn ptr_mut(self: & mut Self) -> *mut T` - Returns a raw mut pointer to the MMIO registers.
- `fn ptr_nonnull(self: & mut Self) -> NonNull<T>` - Returns a `NonNull<T>` pointer to the MMIO registers.
- `fn reborrow(self: & mut Self) -> UniqueMmioPointer<T>` - Returns a new `UniqueMmioPointer` with a lifetime no greater than this one.
- `fn write(self: & mut Self, value: T)` - Performs an MMIO write of the entire `T`.
- `fn read(self: & mut Self) -> T` - Performs an MMIO read of the entire `T`.
- `fn read_unsafe(self: & mut Self) -> T` - Performs an MMIO read of the entire `T`.
- `fn write_unsafe(self: & mut Self, value: T)` - Performs an MMIO write of the entire `T`.

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(value: UniqueMmioPointer<'a, T>) -> Self`
- **From**
  - `fn from(value: UniqueMmioPointer<'a, T>) -> Self`
- **From**
  - `fn from(value: UniqueMmioPointer<'a, [T; LEN]>) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **From**
  - `fn from(r: &'a  mut T) -> Self`



## safe_mmio::UniqueMmioPointerIterator

*Struct*

Iterator over a `UniqueMmioPointer` slice, yielding pointers to items.

This iterator advances by splitting off the head element and shortening the
remaining tail.

**Generic Parameters:**
- 'a
- T

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## safe_mmio::field

*Declarative Macro*

Gets a `UniqueMmioPointer` to a field of a type wrapped in a `UniqueMmioPointer`.

```rust
macro_rules! field {
    ($mmio_pointer:expr, $field:ident) => { ... };
}
```



## safe_mmio::field_shared

*Declarative Macro*

Gets a `SharedMmioPointer` to a field of a type wrapped in a `SharedMmioPointer`.

```rust
macro_rules! field_shared {
    ($mmio_pointer:expr, $field:ident) => { ... };
}
```



## Module: fields

Wrapper types for MMIO fields.



## safe_mmio::split_fields

*Declarative Macro*

Gets `UniqueMmioPointer`s to several fields of a type wrapped in a `UniqueMmioPointer`.

# Safety

The same field name must not be passed more than once.

```rust
macro_rules! split_fields {
    ($mmio_pointer:expr, $( $field:ident ),+) => { ... };
}
```



