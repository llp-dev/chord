*[chrono](../index.md) / [weekday_set](index.md)*

---

# Module `weekday_set`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WeekdaySet`](#weekdayset) | struct | A collection of [`Weekday`]s stored as a single byte. |
| [`WeekdaySetIter`](#weekdaysetiter) | struct | An iterator over a collection of weekdays, starting from a given day. |

## Structs

### `WeekdaySet`

```rust
struct WeekdaySet(u8);
```

A collection of [`Weekday`](../weekday/index.md)s stored as a single byte.

This type is `Copy` and provides efficient set-like and slice-like operations.
Many operations are `const` as well.

Implemented as a bitmask where bits 1-7 correspond to Monday-Sunday.

#### Implementations

- <span id="weekdayset-from-array"></span>`const fn from_array<const C: usize>(days: [Weekday; C]) -> Self` — [`Weekday`](../weekday/index.md#weekday)

  Create a `WeekdaySet` from an array of [`Weekday`](../weekday/index.md)s.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::EMPTY, WeekdaySet::from_array([]));

  assert_eq!(WeekdaySet::single(Mon), WeekdaySet::from_array([Mon]));

  assert_eq!(WeekdaySet::ALL, WeekdaySet::from_array([Mon, Tue, Wed, Thu, Fri, Sat, Sun]));

  ```

- <span id="weekdayset-single"></span>`const fn single(weekday: Weekday) -> Self` — [`Weekday`](../weekday/index.md#weekday)

  Create a `WeekdaySet` from a single [`Weekday`](../weekday/index.md).

- <span id="weekdayset-single-day"></span>`const fn single_day(self) -> Option<Weekday>` — [`Weekday`](../weekday/index.md#weekday)

  Returns `Some(day)` if this collection contains exactly one day.

  

  Returns `None` otherwise.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).single_day(), Some(Mon));

  assert_eq!(WeekdaySet::from_array([Mon, Tue]).single_day(), None);

  assert_eq!(WeekdaySet::EMPTY.single_day(), None);

  assert_eq!(WeekdaySet::ALL.single_day(), None);

  ```

- <span id="weekdayset-insert"></span>`fn insert(&mut self, day: Weekday) -> bool` — [`Weekday`](../weekday/index.md#weekday)

  Adds a day to the collection.

  

  Returns `true` if the day was new to the collection.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  let mut weekdays = WeekdaySet::single(Mon);

  assert!(weekdays.insert(Tue));

  assert!(!weekdays.insert(Tue));

  ```

- <span id="weekdayset-remove"></span>`fn remove(&mut self, day: Weekday) -> bool` — [`Weekday`](../weekday/index.md#weekday)

  Removes a day from the collection.

  

  Returns `true` if the collection did contain the day.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  let mut weekdays = WeekdaySet::single(Mon);

  assert!(weekdays.remove(Mon));

  assert!(!weekdays.remove(Mon));

  ```

- <span id="weekdayset-is-subset"></span>`const fn is_subset(self, other: Self) -> bool`

  Returns `true` if `other` contains all days in `self`.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert!(WeekdaySet::single(Mon).is_subset(WeekdaySet::ALL));

  assert!(!WeekdaySet::single(Mon).is_subset(WeekdaySet::EMPTY));

  assert!(WeekdaySet::EMPTY.is_subset(WeekdaySet::single(Mon)));

  ```

- <span id="weekdayset-intersection"></span>`const fn intersection(self, other: Self) -> Self`

  Returns days that are in both `self` and `other`.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).intersection(WeekdaySet::single(Mon)), WeekdaySet::single(Mon));

  assert_eq!(WeekdaySet::single(Mon).intersection(WeekdaySet::single(Tue)), WeekdaySet::EMPTY);

  assert_eq!(WeekdaySet::ALL.intersection(WeekdaySet::single(Mon)), WeekdaySet::single(Mon));

  assert_eq!(WeekdaySet::ALL.intersection(WeekdaySet::EMPTY), WeekdaySet::EMPTY);

  ```

- <span id="weekdayset-union"></span>`const fn union(self, other: Self) -> Self`

  Returns days that are in either `self` or `other`.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).union(WeekdaySet::single(Mon)), WeekdaySet::single(Mon));

  assert_eq!(WeekdaySet::single(Mon).union(WeekdaySet::single(Tue)), WeekdaySet::from_array([Mon, Tue]));

  assert_eq!(WeekdaySet::ALL.union(WeekdaySet::single(Mon)), WeekdaySet::ALL);

  assert_eq!(WeekdaySet::ALL.union(WeekdaySet::EMPTY), WeekdaySet::ALL);

  ```

- <span id="weekdayset-symmetric-difference"></span>`const fn symmetric_difference(self, other: Self) -> Self`

  Returns days that are in `self` or `other` but not in both.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).symmetric_difference(WeekdaySet::single(Mon)), WeekdaySet::EMPTY);

  assert_eq!(WeekdaySet::single(Mon).symmetric_difference(WeekdaySet::single(Tue)), WeekdaySet::from_array([Mon, Tue]));

  assert_eq!(

      WeekdaySet::ALL.symmetric_difference(WeekdaySet::single(Mon)),

      WeekdaySet::from_array([Tue, Wed, Thu, Fri, Sat, Sun]),

  );

  assert_eq!(WeekdaySet::ALL.symmetric_difference(WeekdaySet::EMPTY), WeekdaySet::ALL);

  ```

