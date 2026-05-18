**rkyv > boxed**

# Module: boxed

## Contents

**Structs**

- [`ArchivedBox`](#archivedbox) - An archived [`Box`].
- [`BoxResolver`](#boxresolver) - The resolver for `Box`.

---

## rkyv::boxed::ArchivedBox

*Struct*

An archived [`Box`].

This is a thin `#[repr(transparent)]` wrapper around a [`RelPtr`] to the
archived type.

**Generic Parameters:**
- T

**Methods:**

- `fn get(self: &Self) -> &T` - Returns a reference to the value of this archived box.
- `fn get_seal(this: Seal<Self>) -> Seal<T>` - Returns a sealed mutable reference to the value of this archived box.
- `fn resolve_from_ref<U>(value: &U, resolver: BoxResolver, out: Place<Self>)` - Resolves an archived box from the given value and parameters.
- `fn serialize_from_ref<U, S>(value: &U, serializer: & mut S) -> Result<BoxResolver, <S as >::Error>` - Serializes an archived box from the given value and serializer.
- `fn resolve_from_raw_parts(resolver: BoxResolver, metadata: <T as >::ArchivedMetadata, out: Place<Self>)` - Resolves an archived box from a [`BoxResolver`] and the raw metadata

**Traits:** Eq, Portable

**Trait Implementations:**

- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **Pointer**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as >::Error>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &T`
- **Borrow**
  - `fn borrow(self: &Self) -> &T`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Box<U>) -> Option<cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedBox<U>) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<Box<T>, <D as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Box<U>) -> bool`



## rkyv::boxed::BoxResolver

*Struct*

The resolver for `Box`.

**Methods:**

- `fn from_pos(pos: usize) -> Self` - Creates a new [`BoxResolver`] from the position of a serialized value.



