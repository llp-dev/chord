**syscalls > set**

# Module: set

## Contents

**Structs**

- [`NonZeroUsizeIter`](#nonzerousizeiter) - Helper for iterating over the non-zero values of the words in the bitset.
- [`SysnoSet`](#sysnoset) - A set of syscalls.
- [`SysnoSetIter`](#sysnosetiter) - An iterator over the syscalls contained in a [`SysnoSet`].

**Functions**

- [`bits_per`](#bits_per)
- [`words`](#words) - Returns the number of words of type `T` required to hold the specified

---

## syscalls::set::NonZeroUsizeIter

*Struct*

Helper for iterating over the non-zero values of the words in the bitset.

**Generic Parameters:**
- 'a

**Fields:**
- `iter: core::slice::Iter<'a, usize>`
- `count: usize`

**Methods:**

- `fn new(iter: core::slice::Iter<'a, usize>) -> Self`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syscalls::set::SysnoSet

*Struct*

A set of syscalls.

This provides constant-time lookup of syscalls within a bitset.

# Examples

```
# use syscalls::{Sysno, SysnoSet};
let syscalls = SysnoSet::new(&[Sysno::read, Sysno::write, Sysno::openat, Sysno::close]);
assert!(syscalls.contains(Sysno::read));
assert!(syscalls.contains(Sysno::close));
```
Most operations can be done at compile-time as well.
```
# use syscalls::{Sysno, SysnoSet};
const SYSCALLS: SysnoSet =
    SysnoSet::new(&[Sysno::read, Sysno::write, Sysno::close])
        .union(&SysnoSet::new(&[Sysno::openat]));
const _: () = assert!(SYSCALLS.contains(Sysno::read));
const _: () = assert!(SYSCALLS.contains(Sysno::openat));
```

**Fields:**
- `data: [usize; 8]`

**Methods:**

- `fn get_idx_mask(sysno: Sysno) -> (usize, usize)` - Compute the index and mask for the given syscall as stored in the set data.
- `fn new(syscalls: &[Sysno]) -> Self` - Initialize the syscall set with the given slice of syscalls.
- `fn empty() -> Self` - Creates an empty set of syscalls.
- `fn all() -> Self` - Creates a set containing all valid syscalls.
- `fn contains(self: &Self, sysno: Sysno) -> bool` - Returns true if the set contains the given syscall.
- `fn is_empty(self: &Self) -> bool` - Returns true if the set is empty. Although this is an O(1) operation
- `fn clear(self: & mut Self)` - Clears the set, removing all syscalls.
- `fn count(self: &Self) -> usize` - Returns the number of syscalls in the set. Although this is an O(1)
- `fn insert(self: & mut Self, sysno: Sysno) -> bool` - Inserts the given syscall into the set. Returns true if the syscall was
- `fn remove(self: & mut Self, sysno: Sysno) -> bool` - Removes the given syscall from the set. Returns true if the syscall was
- `fn union(self: Self, other: &Self) -> Self` - Does a set union with this set and another.
- `fn intersection(self: Self, other: &Self) -> Self` - Does a set intersection with this set and another.
- `fn difference(self: Self, other: &Self) -> Self` - Calculates the difference with this set and another. That is, the
- `fn symmetric_difference(self: Self, other: &Self) -> Self` - Calculates the symmetric difference with this set and another. That is,
- `fn iter(self: &Self) -> SysnoSetIter` - Returns an iterator that iterates over the syscalls contained in the set.

**Traits:** Eq

**Trait Implementations:**

- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, rhs: Self)`
- **BitOr**
  - `fn bitor(self: Self, rhs: Self) -> <Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SysnoSet) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SysnoSet`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self`
- **Extend**
  - `fn extend<T>(self: & mut Self, iter: T)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, sysno: Sysno)`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, rhs: &Self)`
- **Default**
  - `fn default() -> Self`



## syscalls::set::SysnoSetIter

*Struct*

An iterator over the syscalls contained in a [`SysnoSet`].

**Generic Parameters:**
- 'a

**Fields:**
- `iter: NonZeroUsizeIter<'a>`
- `current: Option<core::num::NonZeroUsize>`

**Methods:**

- `fn new(iter: core::slice::Iter<'a, usize>) -> Self`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## syscalls::set::bits_per

*Function*

```rust
fn bits_per<T>() -> usize
```



## syscalls::set::words

*Function*

Returns the number of words of type `T` required to hold the specified
number of `bits`.

```rust
fn words<T>(bits: usize) -> usize
```



