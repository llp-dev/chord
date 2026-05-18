**rkyv > niche > niched_option**

# Module: niche::niched_option

## Contents

**Structs**

- [`NichedOption`](#nichedoption) - A niched `ArchivedOption<T>`.

**Type Aliases**

- [`Iter`](#iter) - An iterator over a reference to the `Some` variant of a `NichedOption`.

---

## rkyv::niche::niched_option::Iter

*Type Alias*: `crate::option::Iter<P>`

An iterator over a reference to the `Some` variant of a `NichedOption`.

This iterator yields one value if the `NichedOption` is a `Some`, otherwise
none.



## rkyv::niche::niched_option::NichedOption

*Struct*

A niched `ArchivedOption<T>`.

It has the same layout as `T`, and thus uses less space by storing the
`None` variant in a custom way based on `N`.

**Generic Parameters:**
- T
- N

**Methods:**

- `fn as_deref(self: &Self) -> Option<&<T as Deref>::Target>` - Converts from `&NichedOption<T, N>` to `Option<&T::Target>`.
- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&T>` - Converts to an `Option<&T>`.
- `fn as_mut(self: & mut Self) -> Option<& mut T>` - Converts to an `Option<&mut T>`.
- `fn as_seal(this: Seal<Self>) -> Option<Seal<T>>` - Converts from `Seal<'_, NichedOption<T, N>>` to `Option<Seal<'_, T>>`.
- `fn iter(self: &Self) -> Iter<&T>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut T>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<T>>` - Returns an iterator over the sealed possibly-contained value.
- `fn resolve_from_option<U>(option: Option<&U>, resolver: Option<<U as >::Resolver>, out: Place<Self>)` - Resolves a `NichedOption<U::Archived, N>` from an `Option<&U>`.
- `fn serialize_from_option<U, S>(option: Option<&U>, serializer: & mut S) -> Result<Option<<U as >::Resolver>, <S as >::Error>` - Serializes a `NichedOption<U::Archived, N>` from an `Option<&U>`.

**Traits:** Portable, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Option<Rhs>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<Option<T>, <D as >::Error>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut C) -> Result<(), <C as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`



