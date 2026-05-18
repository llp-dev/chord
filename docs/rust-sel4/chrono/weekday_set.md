**chrono > weekday_set**

# Module: weekday_set

## Contents

**Structs**

- [`WeekdaySet`](#weekdayset) - A collection of [`Weekday`]s stored as a single byte.
- [`WeekdaySetIter`](#weekdaysetiter) - An iterator over a collection of weekdays, starting from a given day.

---

## chrono::weekday_set::WeekdaySet

*Struct*

A collection of [`Weekday`]s stored as a single byte.

This type is `Copy` and provides efficient set-like and slice-like operations.
Many operations are `const` as well.

Implemented as a bitmask where bits 1-7 correspond to Monday-Sunday.

**Tuple Struct**: `()`

**Methods:**

- `fn from_array<const C>(days: [Weekday; C]) -> Self` - Create a `WeekdaySet` from an array of [`Weekday`]s.
- `fn single(weekday: Weekday) -> Self` - Create a `WeekdaySet` from a single [`Weekday`].
- `fn single_day(self: Self) -> Option<Weekday>` - Returns `Some(day)` if this collection contains exactly one day.
- `fn insert(self: & mut Self, day: Weekday) -> bool` - Adds a day to the collection.
- `fn remove(self: & mut Self, day: Weekday) -> bool` - Removes a day from the collection.
- `fn is_subset(self: Self, other: Self) -> bool` - Returns `true` if `other` contains all days in `self`.
- `fn intersection(self: Self, other: Self) -> Self` - Returns days that are in both `self` and `other`.
- `fn union(self: Self, other: Self) -> Self` - Returns days that are in either `self` or `other`.
- `fn symmetric_difference(self: Self, other: Self) -> Self` - Returns days that are in `self` or `other` but not in both.
- `fn difference(self: Self, other: Self) -> Self` - Returns days that are in `self` but not in `other`.
- `fn first(self: Self) -> Option<Weekday>` - Get the first day in the collection, starting from Monday.
- `fn last(self: Self) -> Option<Weekday>` - Get the last day in the collection, starting from Sunday.
- `fn iter(self: Self, start: Weekday) -> WeekdaySetIter` - Iterate over the [`Weekday`]s in the collection starting from a given day.
- `fn contains(self: Self, day: Weekday) -> bool` - Returns `true` if the collection contains the given day.
- `fn is_empty(self: Self) -> bool` - Returns `true` if the collection is empty.
- `fn len(self: Self) -> u8` - Returns the number of days in the collection.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &WeekdaySet) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &WeekdaySet) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> WeekdaySet`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &WeekdaySet) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Default**
  - `fn default() -> WeekdaySet`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## chrono::weekday_set::WeekdaySetIter

*Struct*

An iterator over a collection of weekdays, starting from a given day.

See [`WeekdaySet::iter()`].



