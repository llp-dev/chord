**rkyv > niche > option_box**

# Module: niche::option_box

## Contents

**Structs**

- [`ArchivedOptionBox`](#archivedoptionbox) - A niched archived `Option<Box<T>>`.

**Enums**

- [`OptionBoxResolver`](#optionboxresolver) - The resolver for [`ArchivedOptionBox`].

**Type Aliases**

- [`Iter`](#iter) - An iterator over a reference to the `Some` variant of an

---

## rkyv::niche::option_box::ArchivedOptionBox

*Struct*

A niched archived `Option<Box<T>>`.

It uses less space by storing the `None` variant as a null pointer.

**Generic Parameters:**
- T

**Methods:**

- `fn resolve_from_option<U>(field: Option<&U>, resolver: OptionBoxResolver, out: Place<Self>)` - Resolves an `ArchivedOptionBox<T::Archived>` from an `Option<&T>`.
- `fn serialize_from_option<U, S>(field: Option<&U>, serializer: & mut S) -> Result<OptionBoxResolver, <S as >::Error>` - Serializes an `ArchivedOptionBox<T::Archived>` from an `Option<&T>`.
- `fn is_none(self: &Self) -> bool` - Returns `true` if the option box is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option box is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&ArchivedBox<T>>` - Converts to an `Option<&ArchivedBox<T>>`.
- `fn as_mut(self: & mut Self) -> Option<& mut ArchivedBox<T>>` - Converts to an `Option<&mut ArchivedBox<T>>`.
- `fn as_seal(this: Seal<Self>) -> Option<Seal<ArchivedBox<T>>>` - Converts from `Seal<'_, ArchivedOption<T>>` to `Option<Seal<'_,
- `fn iter(self: &Self) -> Iter<&ArchivedBox<T>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut ArchivedBox<T>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<ArchivedBox<T>>>` - Returns an iterator over the sealed possibly-contained value.
- `fn as_deref(self: &Self) -> Option<&T>` - Converts from `&ArchivedOptionBox<T>` to `Option<&T>`.

**Traits:** Portable, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Option<Box<T>>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`



## rkyv::niche::option_box::Iter

*Type Alias*: `crate::option::Iter<P>`

An iterator over a reference to the `Some` variant of an
`ArchivedOptionBox`.

This iterator yields one value if the `ArchivedOptionBox` is a `Some`,
otherwise none.



## rkyv::niche::option_box::OptionBoxResolver

*Enum*

The resolver for [`ArchivedOptionBox`].

**Variants:**
- `None` - The `ArchivedOptionBox` was `None`
- `Some(crate::boxed::BoxResolver)` - The resolver for the `ArchivedBox`



