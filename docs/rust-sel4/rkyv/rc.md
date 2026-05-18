**rkyv > rc**

# Module: rc

## Contents

**Structs**

- [`ArcFlavor`](#arcflavor) - The flavor type for [`Arc`](crate::alloc::sync::Arc).
- [`ArchivedRc`](#archivedrc) - An archived `Rc`.
- [`ArchivedRcWeak`](#archivedrcweak) - An archived `rc::Weak`.
- [`RcFlavor`](#rcflavor) - The flavor type for [`Rc`](crate::alloc::rc::Rc).
- [`RcResolver`](#rcresolver) - The resolver for `Rc`.
- [`RcWeakResolver`](#rcweakresolver) - The resolver for `rc::Weak`.

**Traits**

- [`Flavor`](#flavor) - A type marker for `ArchivedRc`.

---

## rkyv::rc::ArcFlavor

*Struct*

The flavor type for [`Arc`](crate::alloc::sync::Arc).

**Unit Struct**

**Traits:** Flavor



## rkyv::rc::ArchivedRc

*Struct*

An archived `Rc`.

This is a thin wrapper around a [`RelPtr`] to the archived type paired with
a "flavor" type. Because there may be many varieties of shared pointers and
they may not be used together, the flavor helps check that memory is not
being shared incorrectly during validation.

**Generic Parameters:**
- T
- F

**Methods:**

- `fn get(self: &Self) -> &T` - Gets the value of the `ArchivedRc`.
- `fn get_seal_unchecked(this: Seal<Self>) -> Seal<T>` - Gets the sealed value of this `ArchivedRc`.
- `fn resolve_from_ref<U>(value: &U, resolver: RcResolver, out: Place<Self>)` - Resolves an archived `Rc` from a given reference.
- `fn serialize_from_ref<U, S>(value: &U, serializer: & mut S) -> Result<RcResolver, <S as >::Error>` - Serializes an archived `Rc` from a given reference.

**Traits:** Portable, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &rc::Rc<U>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedRc<U, UF>) -> bool`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as >::Error>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<rc::Rc<T>, <D as >::Error>`
- **Borrow**
  - `fn borrow(self: &Self) -> &T`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **Pointer**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &sync::Arc<U>) -> bool`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &T`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedRc<U, UF>) -> Option<cmp::Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<sync::Arc<T>, <D as >::Error>`



## rkyv::rc::ArchivedRcWeak

*Struct*

An archived `rc::Weak`.

This is essentially just an optional [`ArchivedRc`].

**Generic Parameters:**
- T
- F

**Methods:**

- `fn upgrade(self: &Self) -> Option<&ArchivedRc<T, F>>` - Attempts to upgrade the weak pointer to an `ArchivedArc`.
- `fn upgrade_seal(this: Seal<Self>) -> Option<Seal<ArchivedRc<T, F>>>` - Attempts to upgrade a sealed weak pointer.
- `fn resolve_from_ref<U>(value: Option<&U>, resolver: RcWeakResolver, out: Place<Self>)` - Resolves an archived `Weak` from a given optional reference.
- `fn serialize_from_ref<U, S>(value: Option<&U>, serializer: & mut S) -> Result<RcWeakResolver, <S as >::Error>` - Serializes an archived `Weak` from a given optional reference.

**Traits:** Portable

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<sync::Weak<T>, <D as >::Error>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<rc::Weak<T>, <D as >::Error>`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as >::Error>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rkyv::rc::Flavor

*Trait*

A type marker for `ArchivedRc`.

**Methods:**

- `ALLOW_CYCLES`: If `true`, cyclic `ArchivedRc`s with this flavor will not fail



## rkyv::rc::RcFlavor

*Struct*

The flavor type for [`Rc`](crate::alloc::rc::Rc).

**Unit Struct**

**Traits:** Flavor



## rkyv::rc::RcResolver

*Struct*

The resolver for `Rc`.

**Methods:**

- `fn from_pos(pos: usize) -> Self` - Creates a new [`RcResolver`] from the position of a serialized value.



## rkyv::rc::RcWeakResolver

*Struct*

The resolver for `rc::Weak`.



