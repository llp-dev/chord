**rkyv > option**

# Module: option

## Contents

**Structs**

- [`Iter`](#iter) - An iterator over a reference to the `Some` variant of an `ArchivedOption`.

**Enums**

- [`ArchivedOption`](#archivedoption) - An archived [`Option`].

---

## rkyv::option::ArchivedOption

*Enum*

An archived [`Option`].

It functions identically to [`Option`] but has a different internal
representation to allow for archiving.

**Generic Parameters:**
- T

**Variants:**
- `None` - No value
- `Some(T)` - Some value `T`

**Methods:**

- `fn ok_or<E>(self: Self, err: E) -> Result<T, E>` - Transforms the `ArchivedOption<T>` into a `Result<T, E>`, mapping
- `fn unwrap(self: Self) -> T` - Returns the contained [`Some`] value, consuming the `self` value.
- `fn unwrap_or(self: Self, default: T) -> T` - Returns the contained [`Some`] value or a provided default.
- `fn unwrap_or_else<F>(self: Self, f: F) -> T` - Returns the contained [`Some`] value or computes it from a closure.
- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&T>` - Converts to an `Option<&T>`.
- `fn as_mut(self: & mut Self) -> Option<& mut T>` - Converts to an `Option<&mut T>`.
- `fn as_seal(this: Seal<Self>) -> Option<Seal<T>>` - Converts from `Seal<'_, ArchivedOption<T>>` to `Option<Seal<'_, T>>`.
- `fn iter(self: &Self) -> Iter<&T>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut T>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<T>>` - Returns an iterator over the sealed possibly-contained value.
- `fn get_or_insert(self: & mut Self, v: T) -> & mut T` - Inserts `v` into the option if it is `None`, then returns a mutable
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut T` - Inserts a value computed from `f` into the option if it is `None`, then
- `fn as_deref_mut(self: & mut Self) -> Option<& mut <T as Deref>::Target>` - Converts from `&mut ArchivedOption<T>` to `Option<&mut T::Target>`.
- `fn as_deref(self: &Self) -> Option<&<T as Deref>::Target>` - Converts from `&ArchivedOption<T>` to `Option<&T::Target>`.

**Traits:** Copy, Portable, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ArchivedOption<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Option<T>) -> Option<cmp::Ordering>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **From**
  - `fn from(val: T) -> ArchivedOption<T>` - Moves `val` into a new [`Some`].
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<Option<T>, <D as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &Option<T>) -> bool`



## rkyv::option::Iter

*Struct*

An iterator over a reference to the `Some` variant of an `ArchivedOption`.

This iterator yields one value if the `ArchivedOption` is a `Some`,
otherwise none.

**Generic Parameters:**
- P

**Methods:**

- `fn new(inner: Option<P>) -> Self` - Returns an iterator over the given `Option`.

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