- <span id="weekdayset-difference"></span>`const fn difference(self, other: Self) -> Self`

  Returns days that are in `self` but not in `other`.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).difference(WeekdaySet::single(Mon)), WeekdaySet::EMPTY);

  assert_eq!(WeekdaySet::single(Mon).difference(WeekdaySet::single(Tue)), WeekdaySet::single(Mon));

  assert_eq!(WeekdaySet::EMPTY.difference(WeekdaySet::single(Mon)), WeekdaySet::EMPTY);

  ```

- <span id="weekdayset-first"></span>`const fn first(self) -> Option<Weekday>` — [`Weekday`](../weekday/index.md#weekday)

  Get the first day in the collection, starting from Monday.

  

  Returns `None` if the collection is empty.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).first(), Some(Mon));

  assert_eq!(WeekdaySet::single(Tue).first(), Some(Tue));

  assert_eq!(WeekdaySet::ALL.first(), Some(Mon));

  assert_eq!(WeekdaySet::EMPTY.first(), None);

  ```

- <span id="weekdayset-last"></span>`fn last(self) -> Option<Weekday>` — [`Weekday`](../weekday/index.md#weekday)

  Get the last day in the collection, starting from Sunday.

  

  Returns `None` if the collection is empty.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).last(), Some(Mon));

  assert_eq!(WeekdaySet::single(Sun).last(), Some(Sun));

  assert_eq!(WeekdaySet::from_array([Mon, Tue]).last(), Some(Tue));

  assert_eq!(WeekdaySet::EMPTY.last(), None);

  ```

- <span id="weekdayset-split-at"></span>`const fn split_at(self, weekday: Weekday) -> (Self, Self)` — [`Weekday`](../weekday/index.md#weekday)

  Split the collection in two at the given day.

  

  Returns a tuple `(before, after)`. `before` contains all days starting from Monday

  up to but __not__ including `weekday`. `after` contains all days starting from `weekday`

  up to and including Sunday.

- <span id="weekdayset-iter"></span>`const fn iter(self, start: Weekday) -> WeekdaySetIter` — [`Weekday`](../weekday/index.md#weekday), [`WeekdaySetIter`](#weekdaysetiter)

  Iterate over the [`Weekday`](../weekday/index.md)s in the collection starting from a given day.

  

  Wraps around from Sunday to Monday if necessary.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  let weekdays = WeekdaySet::from_array([Mon, Wed, Fri]);

  let mut iter = weekdays.iter(Wed);

  assert_eq!(iter.next(), Some(Wed));

  assert_eq!(iter.next(), Some(Fri));

  assert_eq!(iter.next(), Some(Mon));

  assert_eq!(iter.next(), None);

  ```

