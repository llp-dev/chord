**rkyv > result**

# Module: result

## Contents

**Enums**

- [`ArchivedResult`](#archivedresult) - An archived [`Result`] that represents either success

**Type Aliases**

- [`Iter`](#iter) - An iterator over a reference to the `Ok` variant of an [`ArchivedResult`].

---

## rkyv::result::ArchivedResult

*Enum*

An archived [`Result`] that represents either success
([`Ok`](ArchivedResult::Ok)) or failure ([`Err`](ArchivedResult::Err)).

**Generic Parameters:**
- T
- E

**Variants:**
- `Ok(T)` - Contains the success value
- `Err(E)` - Contains the error value

**Methods:**

- `fn ok(self: Self) -> Option<T>` - Converts from `ArchivedResult<T, E>` to `Option<T>`.
- `fn unwrap(self: Self) -> T` - Returns the contained [`Ok`](ArchivedResult::Ok) value, consuming the
- `fn unwrap_or_else<F>(self: Self, op: F) -> T` - Returns the contained `Ok` value or computes it from a closure.
- `fn is_ok(self: &Self) -> bool` - Returns `true` if the result is [`Ok`](ArchivedResult::Ok).
- `fn is_err(self: &Self) -> bool` - Returns `true` if the result is [`Err`](ArchivedResult::Err).
- `fn as_ref(self: &Self) -> Result<&T, &E>` - Returns a `Result` containing the success and error values of this
- `fn as_mut(self: & mut Self) -> Result<& mut T, & mut E>` - Converts from `&mut ArchivedResult<T, E>` to `Result<&mut T, &mut E>`.
- `fn as_seal(this: Seal<Self>) -> Result<Seal<T>, Seal<E>>` - Converts from `Seal<'_, ArchivedResult<T, E>>` to
- `fn iter(self: &Self) -> Iter<&T>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut T>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<T>>` - Returns an iterator over the sealed possibly-contained value.
- `fn as_deref_mut(self: & mut Self) -> Result<& mut <T as Deref>::Target, & mut E>` - Converts from `&mut ArchivedResult<T, E>` to `Result<&mut <T as
- `fn as_deref(self: &Self) -> Result<&<T as Deref>::Target, &E>` - Converts from `&ArchivedResult<T, E>` to `Result<&<T as Deref>::Target,

**Traits:** Eq, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Result<T, E>) -> bool`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<Result<T, U>, <D as >::Error>`



## rkyv::result::Iter

*Type Alias*: `crate::option::Iter<P>`

An iterator over a reference to the `Ok` variant of an [`ArchivedResult`].

The iterator yields one value if the result is `Ok`, otherwise none.

Created by [`ArchivedResult::iter`].



