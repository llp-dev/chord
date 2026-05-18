**rkyv > niche > option_nonzero**

# Module: niche::option_nonzero

## Contents

**Structs**

- [`ArchivedOptionNonZeroI128`](#archivedoptionnonzeroi128) - A niched archived `Option<NonZeroI128>`
- [`ArchivedOptionNonZeroI16`](#archivedoptionnonzeroi16) - A niched archived `Option<NonZeroI16>`
- [`ArchivedOptionNonZeroI32`](#archivedoptionnonzeroi32) - A niched archived `Option<NonZeroI32>`
- [`ArchivedOptionNonZeroI64`](#archivedoptionnonzeroi64) - A niched archived `Option<NonZeroI64>`
- [`ArchivedOptionNonZeroI8`](#archivedoptionnonzeroi8) - A niched archived `Option<NonZeroI8>`
- [`ArchivedOptionNonZeroU128`](#archivedoptionnonzerou128) - A niched archived `Option<NonZeroU128>`
- [`ArchivedOptionNonZeroU16`](#archivedoptionnonzerou16) - A niched archived `Option<NonZeroU16>`
- [`ArchivedOptionNonZeroU32`](#archivedoptionnonzerou32) - A niched archived `Option<NonZeroU32>`
- [`ArchivedOptionNonZeroU64`](#archivedoptionnonzerou64) - A niched archived `Option<NonZeroU64>`
- [`ArchivedOptionNonZeroU8`](#archivedoptionnonzerou8) - A niched archived `Option<NonZeroU8>`

**Type Aliases**

- [`ArchivedOptionNonZeroIsize`](#archivedoptionnonzeroisize) - A niched archived `Option<NonZeroIsize>`
- [`ArchivedOptionNonZeroUsize`](#archivedoptionnonzerousize) - A niched archived `Option<NonZeroUsize>`
- [`Iter`](#iter) - An iterator over a reference to the `Some` variant of an

---

## rkyv::niche::option_nonzero::ArchivedOptionNonZeroI128

*Struct*

A niched archived `Option<NonZeroI128>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroI128>>` - Converts to an `Option<&Archived<NonZeroI128>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroI128>>` - Converts to an `Option<&mut Archived<NonZeroI128>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroI128>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroI128>` to `Option<Seal<'_, Archived<NonZeroI128>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroI128>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroI128>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroI128>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroI128>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroI128) -> & mut Archived<NonZeroI128>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroI128>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroI128>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Copy, Portable, Eq, NoUndef

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroI128`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroI16

*Struct*

A niched archived `Option<NonZeroI16>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroI16>>` - Converts to an `Option<&Archived<NonZeroI16>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroI16>>` - Converts to an `Option<&mut Archived<NonZeroI16>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroI16>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroI16>` to `Option<Seal<'_, Archived<NonZeroI16>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroI16>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroI16>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroI16>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroI16>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroI16) -> & mut Archived<NonZeroI16>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroI16>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroI16>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Copy, Portable, Eq, NoUndef

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroI16`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroI32

*Struct*

A niched archived `Option<NonZeroI32>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroI32>>` - Converts to an `Option<&Archived<NonZeroI32>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroI32>>` - Converts to an `Option<&mut Archived<NonZeroI32>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroI32>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroI32>` to `Option<Seal<'_, Archived<NonZeroI32>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroI32>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroI32>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroI32>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroI32>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroI32) -> & mut Archived<NonZeroI32>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroI32>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroI32>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Copy, Portable, Eq, NoUndef

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroI32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroI64

*Struct*

A niched archived `Option<NonZeroI64>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroI64>>` - Converts to an `Option<&Archived<NonZeroI64>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroI64>>` - Converts to an `Option<&mut Archived<NonZeroI64>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroI64>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroI64>` to `Option<Seal<'_, Archived<NonZeroI64>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroI64>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroI64>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroI64>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroI64>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroI64) -> & mut Archived<NonZeroI64>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroI64>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroI64>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Copy, Portable, Eq, NoUndef

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroI64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroI8

*Struct*

A niched archived `Option<NonZeroI8>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroI8>>` - Converts to an `Option<&Archived<NonZeroI8>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroI8>>` - Converts to an `Option<&mut Archived<NonZeroI8>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroI8>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroI8>` to `Option<Seal<'_, Archived<NonZeroI8>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroI8>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroI8>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroI8>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroI8>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroI8) -> & mut Archived<NonZeroI8>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroI8>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroI8>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Copy, Portable, Eq, NoUndef

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroI8`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroIsize

*Type Alias*: `ArchivedOptionNonZeroI32`

A niched archived `Option<NonZeroIsize>`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroU128

*Struct*

A niched archived `Option<NonZeroU128>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroU128>>` - Converts to an `Option<&Archived<NonZeroU128>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroU128>>` - Converts to an `Option<&mut Archived<NonZeroU128>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroU128>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroU128>` to `Option<Seal<'_, Archived<NonZeroU128>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroU128>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroU128>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroU128>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroU128>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroU128) -> & mut Archived<NonZeroU128>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroU128>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroU128>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** NoUndef, Copy, Portable, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroU128`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroU16

*Struct*

A niched archived `Option<NonZeroU16>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroU16>>` - Converts to an `Option<&Archived<NonZeroU16>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroU16>>` - Converts to an `Option<&mut Archived<NonZeroU16>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroU16>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroU16>` to `Option<Seal<'_, Archived<NonZeroU16>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroU16>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroU16>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroU16>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroU16>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroU16) -> & mut Archived<NonZeroU16>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroU16>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroU16>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Eq, NoUndef, Copy, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroU16`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroU32

*Struct*

A niched archived `Option<NonZeroU32>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroU32>>` - Converts to an `Option<&Archived<NonZeroU32>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroU32>>` - Converts to an `Option<&mut Archived<NonZeroU32>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroU32>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroU32>` to `Option<Seal<'_, Archived<NonZeroU32>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroU32>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroU32>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroU32>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroU32>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroU32) -> & mut Archived<NonZeroU32>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroU32>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroU32>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Copy, Portable, Eq, NoUndef

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroU32`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroU64

*Struct*

A niched archived `Option<NonZeroU64>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroU64>>` - Converts to an `Option<&Archived<NonZeroU64>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroU64>>` - Converts to an `Option<&mut Archived<NonZeroU64>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroU64>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroU64>` to `Option<Seal<'_, Archived<NonZeroU64>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroU64>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroU64>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroU64>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroU64>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroU64) -> & mut Archived<NonZeroU64>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroU64>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroU64>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Copy, Portable, Eq, NoUndef

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroU64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroU8

*Struct*

A niched archived `Option<NonZeroU8>`

**Methods:**

- `fn is_none(self: &Self) -> bool` - Returns `true` if the option is a `None` value.
- `fn is_some(self: &Self) -> bool` - Returns `true` if the option is a `Some` value.
- `fn as_ref(self: &Self) -> Option<&Archived<NonZeroU8>>` - Converts to an `Option<&Archived<NonZeroU8>>`
- `fn as_mut(self: & mut Self) -> Option<& mut Archived<NonZeroU8>>` - Converts to an `Option<&mut Archived<NonZeroU8>>`
- `fn as_seal(this: Seal<Self>) -> Option<Seal<Archived<NonZeroU8>>>` - Converts from `Seal<'_, ArchivedOptionNonZeroU8>` to `Option<Seal<'_, Archived<NonZeroU8>>>`.
- `fn take(self: & mut Self) -> Option<Archived<NonZeroU8>>` - Takes the value out of the option, leaving a `None` in its
- `fn iter(self: &Self) -> Iter<&Archived<NonZeroU8>>` - Returns an iterator over the possibly-contained value.
- `fn iter_mut(self: & mut Self) -> Iter<& mut Archived<NonZeroU8>>` - Returns an iterator over the mutable possibly-contained value.
- `fn iter_seal(this: Seal<Self>) -> Iter<Seal<Archived<NonZeroU8>>>` - Returns an iterator over the sealed mutable possibly-contained
- `fn get_or_insert(self: & mut Self, v: NonZeroU8) -> & mut Archived<NonZeroU8>` - Inserts `v` into the option if it is `None`, then returns a
- `fn get_or_insert_with<F>(self: & mut Self, f: F) -> & mut Archived<NonZeroU8>` - Inserts a value computed from `f` into the option if it is
- `fn resolve_from_option(field: Option<NonZeroU8>, out: Place<Self>)` - Resolves an `ArchivedOptionNonZero` from an `Option<NonZero>`.

**Traits:** Copy, Portable, Eq, NoUndef

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedOptionNonZeroU8`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`



## rkyv::niche::option_nonzero::ArchivedOptionNonZeroUsize

*Type Alias*: `ArchivedOptionNonZeroU32`

A niched archived `Option<NonZeroUsize>`



## rkyv::niche::option_nonzero::Iter

*Type Alias*: `crate::option::Iter<P>`

An iterator over a reference to the `Some` variant of an
`ArchivedOptionNonZero` integer.

This iterator yields one value if the `ArchivedOptionNonZero` integer is a
`Some`, otherwise none.



