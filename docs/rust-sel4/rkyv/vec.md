**rkyv > vec**

# Module: vec

## Contents

**Structs**

- [`ArchivedVec`](#archivedvec) - An archived [`Vec`].
- [`VecResolver`](#vecresolver) - The resolver for [`ArchivedVec`].

---

## rkyv::vec::ArchivedVec

*Struct*

An archived [`Vec`].

This uses a [`RelPtr`] to a `[T]` under the hood. Unlike
[`ArchivedString`](crate::string::ArchivedString), it does not have an
inline representation.

**Generic Parameters:**
- T

**Methods:**

- `fn as_ptr(self: &Self) -> *const T` - Returns a pointer to the first element of the archived vec.
- `fn len(self: &Self) -> usize` - Returns the number of elements in the archived vec.
- `fn is_empty(self: &Self) -> bool` - Returns whether the archived vec is empty.
- `fn as_slice(self: &Self) -> &[T]` - Gets the elements of the archived vec as a slice.
- `fn as_slice_seal(this: Seal<Self>) -> Seal<[T]>` - Gets the elements of the archived vec as a sealed mutable slice.
- `fn resolve_from_slice<U>(slice: &[U], resolver: VecResolver, out: Place<Self>)` - Resolves an archived `Vec` from a given slice.
- `fn resolve_from_len(len: usize, resolver: VecResolver, out: Place<Self>)` - Resolves an archived `Vec` from a given length.
- `fn serialize_from_slice<U, S>(slice: &[U], serializer: & mut S) -> Result<VecResolver, <S as >::Error>` - Serializes an archived `Vec` from a given slice.
- `fn serialize_from_iter<U, I, S>(iter: I, serializer: & mut S) -> Result<VecResolver, <S as >::Error>` - Serializes an archived `Vec` from a given iterator.
- `fn serialize_from_unknown_length_iter<B, I, S>(iter: & mut I, serializer: & mut S) -> Result<VecResolver, <S as >::Error>` - Serializes an archived `Vec` from a given iterator. Compared to

**Traits:** Portable, Eq

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Index**
  - `fn index(self: &Self, index: I) -> &<Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &VecDeque<U>) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &[U; N]) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Vec<U>) -> Option<::core::cmp::Ordering>`
- **Borrow**
  - `fn borrow(self: &Self) -> &[T]`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedVec<U>) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &Vec<U>) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<Vec<T>, <D as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &[U]) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &[T]) -> Option<cmp::Ordering>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[T]`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &VecDeque<T>) -> Option<Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<VecDeque<T>, <D as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedVec<T>) -> Option<cmp::Ordering>`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as >::Error>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rkyv::vec::VecResolver

*Struct*

The resolver for [`ArchivedVec`].

**Methods:**

- `fn from_pos(pos: usize) -> Self` - Creates a new `VecResolver` from a position in the output buffer where