- <span id="weekdayset-contains"></span>`const fn contains(self, day: Weekday) -> bool` — [`Weekday`](../weekday/index.md#weekday)

  Returns `true` if the collection contains the given day.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert!(WeekdaySet::single(Mon).contains(Mon));

  assert!(WeekdaySet::from_array([Mon, Tue]).contains(Tue));

  assert!(!WeekdaySet::single(Mon).contains(Tue));

  ```

- <span id="weekdayset-is-empty"></span>`const fn is_empty(self) -> bool`

  Returns `true` if the collection is empty.

  

  # Example

  ```rust

  use chrono::{Weekday, WeekdaySet};

  assert!(WeekdaySet::EMPTY.is_empty());

  assert!(!WeekdaySet::single(Weekday::Mon).is_empty());

  ```

- <span id="weekdayset-len"></span>`const fn len(self) -> u8`

  Returns the number of days in the collection.

  

  # Example

  ```rust

  use chrono::WeekdaySet;

  use chrono::Weekday::*;

  assert_eq!(WeekdaySet::single(Mon).len(), 1);

  assert_eq!(WeekdaySet::from_array([Mon, Wed, Fri]).len(), 3);

  assert_eq!(WeekdaySet::ALL.len(), 7);

  ```

- <span id="weekdayset-const-empty"></span>`const EMPTY: Self`

- <span id="weekdayset-const-all"></span>`const ALL: Self`

#### Trait Implementations

##### `impl Clone for WeekdaySet`

- <span id="weekdayset-clone"></span>`fn clone(&self) -> WeekdaySet` — [`WeekdaySet`](#weekdayset)

##### `impl Copy for WeekdaySet`

##### `impl Debug for WeekdaySet`

- <span id="weekdayset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WeekdaySet`

- <span id="weekdayset-default"></span>`fn default() -> WeekdaySet` — [`WeekdaySet`](#weekdayset)

##### `impl Display for WeekdaySet`

- <span id="weekdayset-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for WeekdaySet`

##### `impl FromIterator for WeekdaySet`

- <span id="weekdayset-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Weekday>>(iter: T) -> Self`

##### `impl Hash for WeekdaySet`

- <span id="weekdayset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for WeekdaySet`

- <span id="weekdayset-ord-cmp"></span>`fn cmp(&self, other: &WeekdaySet) -> cmp::Ordering` — [`WeekdaySet`](#weekdayset)

##### `impl PartialEq for WeekdaySet`

- <span id="weekdayset-partialeq-eq"></span>`fn eq(&self, other: &WeekdaySet) -> bool` — [`WeekdaySet`](#weekdayset)

##### `impl PartialOrd for WeekdaySet`

- <span id="weekdayset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &WeekdaySet) -> option::Option<cmp::Ordering>` — [`WeekdaySet`](#weekdayset)

##### `impl StructuralPartialEq for WeekdaySet`

##### `impl ToString for WeekdaySet`

- <span id="weekdayset-tostring-to-string"></span>`fn to_string(&self) -> String`

### `WeekdaySetIter`

```rust
struct WeekdaySetIter {
    days: WeekdaySet,
    start: crate::Weekday,
}
```

An iterator over a collection of weekdays, starting from a given day.

See `WeekdaySet::iter()`.

#### Trait Implementations

##### `impl Clone for WeekdaySetIter`

- <span id="weekdaysetiter-clone"></span>`fn clone(&self) -> WeekdaySetIter` — [`WeekdaySetIter`](#weekdaysetiter)

##### `impl Debug for WeekdaySetIter`

- <span id="weekdaysetiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for WeekdaySetIter`

- <span id="weekdaysetiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for WeekdaySetIter`

- <span id="weekdaysetiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl FusedIterator for WeekdaySetIter`

##### `impl IntoIterator for WeekdaySetIter`

- <span id="weekdaysetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="weekdaysetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="weekdaysetiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for WeekdaySetIter`

- <span id="weekdaysetiter-iterator-type-item"></span>`type Item = Weekday`

- <span id="weekdaysetiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

