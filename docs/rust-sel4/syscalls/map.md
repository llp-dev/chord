**syscalls > map**

# Module: map

## Contents

**Structs**

- [`SysnoMap`](#sysnomap) - A map of syscalls to a type `T`.
- [`SysnoMapIter`](#sysnomapiter) - An iterator over the syscall (number, value) pairs contained in a
- [`SysnoMapValues`](#sysnomapvalues) - An iterator over the syscall values contained in a [`SysnoMap`].

**Functions**

- [`get_idx`](#get_idx) - Get internal data index based on sysno value

**Type Aliases**

- [`DataArray`](#dataarray)

---

## syscalls::map::DataArray

*Type Alias*: `[core::mem::MaybeUninit<T>; 470]`



## syscalls::map::SysnoMap

*Struct*

A map of syscalls to a type `T`.

This provides constant-time lookup of syscalls within a static array.

# Examples

```
# use syscalls::{Sysno, SysnoMap};
struct Point { x: i32, y: i32 }

let mut map = SysnoMap::new();
map.insert(Sysno::openat, Point { x: 1, y: 2 });
assert!(map.get(Sysno::openat).is_some());
```

Use function callbacks:
```
# use syscalls::{Sysno, SysnoMap};
let mut map = SysnoMap::<fn() -> i32>::new();
map.insert(Sysno::openat, || 1);
map.insert(Sysno::close, || -1);
assert_eq!(map.get(Sysno::openat).unwrap()(), 1);
assert_eq!(map.get(Sysno::close).unwrap()(), -1);
```

```
# use syscalls::{Sysno, SysnoMap};
let mut syscalls = SysnoMap::from_iter([
    (Sysno::openat, 0),
    (Sysno::close, 42),
]);

assert!(!syscalls.is_empty());
assert_eq!(syscalls.remove(Sysno::openat), Some(0));
assert_eq!(syscalls.insert(Sysno::close, 4), Some(42));
assert!(syscalls.contains_key(Sysno::close));
assert_eq!(syscalls.get(Sysno::close), Some(&4));
assert_eq!(syscalls.insert(Sysno::close, 11), Some(4));
assert_eq!(syscalls.count(), 1);
assert_eq!(syscalls.remove(Sysno::close), Some(11));
assert!(syscalls.is_empty());
```

**Generic Parameters:**
- T

**Fields:**
- `is_set: crate::SysnoSet`
- `data: [core::mem::MaybeUninit<T>; 470]`

**Methods:**

- `fn from_slice(slice: &[(Sysno, T)]) -> Self` - Initialize a syscall map from the given slice. Note that `T` must be
- `fn init_all(default: &T) -> Self` - Initializes all possible syscalls in the map with the given default
- `fn new() -> Self` - Initializes an empty syscall map.
- `fn contains_key(self: &Self, sysno: Sysno) -> bool` - Returns true if the map contains the given syscall.
- `fn clear(self: & mut Self)` - Clears the map, removing all syscalls.
- `fn is_empty(self: &Self) -> bool` - Returns true if the map is empty. Athough this is an O(1) operation
- `fn count(self: &Self) -> usize` - Returns the number of syscalls in the map. Although This is an O(1)
- `fn insert(self: & mut Self, sysno: Sysno, value: T) -> Option<T>` - Inserts the given syscall into the map. Returns true if the syscall was
- `fn remove(self: & mut Self, sysno: Sysno) -> Option<T>` - Removes the given syscall from the map. Returns old value if the syscall
- `fn get(self: &Self, sysno: Sysno) -> Option<&T>` - Returns a reference to the value corresponding to `sysno`. Returns
- `fn get_mut(self: & mut Self, sysno: Sysno) -> Option<& mut T>` - Returns a mutable reference to the value corresponding to `sysno`.
- `fn iter(self: &Self) -> SysnoMapIter<T>` - Returns an iterator that iterates over the syscalls contained in the map.
- `fn values(self: &Self) -> SysnoMapValues<T>` - Returns an iterator that iterates over all enabled values contained in

**Trait Implementations:**

- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **IndexMut**
  - `fn index_mut(self: & mut Self, sysno: Sysno) -> & mut T`
- **Index**
  - `fn index(self: &Self, sysno: Sysno) -> &T`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **Default**
  - `fn default() -> Self`
- **Drop**
  - `fn drop(self: & mut Self)`



## syscalls::map::SysnoMapIter

*Struct*

An iterator over the syscall (number, value) pairs contained in a
[`SysnoMap`].

**Generic Parameters:**
- 'a
- T

**Fields:**
- `iter: crate::set::SysnoSetIter<'a>`
- `data: &'a [core::mem::MaybeUninit<T>; 470]`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syscalls::map::SysnoMapValues

*Struct*

An iterator over the syscall values contained in a [`SysnoMap`].

**Generic Parameters:**
- 'a
- T

**Tuple Struct**: `(crate::set::SysnoSetIter<'a>, &'a [core::mem::MaybeUninit<T>; 470])`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syscalls::map::get_idx

*Function*

Get internal data index based on sysno value

```rust
fn get_idx(sysno: super::Sysno) -> usize
```



