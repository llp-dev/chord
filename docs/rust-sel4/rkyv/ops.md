**rkyv > ops**

# Module: ops

## Contents

**Structs**

- [`ArchivedRange`](#archivedrange) - An archived [`Range`](::core::ops::Range).
- [`ArchivedRangeFrom`](#archivedrangefrom) - An archived [`RangeFrom`](::core::ops::RangeFrom).
- [`ArchivedRangeFull`](#archivedrangefull) - An archived [`RangeFull`](::core::ops::RangeFull).
- [`ArchivedRangeInclusive`](#archivedrangeinclusive) - An archived [`RangeInclusive`](::core::ops::RangeInclusive).
- [`ArchivedRangeTo`](#archivedrangeto) - An archived [`RangeTo`](::core::ops::RangeTo).
- [`ArchivedRangeToInclusive`](#archivedrangetoinclusive) - An archived [`RangeToInclusive`](::core::ops::RangeToInclusive).

**Enums**

- [`ArchivedBound`](#archivedbound) - An archived [`Bound`].

---

## rkyv::ops::ArchivedBound

*Enum*

An archived [`Bound`].

**Generic Parameters:**
- T

**Variants:**
- `Included(T)` - An inclusive bound.
- `Excluded(T)` - An exclusive bound.
- `Unbounded` - An infinite endpoint. Indicates that there is no bound in this

**Methods:**

- `fn as_ref(self: &Self) -> Bound<&T>` - Converts from `&ArchivedBound<T>` to `Bound<&T>`.
- `fn as_mut(self: & mut Self) -> Bound<& mut T>` - Converts from `&mut ArchivedBound<T>` to `Bound<&mut T>`.
- `fn as_seal(this: Seal<Self>) -> Bound<Seal<T>>` - Converts from `Seal<&ArchivedBound<T>>` to `Bound<Seal<&T>>`.

**Traits:** Copy, Eq, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Bound<T>) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<Bound<T>, <D as Fallible>::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedBound<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedBound<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`



## rkyv::ops::ArchivedRange

*Struct*

An archived [`Range`](::core::ops::Range).

**Generic Parameters:**
- T

**Fields:**
- `start: T` - The lower bound of the range (inclusive).
- `end: T` - The upper bound of the range (inclusive).

**Methods:**

- `fn contains<U>(self: &Self, item: &U) -> bool` - Returns `true` if `item` is contained in the range.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the range contains no items.

**Traits:** Eq, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Range<T>) -> bool`
- **Default**
  - `fn default() -> ArchivedRange<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedRange<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<Range<T>, <D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedRange<T>`
- **RangeBounds**
  - `fn start_bound(self: &Self) -> Bound<&T>`
  - `fn end_bound(self: &Self) -> Bound<&T>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`



## rkyv::ops::ArchivedRangeFrom

*Struct*

An archived [`RangeFrom`](::core::ops::RangeFrom).

**Generic Parameters:**
- T

**Fields:**
- `start: T` - The lower bound of the range (inclusive).

**Methods:**

- `fn contains<U>(self: &Self, item: &U) -> bool` - Returns `true` if `item` is contained in the range.

**Traits:** Eq, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RangeFrom<T>) -> bool`
- **Default**
  - `fn default() -> ArchivedRangeFrom<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedRangeFrom<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **RangeBounds**
  - `fn start_bound(self: &Self) -> Bound<&T>`
  - `fn end_bound(self: &Self) -> Bound<&T>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<RangeFrom<T>, <D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedRangeFrom<T>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`



## rkyv::ops::ArchivedRangeFull

*Struct*

An archived [`RangeFull`](::core::ops::RangeFull).

**Unit Struct**

**Traits:** Copy, Eq, Portable

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ArchivedRangeFull`
- **Default**
  - `fn default() -> ArchivedRangeFull`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedRangeFull) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, _: &RangeFull) -> bool`
- **Deserialize**
  - `fn deserialize(self: &Self, _: & mut D) -> Result<RangeFull, <D as >::Error>`



## rkyv::ops::ArchivedRangeInclusive

*Struct*

An archived [`RangeInclusive`](::core::ops::RangeInclusive).

**Generic Parameters:**
- T

**Fields:**
- `start: T` - The lower bound of the range (inclusive).
- `end: T` - The upper bound of the range (inclusive).

**Methods:**

- `fn contains<U>(self: &Self, item: &U) -> bool` - Returns `true` if `item` is contained in the range.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the range contains no items.

**Traits:** Portable, Eq

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &RangeInclusive<T>) -> bool`
- **Default**
  - `fn default() -> ArchivedRangeInclusive<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedRangeInclusive<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **RangeBounds**
  - `fn start_bound(self: &Self) -> Bound<&T>`
  - `fn end_bound(self: &Self) -> Bound<&T>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<RangeInclusive<T>, <D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedRangeInclusive<T>`



## rkyv::ops::ArchivedRangeTo

*Struct*

An archived [`RangeTo`](::core::ops::RangeTo).

**Generic Parameters:**
- T

**Fields:**
- `end: T` - The upper bound of the range (exclusive).

**Methods:**

- `fn contains<U>(self: &Self, item: &U) -> bool` - Returns `true` if `item` is contained in the range.

**Traits:** Eq, Portable

**Trait Implementations:**

- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &RangeTo<T>) -> bool`
- **Default**
  - `fn default() -> ArchivedRangeTo<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedRangeTo<T>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **RangeBounds**
  - `fn start_bound(self: &Self) -> Bound<&T>`
  - `fn end_bound(self: &Self) -> Bound<&T>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<RangeTo<T>, <D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedRangeTo<T>`



## rkyv::ops::ArchivedRangeToInclusive

*Struct*

An archived [`RangeToInclusive`](::core::ops::RangeToInclusive).

**Generic Parameters:**
- T

**Fields:**
- `end: T` - The upper bound of the range (inclusive).

**Methods:**

- `fn contains<U>(self: &Self, item: &U) -> bool` - Returns `true` if `item` is contained in the range.

**Traits:** Eq, Portable

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **RangeBounds**
  - `fn start_bound(self: &Self) -> Bound<&T>`
  - `fn end_bound(self: &Self) -> Bound<&T>`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<RangeToInclusive<T>, <D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedRangeToInclusive<T>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &RangeToInclusive<T>) -> bool`
- **Default**
  - `fn default() -> ArchivedRangeToInclusive<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedRangeToInclusive<T>) -> bool`



